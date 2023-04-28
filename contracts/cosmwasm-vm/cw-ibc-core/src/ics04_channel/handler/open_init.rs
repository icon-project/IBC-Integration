use cosmwasm_std::Coin;

use super::*;

pub fn on_chan_open_init_submessage(
    msg: &MsgChannelOpenInit,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelOpenMsg {
    let port_id = msg.port_id_on_a.clone();
    let channel_id = channel_id.ibc_channel_id();
    let counter_party_port_id = msg.port_id_on_b.clone();
    let counter_party_channel = IbcChannelId::default();
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
        msg.version_proposal.to_string(),
        connection_id.connection_id().to_string(),
    );
    cosmwasm_std::IbcChannelOpenMsg::OpenInit {
        channel: ibc_channel,
    }
}

pub fn create_channel_submesssage(
    address: String,
    msg: Binary,
    funds: Vec<Coin>,
    id: u64,
) -> SubMsg {
    let on_channel: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: address,
        msg,
        funds,
    });
    let sub_msg: SubMsg =
        SubMsg::reply_on_success(on_channel, id).with_gas_limit(GAS_FOR_SUBMESSAGE_XCALL);

    sub_msg
}

pub fn channel_open_init_msg_validate(
    message: &MsgChannelOpenInit,
    conn_end_on_a: ConnectionEnd,
) -> Result<(), ContractError> {
    let conn_version = match conn_end_on_a.versions() {
        [version] => version,
        _ => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidVersionLengthConnection,
            })
        }
    };
    let channel_feature = message.ordering.to_string();
    // channel version should be valid
    if !conn_version.is_supported_feature(channel_feature) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelFeatureNotSupportedByConnection,
        });
    };

    Ok(())
}
