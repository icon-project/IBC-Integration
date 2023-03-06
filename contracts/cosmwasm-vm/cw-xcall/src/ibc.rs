use crate::{
    ack::make_ack_fail,
    events::{event_call_message, event_response_message, event_rollback_message},
    state::{CwCallService, IbcConfig},
    types::{
        address::Address,
        message::{CallServiceMessage, CallServiceMessageType},
        request::CallServiceMessageRequest,
        response::{to_int, CallServiceMessageReponse, CallServiceResponseType},
    },
    ContractError,
};

use cosmwasm_std::{
    entry_point, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacket, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, Never,
};

pub const IBC_VERSION: &str = "xcall-1";

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;

    let source = msg.channel().endpoint.clone();
    let destination = msg.channel().counterparty_endpoint.clone();

    let ibc_config = IbcConfig::new(source, destination);
    let mut call_service = CwCallService::default();
    call_service.save_config(deps, &ibc_config)?;

    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_channel_connect"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
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

pub fn validate_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    // We expect an unordered channel here. Ordered channels have the
    // property that if a message is lost the entire channel will stop
    // working until you start it again.
    if channel.order != IbcOrder::Unordered {
        return Err(ContractError::OrderedChannel {});
    }

    if channel.version != IBC_VERSION {
        return Err(ContractError::InvalidVersion {
            actual: channel.version.to_string(),
            expected: IBC_VERSION.to_string(),
        });
    }

    // Make sure that we're talking with a counterparty who speaks the
    // same "protocol" as us.
    //
    // For a connection between chain A and chain B being established
    // by chain A, chain B knows counterparty information during
    // `OpenTry` and chain A knows counterparty information during
    // `OpenAck`. We verify it when we have it but when we don't it's
    // alright.
    if let Some(counterparty_version) = counterparty_version {
        if counterparty_version != IBC_VERSION {
            return Err(ContractError::InvalidVersion {
                actual: counterparty_version.to_string(),
                expected: IBC_VERSION.to_string(),
            });
        }
    }

    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    // Regardless of if our processing of this packet works we need to
    // commit an ACK to the chain. As such, we wrap all handling logic
    // in a seprate function and on error write out an error ack.
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
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let call_service = CwCallService::default();
    let _channel = msg.packet.dest.channel_id.clone();

    call_service.receive_packet_data(deps, msg.packet)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // Nothing to do here. We don't keep any state about the other
    // chain, just deliver messages so nothing to update.
    //
    // If we did care about how the other chain received our message
    // we could deserialize the data field into an `Ack` and inspect
    // it.
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_ack"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // As with ack above, nothing to do here. If we cared about
    // keeping track of state between the two chains then we'd want to
    // respond to this likely as it means that the packet in question
    // isn't going anywhere.
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
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
