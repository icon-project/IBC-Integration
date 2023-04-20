use cosmwasm_std::to_vec;

use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-mock-dapp";
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
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    ) -> Result<Response, ContractError> {
        let sequence = self.increment_sequence(deps.storage)?;
        let address = self.xcall_address().load(deps.storage).map_err(|_e| ContractError::ModuleAddressNotFound)?;

        let roll_back = match rollback.clone() {
            Some(data) => {
                let roll_back = RollbackData {
                    id: sequence,
                    rollback: data.clone(),
                };
                self.roll_back().save(deps.storage, sequence, &data)?;
                to_vec(&roll_back).unwrap()
            }
            None => vec![],
        };

        let msg = cw_xcall::msg::ExecuteMsg::SendCallMessage {
            to,
            data,
            rollback: roll_back,
        };
        let message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: address,
            msg: to_binary(&msg).unwrap(),
            funds: info.funds,
        });

        Ok(Response::new()
            .add_attribute("Action", "SendMessage")
            .add_message(message))
    }

    pub fn handle_call_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        from: String,
        data: Vec<u8>,
    ) -> Result<Response, ContractError> {
        if info.sender.to_string() == from {
            let recieved_rollback =
                serde_json_wasm::from_slice::<RollbackData>(&data).map_err(|e| {
                    ContractError::DecodeError {
                        error: e.to_string(),
                    }
                })?;
            let seq = recieved_rollback.id;
            let rollback_store = self.roll_back().load(deps.storage, seq)?;
            if rollback_store != data {
                return Err(ContractError::RollBackMismatch { sequence: seq });
            }
            self.roll_back().remove(deps.storage, seq);

            Ok(Response::new()
                .add_attribute("action", "RollbackDataReceived")
                .add_attribute("from", from)
                .add_attribute("sequence", seq.to_string()))
        } else {
            let msg_data = serde_json_wasm::from_slice::<String>(&data).map_err(|e| {
                ContractError::DecodeError {
                    error: e.to_string(),
                }
            })?;
            if "revertMessage" == msg_data {
                return Err(ContractError::RevertFromDAPP);
            }
            Ok(Response::new()
                .add_attribute("from", from)
                .add_attribute("data", msg_data))
        }
    }
}
