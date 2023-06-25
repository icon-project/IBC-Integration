use crate::types::LOG_PREFIX;
use cosmwasm_std::{
    coins, to_binary, Coin, CosmosMsg, Deps, QuerierWrapper, QueryRequest, SubMsg, WasmMsg,
};

use crate::{
    error::ContractError,
    state::{CwCallService, SEND_CALL_MESSAGE_REPLY_ID},
};

impl<'a> CwCallService<'a> {
    pub fn call_connection_send_message(
        &self,
        address: String,
        fee: Vec<Coin>,
        nid_to: String,
        sequence_no: u128,
        need_response: bool,
        msg: Vec<u8>,
    ) -> Result<SubMsg, ContractError> {
        let sn: i64 = if need_response { sequence_no } else { 0 }
            .try_into()
            .unwrap();
        let message = cw_common::xcall_connection_msg::ExecuteMsg::SendMessage { nid_to, sn, msg };

        let cosm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: address.clone(),
            msg: to_binary(&message).map_err(ContractError::Std)?,
            funds: fee,
        });
        let submessage = SubMsg {
            id: SEND_CALL_MESSAGE_REPLY_ID,
            msg: cosm_msg,
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };
        println!("{LOG_PREFIX} sent message to connection :{address}");
        Ok(submessage)
    }

    pub fn query_connection_fee(
        &self,
        deps: Deps,
        nid: &str,
        need_response: bool,
        address: &str,
    ) -> Result<u128, ContractError> {
        let query_message = cw_common::xcall_connection_msg::QueryMsg::GetFee {
            nid: nid.to_string(),
            response: need_response,
        };

        let query_request = QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
            contract_addr: address.to_string(),
            msg: to_binary(&query_message).map_err(ContractError::Std)?,
        });
        let fee: u128 = deps
            .querier
            .query(&query_request)
            .map_err(ContractError::Std)?;
        Ok(fee)
    }
}
