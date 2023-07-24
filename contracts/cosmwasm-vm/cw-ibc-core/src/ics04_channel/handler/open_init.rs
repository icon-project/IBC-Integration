use cosmwasm_std::{Coin, ReplyOn};

use super::*;

/// The function creates an IBC channel open message for channel initialization.
///
/// Arguments:
///
/// * `msg`: A reference to a `MsgChannelOpenInit` struct, which contains the information needed to
/// initialize a new IBC channel.
/// * `channel_id`: The unique identifier of the channel being opened.
/// * `connection_id`: The ID of the connection associated with the channel being opened.
pub fn on_chan_open_init_submessage(
    channel: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelOpenMsg {
   
    let channel_id = channel_id;
    let counter_party_port_id = channel.remote.port_id();
    let counter_party_channel = channel.remote.channel_id.as_ref().map(|e|e.to_string()).unwrap_or("".to_string());
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
        channel.ordering.to_ibc_order().unwrap(),
        channel.version.to_string(),
        connection_id.to_string(),
    );
    cosmwasm_std::IbcChannelOpenMsg::OpenInit {
        channel: ibc_channel,
    }
}

/// This function creates a submessage to execute a WasmMsg on a given contract address with specified
/// funds and returns a reply on success.
///
/// Arguments:
///
/// * `address`: The address of the smart contract to which the submessage will be sent.
/// * `msg`: The `msg` parameter is of type `Binary` and represents the message payload that will be
/// sent to the smart contract at the given `address`. This message payload is expected to be in binary
/// format.
/// * `funds`: The `funds` parameter is a vector of `Coin` structs, which represent the amount of tokens
/// to be sent along with the message. These tokens will be transferred to the contract address
/// specified in the `address` parameter.
/// * `id`: The `id` parameter is an unsigned 64-bit integer used to identify the submessage. It is
/// typically used to match the response of the submessage with the original message that triggered it.
///
/// Returns:
///
/// a `SubMsg` object.
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

    SubMsg {
        id,
        msg: on_channel,
        gas_limit: Some(GAS_FOR_SUBMESSAGE_XCALL),
        reply_on: ReplyOn::Always,
    }
}

/// This function validates the channel open initialization message.
///
/// Arguments:
///
/// * `message`: A reference to a `MsgChannelOpenInit` struct, which contains information about the
/// initial channel opening message.
/// * `conn_end_on_a`: `conn_end_on_a` is a variable of type `ConnectionEnd` which represents one end of
/// a connection in the Inter-Blockchain Communication (IBC) protocol. It contains information about the
/// connection, such as its state, version, and counterparty. In the function
/// `channel_open_init_msg_validate
///
/// Returns:
///
/// a `Result` type with either an `Ok(())` value indicating that the validation was successful, or an
/// `Err` value with a `ContractError` type indicating that the validation failed with a specific error
/// message.
pub fn channel_open_init_msg_validate(
    channel: &ChannelEnd,
    conn_end_on_a: ConnectionEnd,
) -> Result<(), ContractError> {
    let conn_version = match conn_end_on_a.versions() {
        [version] => version,
        _ => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidVersionLengthConnection,
            });
        }
    };
    let channel_feature = channel.ordering.to_string();
    // channel version should be valid
    if !conn_version.is_supported_feature(channel_feature) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelFeatureNotSupportedByConnection,
        });
    };

    Ok(())
}
