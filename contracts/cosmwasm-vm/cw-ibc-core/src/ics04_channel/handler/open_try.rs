use super::*;

/// This function validates if a channel open try message is valid based on the connection end on B.
///
/// Arguments:
///
/// * `message`: A message of type `MsgChannelOpenTry` which contains information about the channel
/// being opened and its associated parameters.
/// * `conn_end_on_b`: `conn_end_on_b` is a reference to a `ConnectionEnd` object representing the
/// connection end on the counterparty chain.
///
/// Returns:
///
/// a `Result` type with either an `Ok(())` value indicating that the validation was successful, or an
/// `Err(ContractError)` value indicating that the validation failed with a specific `ContractError`
/// type.
pub fn channel_open_try_msg_validate(
    channel: &ChannelEnd,
    conn_end_on_b: &ConnectionEnd,
) -> Result<(), ContractError> {
    if !conn_end_on_b.state_matches(&ConnectionState::Open) {
        return Err(ChannelError::ConnectionNotOpen {
            connection_id: channel.connection_hops[0].clone(),
        })
        .map_err(Into::<ContractError>::into);
    };

    let conn_version = match conn_end_on_b.versions() {
        [version] => version,
        _ => {
            return Err(ChannelError::InvalidVersionLengthConnection)
                .map_err(Into::<ContractError>::into)
        }
    };

    let channel_feature = channel.ordering.to_string();
    if !conn_version.is_supported_feature(channel_feature) {
        return Err(ChannelError::ChannelFeatureNotSupportedByConnection)
            .map_err(Into::<ContractError>::into);
    }

    Ok(())
}

/// This function creates an IBC channel open try message based on the provided channel end, port ID,
/// channel ID, and connection ID.
///
/// Arguments:
///
/// * `msg`: A reference to a ChannelEnd struct, which contains information about the channel.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The channel identifier, which is a unique identifier for the channel within the
/// given port and connection.
/// * `connection_id`: The ID of the connection associated with the channel being opened.
pub fn on_chan_open_try_submessage(
    msg: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelOpenMsg {
    let port_id = port_id.clone();
    let channel_id = channel_id;
    let counter_party_port_id = msg.counterparty().port_id.clone();
    let counter_party_channel = msg.counterparty().channel_id().unwrap().clone();
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
        msg.ordering().to_ibc_order().unwrap(),
        msg.version.to_string(),
        connection_id.to_string(),
    );
    cosmwasm_std::IbcChannelOpenMsg::OpenTry {
        channel: ibc_channel,
        counterparty_version: msg.version.to_string(),
    }
}
