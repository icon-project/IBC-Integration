use crate::types::{message::CSMessage, LOG_PREFIX};
use common::rlp;
use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, DepsMut, QueryRequest, SubMsg, WasmMsg,
};
use cosmwasm_std::{MessageInfo, Response};
use cw_xcall_lib::network_address::NetId;
use cw_xcall_lib::xcall_connection_msg;

use crate::{
    error::ContractError,
    state::{CwCallService, SEND_CALL_MESSAGE_REPLY_ID},
};

impl<'a> CwCallService<'a> {
    pub fn call_connection_send_message(
        &self,
        address: &Addr,
        fee: Vec<Coin>,
        to: NetId,
        sn: i64,
        msg: &CSMessage,
    ) -> Result<SubMsg, ContractError> {
        let msg = rlp::encode(msg).to_vec();
        self.ensure_data_length(msg.len())?;
        let message = xcall_connection_msg::ExecuteMsg::SendMessage { to, sn, msg };

        let cosm_msg = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: address.to_string(),
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
        nid: NetId,
        need_response: bool,
        address: &str,
    ) -> Result<u128, ContractError> {
        let query_message = xcall_connection_msg::QueryMsg::GetFee {
            nid,
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

    pub fn set_default_connection(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        nid: NetId,
        address: Addr,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        deps.api.addr_validate(address.as_str())?;
        self.store_default_connection(deps.storage, nid, address)?;

        Ok(Response::new().add_attribute("method", "set_default_connection"))
    }
}
