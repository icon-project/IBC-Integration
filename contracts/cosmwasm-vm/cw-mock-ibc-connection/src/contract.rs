use super::*;
use crate::types::{config::Config, LOG_PREFIX};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall-ibc-connection";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwIbcConnection<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        self.init(deps.storage, info, msg)
    }

    fn init(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let owner = info.sender.as_str().to_string();

        self.add_admin(store, info, owner)?;
        self.set_ibc_host(store, msg.ibc_host.clone())?;
        self.set_xcall_host(store, msg.xcall_address)?;
        let config = Config {
            port_id: msg.port_id,
            denom: msg.denom,
        };
        self.store_config(store, &config)?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init")
            .add_attribute("ibc_host", msg.ibc_host))
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SendMessage { to, sn, msg } => {
                println!("{LOG_PREFIX} Received Payload From XCall App");
                // return Ok(Response::new());
                self.send_message(deps, info, env, to, sn, msg)
            }
            ExecuteMsg::SetFees {
                nid,
                packet_fee,
                ack_fee,
            } => {
                self.ensure_admin(deps.as_ref().storage, info.sender)?;
                self.set_fee(deps.storage, nid, packet_fee, ack_fee)
            }
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetFee { nid, response } => {
                let fees = self.get_network_fees(deps.storage, nid);
                if response {
                    return to_binary(&(fees.send_packet_fee + fees.ack_fee));
                }
                to_binary(&(fees.send_packet_fee))
            }
        }
    }

    pub fn migrate(
        &self,
        deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(Response::default().add_attribute("migrate", "successful"))
    }
}
