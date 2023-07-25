use super::*;

/// The function validates the channel open acknowledgement message and returns an error if the channel
/// state or connection hops are invalid.
///
/// Arguments:
///
/// * `message`: A reference to a `MsgChannelOpenAck` struct, which contains information about the
/// acknowledgement of a channel opening message.
/// * `chan_end_on_a`: `chan_end_on_a` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the "A" side (i.e. the side that initiated the channel opening handshake). This
/// object contains information such as the channel ID, the counterparty channel ID, the connection hops
///
/// Returns:
///
/// a `Result` type with either an `Ok(())` value indicating that the validation was successful, or an
/// `Err(ContractError)` value indicating that the validation failed with a specific `ContractError`
/// type.
pub fn channel_open_ack_validate(
    channel_id:&IbcChannelId,
    channel: &ChannelEnd,
) -> Result<(), ContractError> {
    ensure_channel_state(channel_id, channel, &State::Init)?;
    validate_connection_length(channel)?;

    Ok(())
}

/// This function creates an IBC channel connect message for an open acknowledgement submessage.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and version.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The ID of the channel being opened and acknowledged.
/// * `connection_id`: The ID of the connection associated with the channel being opened.
///
/// Returns:
///
/// a `Result` with `cosmwasm_std::IbcChannelConnectMsg` as the success type and `ContractError` as the
/// error type.
pub fn on_chan_open_ack_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> Result<cosmwasm_std::IbcChannelConnectMsg, ContractError> {
    let port_id = port_id.clone();
    let channel_id = channel_id;
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
    let ibc_order = match channel_end.ordering {
        Order::Unordered => cosmwasm_std::IbcOrder::Unordered,
        Order::Ordered => cosmwasm_std::IbcOrder::Ordered,
        Order::None => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::UnknownOrderType {
                    type_id: "None".to_string(),
                },
            });
        }
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        ibc_order,
        channel_end.version.to_string(),
        connection_id.to_string(),
    );
    let data = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: ibc_channel,
        counterparty_version: channel_end.version.to_string(),
    };
    Ok(data)
}
