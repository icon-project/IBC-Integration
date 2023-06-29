use cosmwasm_std::{to_binary, Addr, Binary, CosmosMsg, MessageInfo, StdError, SubMsg, WasmMsg};
use cw_common::xcall_types::network_address::NetworkAddress;

use crate::{error::ContractError, state::CwCallService};

impl<'a> CwCallService<'a> {
    pub fn call_dapp_handle_message(
        &self,
        info: MessageInfo,
        to: Addr,
        from: NetworkAddress,
        data: Vec<u8>,
        protocols: Vec<String>,
        reply_id: u64,
    ) -> Result<SubMsg, ContractError> {
        let cosm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: to.to_string(),
            msg: self
                .get_handle_message(from, data, protocols)
                .map_err(ContractError::Std)?,
            funds: info.funds,
        });
        let submessage = SubMsg {
            id: reply_id,
            msg: cosm_msg,
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };

        Ok(submessage)
    }

    pub fn get_handle_message(
        &self,
        from: NetworkAddress,
        data: Vec<u8>,
        protocols: Vec<String>,
    ) -> Result<Binary, StdError> {
        if protocols.is_empty() {
            let message = cw_common::dapp_msg::ExecuteMsg::HandleCallMessage { from: from, data };
            let msg = to_binary(&message);
            return msg;
        }
        let message = cw_common::dapp_multi_msg::ExecuteMsg::HandleCallMessage {
            from: from,
            data,
            protocols,
        };

        to_binary(&message)
    }
}
