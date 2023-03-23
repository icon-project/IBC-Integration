use crate::ibc::{
    ibc_channel_close, ibc_channel_connect, ibc_channel_open, ibc_packet_ack, ibc_packet_receive,
};

use super::*;

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
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelOpen { msg } => {
                let response = ibc_channel_open(deps, env, msg).map_err(|error| return error)?;

                match response {
                    Some(data) => Ok(Response::new().add_attribute("version", data.version)),
                    None => Ok(Response::new()),
                }
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelConnect { msg } => {
                let response = ibc_channel_connect(deps, env, msg).map_err(|error| return error)?;

                Ok(Response::new()
                    .add_attributes(response.attributes)
                    .add_events(response.events)
                    .add_submessages(response.messages))
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelClose { msg } => {
                let response = ibc_channel_close(deps, env, msg).map_err(|error| return error)?;
                Ok(Response::new()
                    .add_attributes(response.attributes)
                    .add_events(response.events)
                    .add_submessages(response.messages))
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketReceive { msg } => {
                let response = ibc_packet_receive(deps, env, msg).map_err(|error| {
                    return ContractError::Std(StdError::NotFound {
                        kind: error.to_string(),
                    });
                })?;

                let response_data = Response::new()
                    .add_attributes(response.attributes)
                    .add_events(response.events)
                    .add_submessages(response.messages)
                    .set_data(response.acknowledgement);

                Ok(response_data)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketAck { msg } => {
                let response = ibc_packet_ack(deps, env, msg).map_err(|error| return error)?;
                Ok(Response::new()
                    .add_attributes(response.attributes)
                    .add_events(response.events)
                    .add_submessages(response.messages))
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
            ACK_FAILURE_ID => self.reply_ack_on_error(msg),
            _ => Err(ContractError::ReplyError {
                code: msg.id,
                msg: "Unkown".to_string(),
            }),
        }
    }

    // pub fn execute(
    //     &mut self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     msg: ExecuteMsg,
    // ) -> Result<Response, ContractError> {
    //     match msg {
    //         ExecuteMsg::SetAdmin { address } => self.add_admin(deps.storage, info, address),
    //         ExecuteMsg::SetProtocol { value } => self.set_protocol_fee(deps, info, value),
    //         ExecuteMsg::SetProtocolFeeHandler { address } => {
    //             self.set_protocol_feehandler(deps, env, info, address)
    //         }
    //         ExecuteMsg::SendCallMessage { to, data, rollback } => {
    //             self.send_packet(env, deps, info, to, data, rollback, 0)
    //         }
    //         ExecuteMsg::ExecuteCall { request_id } => self.execute_call(deps, info, request_id),
    //         ExecuteMsg::ExecuteRollback { sequence_no } => {
    //             self.execute_rollback(deps, info, sequence_no)
    //         }
    //     }
    // }
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

    fn reply_execute_rollback(&self, deps: Deps, msg: Reply) -> Result<Response, ContractError> {
        let sequence_no = self.last_sequence_no().load(deps.storage)?;

        let response = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => CallServiceMessageReponse::new(
                sequence_no,
                CallServiceResponseType::CallServiceResponseSucess,
                "",
            ),
            cosmwasm_std::SubMsgResult::Err(err) => {
                let error_message = format!("CallService Reverted : {err}");
                CallServiceMessageReponse::new(
                    sequence_no,
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                )
            }
        };

        let event = event_rollback_executed(
            sequence_no,
            to_int(response.response_code()),
            &to_string(response.message()).unwrap(),
        );

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_rollback")
            .add_event(event))
    }

    fn reply_execute_call_message(
        &self,
        deps: Deps,
        env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.last_request_id().load(deps.storage)?;
        let request = self.message_request().load(deps.storage, req_id)?;

        let responses = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                let code = 0;

                let message_response = CallServiceMessageReponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseSucess,
                    "",
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
                    &error_message,
                );
                let event = event_call_executed(req_id, code, &error_message);
                (message_response, event)
            }
        };

        if !request.rollback().is_empty() {
            let message: CallServiceMessage = responses.0.into();

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

    fn create_packet_response(&self, deps: Deps, env: Env, data: Binary) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));

        IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data,
            timeout,
        }
    }
    fn reply_ack_on_error(&self, reply: Reply) -> Result<Response, ContractError> {
        match reply.result {
            SubMsgResult::Ok(_) => Ok(Response::new()),
            SubMsgResult::Err(err) => Ok(Response::new().set_data(make_ack_fail(err))),
        }
    }
}
