use std::str::from_utf8;

use cw_common::xcall_types::network_address::NetworkAddress;

use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-mock-dapp-multi";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwMockService<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        let sequence = u64::default();
        self.sequence().save(deps.storage, &sequence)?;
        self.xcall_address().save(deps.storage, &msg.address)?;

        Ok(Response::new())
    }

    pub fn send_call_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        to: NetworkAddress,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    ) -> Result<Response, ContractError> {
        let _sequence = self.increment_sequence(deps.storage)?;
        let address = self
            .xcall_address()
            .load(deps.storage)
            .map_err(|_e| ContractError::ModuleAddressNotFound)?;

        let network_id = to.nid().to_string();
        let connections = self.get_connections(deps.storage, network_id)?;
        let (sources, destinations) =
            connections
                .into_iter()
                .fold((Vec::new(), Vec::new()), |mut acc, x| {
                    acc.0.push(x.src_endpoint);
                    acc.1.push(x.dest_endpoint);
                    acc
                });
        let msg = cw_common::xcall_app_msg::ExecuteMsg::SendCallMessage {
            to,
            data,
            sources: Some(sources),
            destinations: Some(destinations),
            rollback,
        };
        let message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: address,
            msg: to_binary(&msg).unwrap(),
            funds: info.funds,
        });

        println!("{:?}", message);

        Ok(Response::new()
            .add_attribute("Action", "SendMessage")
            .add_message(message))
    }

    pub fn handle_call_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        from: NetworkAddress,
        data: Vec<u8>,
        _protocols: Vec<String>,
    ) -> Result<Response, ContractError> {
        if info.sender == from.account() {
            let recieved_rollback =
                serde_json_wasm::from_slice::<RollbackData>(&data).map_err(|e| {
                    ContractError::DecodeError {
                        error: e.to_string(),
                    }
                })?;
            let seq = recieved_rollback.id;
            let rollback_store = self
                .roll_back()
                .load(deps.storage, seq)
                .map_err(|_e| ContractError::MisiingRollBack { sequence: seq })?;
            if rollback_store != recieved_rollback.rollback {
                return Err(ContractError::RollBackMismatch { sequence: seq });
            }
            self.roll_back().remove(deps.storage, seq);

            Ok(Response::new()
                .add_attribute("action", "RollbackDataReceived")
                .add_attribute("from", from.to_string())
                .add_attribute("sequence", seq.to_string()))
        } else {
            let msg_data = from_utf8(&data).map_err(|e| ContractError::DecodeError {
                error: e.to_string(),
            })?;
            if "revertMessage" == msg_data {
                return Err(ContractError::RevertFromDAPP);
            }
            Ok(Response::new()
                .add_attribute("from", from.to_string())
                .add_attribute("data", msg_data))
        }
    }
}
