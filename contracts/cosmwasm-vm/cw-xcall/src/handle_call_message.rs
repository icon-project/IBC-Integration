use crate::ack::acknowledgement_data_on_success;

use super::*;

impl<'a> CwCallService<'a> {
    pub fn execute_call(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let proxy_requests = self
            .query_message_request(deps.storage, request_id)
            .unwrap();

        self.ensure_request_not_null(request_id, &proxy_requests)
            .unwrap();

        let message = XCallMessage {
            data: proxy_requests.data()?.to_vec(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_requests.to().to_string(),
            msg: to_binary(&message).unwrap(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn execute_rollback(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        sequence_no: u128,
    ) -> Result<Response, ContractError> {
        let call_request = self.query_request(deps.storage, sequence_no)?;

        self.ensure_call_request_not_null(sequence_no, &call_request)
            .unwrap();
        self.ensure_rollback_enabled(call_request.enabled())
            .unwrap();

        let message = XCallMessage {
            data: call_request.rollback().to_vec(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: call_request.to().to_string(),
            msg: to_binary(&message).unwrap(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_ROLLBACK_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn receive_packet_data(
        &self,
        deps: DepsMut,
        message: CwPacket,
    ) -> Result<CwReceiveResponse, ContractError> {
        let call_service_message: CallServiceMessage = from_binary(&message.data)?;

        match call_service_message.message_type() {
            CallServiceMessageType::CallServiceRequest => {
                self.hanadle_request(deps, call_service_message.payload(), &message)
            }
            CallServiceMessageType::CallServiceResponse => {
                self.handle_response(deps, call_service_message.payload(), &message)
            }
        }
    }

    pub fn hanadle_request(
        &self,
        deps: DepsMut,
        data: &[u8],
        packet: &CwPacket,
    ) -> Result<CwReceiveResponse, ContractError> {
        let request_id = self.increment_last_request_id(deps.storage)?;
        let message_request: CallServiceMessageRequest = data.try_into()?;

        let from = message_request.from();
        let to = message_request.to();

        let request = CallServiceMessageRequest::new(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            message_request.rollback(),
            message_request.data()?.into(),
        );

        self.insert_request(deps.storage, request_id, request)?;

        let event = event_call_message(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            request_id,
        );
        let acknowledgement_data =
            to_binary(&cw_common::client_response::XcallPacketResponseData {
                packet: packet.clone(),
                acknowledgement: make_ack_success().to_vec(),
            })
            .map_err(ContractError::Std)?;

        Ok(CwReceiveResponse::new()
            .add_attribute("action", "call_service")
            .add_attribute("method", "handle_response")
            .set_ack(acknowledgement_data)
            .add_event(event))
    }

    pub fn handle_response(
        &self,
        deps: DepsMut,
        data: &[u8],
        packet: &CwPacket,
    ) -> Result<CwReceiveResponse, ContractError> {
        let message: CallServiceMessageResponse = data.try_into()?;
        let response_sequence_no = message.sequence_no();

        let mut call_request = self.query_request(deps.storage, response_sequence_no)?;

        if call_request.is_null() {
            let acknowledgement_data =
                to_binary(&cw_common::client_response::XcallPacketResponseData {
                    packet: packet.clone(),
                    acknowledgement: make_ack_fail(format!(
                        "handle_resposne: no request for {}",
                        response_sequence_no
                    ))
                    .to_vec(),
                })
                .map_err(ContractError::Std)?;
            return Ok(CwReceiveResponse::new()
                .add_attribute("action", "call_service")
                .add_attribute("method", "handle_response")
                .set_ack(acknowledgement_data)
                .add_attribute(
                    "message",
                    format!("handle_resposne: no request for {}", response_sequence_no),
                ));
        }

        match message.response_code() {
            CallServiceResponseType::CallServiceResponseSuccess => {
                let event = match message.message().is_empty() {
                    true => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        "",
                    ),
                    false => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        message.message(),
                    ),
                };
                self.cleanup_request(deps.storage, response_sequence_no);
                Ok(CwReceiveResponse::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_ack(acknowledgement_data_on_success(packet)?)
                    .add_event(event))
            }
            _ => {
                self.ensure_rollback_length(call_request.rollback())
                    .unwrap();
                call_request.set_enabled();
                self.set_call_request(deps.storage, response_sequence_no, call_request)?;

                let event = event_rollback_message(response_sequence_no);

                Ok(CwReceiveResponse::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_ack(acknowledgement_data_on_success(packet)?)
                    .add_event(event))
            }
        }
    }

    pub fn cleanup_request(&self, store: &mut dyn Storage, sequence_no: u128) {
        self.remove_call_request(store, sequence_no);
    }

    pub fn save_config(
        &mut self,
        store: &mut dyn Storage,
        config: &IbcConfig,
    ) -> Result<(), ContractError> {
        match self.ibc_config().save(store, config) {
            Ok(_) => Ok(()),
            Err(err) => Err(ContractError::Std(err)),
        }
    }
}
