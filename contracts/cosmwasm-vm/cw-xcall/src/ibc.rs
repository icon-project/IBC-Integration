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
    call_service.save_config(deps.storage, &ibc_config)?;

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
