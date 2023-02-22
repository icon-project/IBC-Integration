use crate::{error::ContractError, state::CwCallservice, types::{response::{CallServiceMessageReponse, CallServiceResponseType}, message::CallServiceMessageType}, msg};
use cosmwasm_std::{
    entry_point, CosmosMsg, DepsMut, Empty, Env, Event, MessageInfo, Reply, Response, SubMsg,
    WasmMsg, IbcMsg, Deps, Binary, IbcTimeoutBlock, IbcTimeout, to_binary,
};



const EXECUTE_CALL: u64 = 0;

impl<'a> CwCallservice<'a> {
    pub fn execute_call(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let proxy_reqs = self
            .message_request()
            .may_load(deps.storage, request_id)
            .unwrap();

        assert!(proxy_reqs.is_none(), "InvalidRequestId");
        self.message_request()
            .remove(deps.storage, request_id.try_into().unwrap());

        let network_address = proxy_reqs.clone().unwrap().from().to_string();

      let mut msgRes= CallServiceMessageReponse::default();

    msgRes.set_fields(proxy_reqs.clone().unwrap().sequence_no(), CallServiceResponseType::CallServiceResponseSucess, " ".into());

    if proxy_reqs.clone().unwrap().rollback() {
        sequence_no : u128 = proxy_reqs.clone().unwrap().sequence_no();
        
       

    }

        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_reqs.clone().unwrap().to().to_string(),
            msg: cosmwasm_std::Binary(proxy_reqs.unwrap().data().to_owned()),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL);

        Ok(Response::new()
            .add_attribute("call_message", "execute_call")
            .add_submessage(sub_msg))  
    }

    pub fn create_packet_response(
        &self,
        deps : Deps,
        env : Env,
        sequence_no: u128,
        time_out_height : u64,
        data : Binary,
    ) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();
        let timeout_block = IbcTimeoutBlock{
            revision:0,
            height:time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));
        
    let data = CallServiceMessageReponse::set_fields(&self, sequence_no, CallServiceResponseType::CallServiceResponseSucess, "msgRes".into());
    
    IbcMsg::SendPacket {
        channel_id: ibc_config.dst_endpoint().channel_id.clone(),
        data: to_binary(&data).unwrap(),
        timeout,
    }

    }
    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        EXECUTE_CALL => reply_message_sent(msg),
        id => Err(ContractError::Unauthorized {}),
    }
}

pub fn reply_message_sent(msg: Reply) -> Result<Response, ContractError> {
    match msg.result {
        cosmwasm_std::SubMsgResult::Ok(res) => Ok(Response::new()
            .add_attribute("call_message", "reply_message_sent")
            .add_events(res.events)),

        cosmwasm_std::SubMsgResult::Err(err) => {
            Err(ContractError::ReplyError { code: 1, msg: err })
        }
    }
}
pub fn call_executed(request_id: u128, code: u8, msg: String) -> Event {
    Event::new("callexecuted")
        .add_attribute("request_id", request_id.to_string())
        .add_attribute("code", code.to_string())
        .add_attribute("msg", msg.to_string())
}

