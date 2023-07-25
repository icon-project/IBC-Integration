use super::*;

/// This function validates the channel open confirmation message and returns an error if the channel
/// state or connection hops are invalid.
///
/// Arguments:
///
/// * `message`: A reference to a `MsgChannelOpenConfirm` struct, which contains information about the
/// channel open confirmation message being validated.
/// * `chan_end_on_b`: `chan_end_on_b` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the counterparty chain.
///
/// Returns:
///
/// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
/// containing a `ContractError` type.
pub fn channel_open_confirm_validate(
    channel_id: &ChannelId,
    chan_end_on_b: &ChannelEnd,
) -> Result<(), ContractError> {
    ensure_channel_state(channel_id, chan_end_on_b, &State::TryOpen)?;

    validate_connection_length(chan_end_on_b)?;

    Ok(())
}

/// This function creates an IBC channel connect message for an open confirmation submessage for calling in xcall.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and connection hops.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The unique identifier of the channel within the given port.
///
/// Returns:
///
/// a `Result` with a `cosmwasm_std::IbcChannelConnectMsg` as the success type and a `ContractError` as
/// the error type. The success type is the result of creating an `IbcChannelConnectMsg` with the
/// `OpenConfirm` variant, which contains an `IbcChannel` struct with information about the channel
/// endpoint, counterparty,
pub fn on_chan_open_confirm_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Result<cosmwasm_std::IbcChannelConnectMsg, ContractError> {
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
        channel_end.connection_hops[0].clone().as_str(),
    );
    let data = cosmwasm_std::IbcChannelConnectMsg::OpenConfirm {
        channel: ibc_channel,
    };
    Ok(data)
}
