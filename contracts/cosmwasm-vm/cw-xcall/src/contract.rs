use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{CwCallService, EXECUTE_CALL_ID, EXECUTE_ROLLBACK_ID},
    types::address::Address,
};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallService<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        self.init(deps, info)
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetAdmin { address } => self.add_admin(deps.storage, info, address),
            ExecuteMsg::SetProtocol { value } => self.set_protocol_fee(deps, info, value),
            ExecuteMsg::SetProtocolFeeHandler { address } => {
                self.set_protocol_feehandler(deps, env, info, address)
            }
            ExecuteMsg::SendCallMessage { to, data, rollback } => {
                self.send_packet(env, deps, info, to, data, rollback, 0)
            }
            ExecuteMsg::ExecuteCall { request_id } => self.execute_call(deps, info, request_id),
            ExecuteMsg::ExecuteRollback { sequence_no } => {
                self.execute_rollback(deps, info, sequence_no)
            }
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetAdmin {} => to_binary(&self.query_admin(deps.storage).unwrap()),
            QueryMsg::GetProtocolFee {} => to_binary(&self.get_protocol_fee(deps)),
            QueryMsg::GetProtocolFeeHandler {} => to_binary(&self.get_protocol_feehandler(deps)),
        }
    }

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            EXECUTE_CALL_ID => self.reply_execute_call_message(deps.as_ref(), env, msg),
            EXECUTE_ROLLBACK_ID => self.reply_execute_rollback(deps.as_ref(), msg),
            _ => Err(ContractError::Unauthorized {}),
        }
    }
}

impl<'a> CwCallService<'a> {
    fn init(&self, deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let last_sequence_no = u128::default();
        let last_request_id = u128::default();
        let owner = Address::from(info.sender.as_str());

        self.add_owner(deps.storage, owner)?;
        self.init_last_sequnce_no(deps.storage, last_sequence_no)?;
        self.init_last_request_id(deps.storage, last_request_id)?;

        Ok(Response::new())
    }
}
