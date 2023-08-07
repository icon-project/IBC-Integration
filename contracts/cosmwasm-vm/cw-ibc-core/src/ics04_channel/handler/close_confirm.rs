use super::*;

/// The function validates a channel close confirmation message.
///
/// Arguments:
///
/// * `message`: A message of type `MsgChannelCloseConfirm` which contains information about the channel
/// close confirmation being validated.
/// * `chan_end_on_b`: `chan_end_on_b` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the counterparty chain.
///
/// Returns:
///
/// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
/// containing a `ContractError` type.
pub fn channel_close_confirm_validate(
    channel_id: &ChannelId,
    chan_end_on_b: &ChannelEnd,
) -> Result<(), ContractError> {
    ensure_channel_not_closed(channel_id, chan_end_on_b)?;
    validate_connection_length(chan_end_on_b)?;

    Ok(())
}

/// This function creates an IBC channel close confirmation sub message for calling xcall.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and connection hops.
/// * `port_id`: The identifier of the port associated with the channel being closed.
/// * `channel_id`: The unique identifier of the channel within the given port.
///
/// Returns:
///
/// a `Result` with an `IbcChannelCloseMsg` as the Ok variant and a `ContractError` as the Err variant.
pub fn on_chan_close_confirm_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Result<cosmwasm_std::IbcChannelCloseMsg, ContractError> {
    let counter_party_port_id = channel_end.counterparty().port_id.clone();
    let counter_party_channel = channel_end.counterparty().channel_id().unwrap().clone();
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: counter_party_port_id.to_string(),
        channel_id: counter_party_channel.to_string(),
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        channel_end.ordering.to_ibc_order().unwrap(),
        channel_end.version.to_string(),
        channel_end.connection_hops[0].clone().as_str(),
    );
    let data = cosmwasm_std::IbcChannelCloseMsg::CloseConfirm {
        channel: ibc_channel,
    };
    Ok(data)
}
