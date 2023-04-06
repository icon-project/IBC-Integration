use super::*;

pub fn channel_close_init_validate(
    chan_end_on_a: &ChannelEnd,
    message: &MsgChannelCloseInit,
) -> Result<(), ContractError> {
    // Validate that the channel end is in a state where it can be closed.
    if chan_end_on_a.state_matches(&State::Closed) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidChannelState {
                channel_id: message.chan_id_on_a.clone(),
                state: chan_end_on_a.state,
            },
        });
    }

    // An OPEN IBC connection running on the local (host) chain should exist.
    if chan_end_on_a.connection_hops().len() != 1 {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: chan_end_on_a.connection_hops().len(),
            },
        });
    }

    Ok(())
}

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
