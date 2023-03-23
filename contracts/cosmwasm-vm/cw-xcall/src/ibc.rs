use super::*;

pub const IBC_VERSION: &str = "xcall-1";
pub const APP_ORDER: IbcOrder = IbcOrder::Unordered;

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    let channel = msg.channel();

    check_order(&channel.order)?;

    if let Some(counter_version) = msg.counterparty_version() {
        check_version(counter_version)?;
    }

    Ok(Some(Ibc3ChannelOpenResponse {
        version: IBC_VERSION.to_string(),
    }))
}

#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel();

    check_order(&channel.order)?;

    if let Some(counter_version) = msg.counterparty_version() {
        check_version(counter_version)?;
    }

    let source = msg.channel().endpoint.clone();
    let destination = msg.channel().counterparty_endpoint.clone();

    let ibc_config = IbcConfig::new(source, destination);
    let mut call_service = CwCallService::default();
    call_service.save_config(deps, &ibc_config)?;

    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_channel_connect"))
}

#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel().endpoint.channel_id.clone();
    // Reset the state for the channel.

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("channel", channel))
}

#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    match do_ibc_packet_receive(deps, env, msg) {
        Ok(response) => Ok(response),
        Err(error) => Ok(IbcReceiveResponse::new()
            .add_attribute("method", "ibc_packet_receive")
            .add_attribute("error", error.to_string())
            .set_ack(make_ack_fail(error.to_string()))),
    }
}

fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let call_service = CwCallService::default();
    let _channel = msg.packet.dest.channel_id.clone();

    call_service.receive_packet_data(deps, msg.packet)
}

#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let ack_response: Ack = from_binary(&ack.acknowledgement.data)?;

    match ack_response {
        Ack::Result(_) => on_ack_sucess(ack.original_packet),
        Ack::Error(err) => on_ack_failure(ack.original_packet, &err),
    }
}

#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let submsg = SubMsg::reply_on_error(CosmosMsg::Custom(Empty {}), ACK_FAILURE_ID);
    Ok(IbcBasicResponse::new()
        .add_submessage(submsg)
        .add_attribute("method", "ibc_packet_timeout"))
}

impl<'a> CwCallService<'a> {
    fn receive_packet_data(
        &self,
        deps: DepsMut,
        message: IbcPacket,
    ) -> Result<IbcReceiveResponse, ContractError> {
        // TODO : ADD check for sender logic

        let call_service_message: CallServiceMessage = message.data.try_into()?;

        match call_service_message.message_type() {
            CallServiceMessageType::CallServiceRequest => {
                self.hanadle_request(deps, call_service_message.payload())
            }
            CallServiceMessageType::CallServiceResponse => {
                self.handle_response(deps, call_service_message.payload())
            }
        }
    }

    fn hanadle_request(
        &self,
        deps: DepsMut,
        data: &[u8],
    ) -> Result<IbcReceiveResponse, ContractError> {
        let request_id = self.increment_last_request_id(deps.storage)?;
        let message_request: CallServiceMessageRequest = data.try_into()?;

        let from = message_request.from();
        let to = message_request.to();

        let request = CallServiceMessageRequest::new(
            from.clone(),
            to.to_string(),
            message_request.sequence_no(),
            message_request.rollback().into(),
            message_request.data().into(),
        );

        self.insert_request(deps.storage, request_id, request)?;

        let event = event_call_message(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            request_id,
        );

        Ok(IbcReceiveResponse::new()
            .add_attribute("action", "call_service")
            .add_attribute("method", "handle_response")
            .set_ack(make_ack_success())
            .add_event(event))
    }

    fn handle_response(
        &self,
        deps: DepsMut,
        data: &[u8],
    ) -> Result<IbcReceiveResponse, ContractError> {
        let message: CallServiceMessageReponse = data.try_into()?;
        let response_sequence_no = message.sequence_no();

        let mut call_request = self.query_request(deps.storage, response_sequence_no)?;

        if call_request.is_null() {
            return Ok(IbcReceiveResponse::new()
                .add_attribute("action", "call_service")
                .add_attribute("method", "handle_response")
                .set_ack(make_ack_fail(format!(
                    "handle_resposne: no request for {}",
                    response_sequence_no
                )))
                .add_attribute(
                    "message",
                    format!("handle_resposne: no request for {}", response_sequence_no),
                ));
        }

        match message.response_code() {
            CallServiceResponseType::CallServiceResponseSucess => {
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
                self.cleanup_request(deps, response_sequence_no);
                Ok(IbcReceiveResponse::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_ack(make_ack_success())
                    .add_event(event))
            }
            _ => {
                self.ensure_rollback_length(call_request.rollback())
                    .unwrap();
                call_request.set_enabled();
                self.set_call_request(deps.storage, response_sequence_no, call_request)?;

                let event = event_rollback_message(response_sequence_no);

                Ok(IbcReceiveResponse::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_ack(make_ack_success())
                    .add_event(event))
            }
        }
    }

    fn cleanup_request(&self, deps: DepsMut, sequence_no: u128) {
        self.remove_call_request(deps.storage, sequence_no);
    }

    fn save_config(&mut self, deps: DepsMut, config: &IbcConfig) -> Result<(), ContractError> {
        match self.ibc_config().save(deps.storage, config) {
            Ok(_) => Ok(()),
            Err(err) => Err(ContractError::Std(err)),
        }
    }
}

fn on_ack_sucess(packet: IbcPacket) -> Result<IbcBasicResponse, ContractError> {
    let message: CallServiceMessage = from_binary(&packet.data)?;

    let message_type = match message.message_type() {
        CallServiceMessageType::CallServiceRequest => "call_service_request",
        CallServiceMessageType::CallServiceResponse => "call_service_response",
    };

    let attributes = vec![
        attr("action", "acknowledge"),
        attr("success", "true"),
        attr("message_type", message_type),
    ];

    Ok(IbcBasicResponse::new().add_attributes(attributes))
}

fn on_ack_failure(packet: IbcPacket, error: &str) -> Result<IbcBasicResponse, ContractError> {
    let message: CallServiceMessage = from_binary(&packet.data)?;
    let message_type = match message.message_type() {
        CallServiceMessageType::CallServiceRequest => "call_service_request",
        CallServiceMessageType::CallServiceResponse => "call_service_response",
    };

    Ok(IbcBasicResponse::new()
        .add_attribute("action", "acknowledge")
        .add_attribute("message_type", message_type)
        .add_attribute("success", "false")
        .add_attribute("error", error))
}
