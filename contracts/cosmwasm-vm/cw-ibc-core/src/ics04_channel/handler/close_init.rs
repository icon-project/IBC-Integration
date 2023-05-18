use super::*;

/// This function validates that a channel can be closed based on its current state and the existence of
/// an open IBC connection.
///
/// Arguments:
///
/// * `chan_end_on_a`: A reference to a `ChannelEnd` struct representing the channel end on chain A.
/// * `message`: The `message` parameter is of type `MsgChannelCloseInit` and represents the message
/// that initiates the closing of a channel.
///
/// Returns:
///
/// a `Result` type with the `Ok` variant containing an empty tuple `()` if the validation passes, and
/// the `Err` variant containing a `ContractError` with a specific error message if the validation
/// fails.
pub fn channel_close_init_validate(
    chan_end_on_a: &ChannelEnd,
    message: &MsgChannelCloseInit,
) -> Result<(), ContractError> {
    // Validate that the channel end is in a state where it can be closed.
    if chan_end_on_a.state_matches(&State::Closed) {
        return Err(ChannelError::InvalidChannelState {
            channel_id: message.chan_id_on_a.clone(),
            state: chan_end_on_a.state,
        })
        .map_err(|e| Into::<ContractError>::into(e));
    }

    // An OPEN IBC connection running on the local (host) chain should exist.
    if chan_end_on_a.connection_hops().len() != 1 {
        return Err(ChannelError::InvalidConnectionHopsLength {
            expected: 1,
            actual: chan_end_on_a.connection_hops().len(),
        })
        .map_err(|e| Into::<ContractError>::into(e));
    }

    Ok(())
}

/// The function creates an IBC channel close message with the given channel information for calling xcall.
///
/// Arguments:
///
/// * `msg`: `msg` is a reference to a `MsgChannelCloseInit` struct, which contains information about
/// the initial channel close message.
/// * `channel_end`: `channel_end` is a reference to a `ChannelEnd` struct, which represents the local
/// state of a channel. It contains information such as the channel's state, ordering, and version.
/// * `connection_id`: The ID of the connection associated with the channel being closed.
pub fn on_chan_close_init_submessage(
    msg: &MsgChannelCloseInit,
    channel_end: &ChannelEnd,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelCloseMsg {
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: msg.port_id_on_a.clone().to_string(),
        channel_id: msg.chan_id_on_a.clone().to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: channel_end.counterparty().port_id().to_string(),
        channel_id: channel_end.counterparty().channel_id().unwrap().to_string(),
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        cosmwasm_std::IbcOrder::Unordered,
        channel_end.version().to_string(),
        connection_id.connection_id().to_string(),
    );

    cosmwasm_std::IbcChannelCloseMsg::CloseInit {
        channel: ibc_channel,
    }
}
