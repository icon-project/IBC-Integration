
use crate::{
    error::ContractError,
    state::CwCallservice,
    types::{
        address::Address, request::CallServiceMessageRequest, response::CallServiceMessageReponse,
    }, handle_callmessage,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, CosmosMsg, DepsMut, Empty, MessageInfo, Response, SubMsg, WasmMsg,
};


#[cw_serde]
pub enum Contract {
    From,
    Data,
}

impl<'a> CwCallservice<'a> {
    pub fn execute_call(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let mut proxyReqs = self
            .message_request()
            .may_load(deps.storage, request_id)
            .unwrap();

        assert!(proxyReqs.is_none(), "InvalidRequestId");
        self.message_request()
            .remove(deps.storage, request_id.try_into().unwrap());

        let network_address = Address::from_str(&proxyReqs.unwrap().from().to_string());

    
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxyReqs.unwrap().to().to_string(),
            msg: cosmwasm_std::Binary(proxyReqs.unwrap().data().to_owned()),
            funds: info.funds,
        });

        let message = SubMsg::new(call_message);

        Ok(Response::new()
            .add_attribute("call_message", "execute_call")
            .add_submessage(message))

}



        }
    

   

    
    



