use crate::{
    error::ContractError,
    state::CwCallservice,
    types::response::{CallServiceMessageReponse, CallServiceResponseType},
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, Event, IbcMsg,
    IbcTimeout, MessageInfo, Reply, Response, SubMsg, WasmMsg,
};

const EXECUTE_CALL: u64 = 0;

impl<'a> CwCallservice<'a> {
    pub fn execute_call(
        &self,
        env: Env,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let proxy_reqs = self
            .query_message_request(deps.storage, request_id)
            .unwrap();

        self.contains_request(deps.storage, request_id);

        self.remove_request(deps.storage, request_id);

        let network_address = proxy_reqs.clone().from().to_string();

        let mut msgRes = CallServiceMessageReponse::default();

        msgRes.set_fields(
            proxy_reqs.clone().sequence_no(),
            CallServiceResponseType::CallServiceResponseSucess,
            " ".into(),
        );

        if !proxy_reqs.clone().rollback().is_empty() {
            let sequence_no: u128 = proxy_reqs.clone().sequence_no();
            self.create_packet_response(
                deps.as_ref(),
                env,
                sequence_no,
                to_binary(&msgRes).unwrap(),
            );
        }

        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_reqs.clone().to().to_string(),
            msg: cosmwasm_std::Binary(proxy_reqs.data().to_owned()),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL);

        Ok(Response::new()
            .add_attribute("call_message", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn create_packet_response(
        &self,
        deps: Deps,
        env: Env,
        sequence_no: u128,
        data: Binary,
    ) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));

        let mut msgRes = CallServiceMessageReponse::default();
        msgRes.set_fields(
            sequence_no,
            CallServiceResponseType::CallServiceResponseSucess,
            "msgRes".into(),
        );

        IbcMsg::SendPacket {
            channel_id: ibc_config.dsr_endpoint().channel_id.clone(),
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
