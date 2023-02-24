use crate::{
    error::ContractError,
    events::event_call_executed,
    state::{CwCallservice, EXECUTE_CALL},
    types::{
        message::{CallServiceMessage, CallServiceMessageType},
        response::{CallServiceMessageReponse, CallServiceResponseType},
    },
};
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, IbcMsg, IbcTimeout, MessageInfo,
    Reply, Response, SubMsg, WasmMsg,
};

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

        self.ensure_request_not_null(request_id, &proxy_reqs)
            .unwrap();

        let sequence_no = proxy_reqs.sequence_no();

        let msg_res = CallServiceMessageReponse::new(
            sequence_no,
            CallServiceResponseType::CallServiceResponseSucess,
            Binary::from_base64("").unwrap(),
        );

        let data = to_binary(&msg_res).unwrap();
        self.create_packet_response(deps.as_ref(), env, data);

        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_reqs.to().to_string(),
            msg: to_binary(proxy_reqs.data()).unwrap(), //TODO : Need to update
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn create_packet_response(&self, deps: Deps, env: Env, data: Binary) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));

        IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data,
            timeout,
        }
    }

    pub fn reply_message_sent(
        &self,
        deps: Deps,
        env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.last_request_id().load(deps.storage)?;
        let request = self.message_request().load(deps.storage, req_id)?;

        let responses = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(res) => {
                let code = 0;

                let message_response = CallServiceMessageReponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseSucess,
                    "".as_bytes().into(),
                );
                let event = event_call_executed(req_id, code, "");
                (message_response, event)
            }
            cosmwasm_std::SubMsgResult::Err(err) => {
                let code = -1;
                let error_message = format!("CallService Reverted : {err}");
                let message_response = CallServiceMessageReponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseFailure,
                    error_message.as_bytes().into(),
                );
                let event = event_call_executed(req_id, code, &error_message);
                (message_response, event)
            }
        };

        if !request.rollback().is_empty() {
            let message = CallServiceMessage::new(
                CallServiceMessageType::CallServiceResponse,
                to_binary(&responses.0).unwrap(),
            );

            let packet = self.create_packet_response(deps, env, to_binary(&message).unwrap());

            return Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "execute_callback")
                .add_message(packet));
        }
        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_callback")
            .add_event(responses.1))
    }
}
