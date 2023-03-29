use super::*;

pub fn channel_open_try_msg_validate(
    message: &MsgChannelOpenTry,
    conn_end_on_b: &ConnectionEnd,
) -> Result<(), ContractError> {
    if !conn_end_on_b.state_matches(&ConnectionState::Open) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ConnectionNotOpen {
                connection_id: message.connection_hops_on_b[0].clone(),
            },
        });
    };

    let conn_version = match conn_end_on_b.versions() {
        [version] => version,
        _ => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidVersionLengthConnection,
            })
        }
    };

    let channel_feature = message.ordering.to_string();
    if !conn_version.is_supported_feature(channel_feature) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelFeatureNotSupportedByConnection,
        });
    }

    // TODO verification 

    Ok(())
}

pub fn on_chan_open_try_submessage(
    msg: &MsgChannelOpenTry,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelOpenMsg {
    let port_id = msg.port_id_on_b.clone();
    let channel_id = channel_id.ibc_channel_id();
    let counter_party_port_id = msg.port_id_on_a.clone();
    let counter_party_channel = msg.chan_id_on_a.clone();
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
        cosmwasm_std::IbcOrder::Unordered,
        msg.version_supported_on_a.to_string(),
        connection_id.connection_id().to_string(),
    );
    let data = cosmwasm_std::IbcChannelOpenMsg::OpenTry {
        channel: ibc_channel,
        counterparty_version: msg.version_supported_on_a.to_string(),
    };
    data
}
