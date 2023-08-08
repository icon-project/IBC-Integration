use debug_print::debug_println;
use cw_common::cw_println;

use super::*;

/// These are constants used in the IBC (Inter-Blockchain Communication) protocol implementation in the
/// Rust programming language.
pub const IBC_VERSION: &str = "ics20-1";
pub const APP_ORDER: CwOrder = CwOrder::Unordered;

/// This function handles the opening of an IBC channel and performs some checks before returning a
/// response.
///
/// Arguments:
///
/// * `_deps`: _deps is a mutable dependency injector that provides access to the necessary dependencies
/// for the function to execute, such as the storage, API, and other modules.
/// * `_env`: _env is an object that represents the current execution environment of the contract. It
/// contains information such as the current block height, the sender address, the contract address, and
/// the current time.
/// * `msg`: The `msg` parameter is of type `IbcChannelOpenMsg`, which is a struct that contains
/// information about the channel being opened in an IBC transaction. It includes details such as the
/// channel's order (whether it is ordered or unordered), the counterparty's version (if applicable),
/// and
///
/// Returns:
///
/// a `Result` containing an `IbcChannelOpenResponse` or a `ContractError`. The `IbcChannelOpenResponse`
/// is wrapped in an `Ok` variant and contains an `Ibc3ChannelOpenResponse` struct with a `version`
/// field set to a string representing the IBC version.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: CwChannelOpenMsg,
) -> Result<CwChannelOpenResponse, ContractError> {
    let mut service = CwIbcConnection::default();
    let _res = service.on_channel_open(deps, msg)?;

    Ok(Some(Cw3ChannelOpenResponse {
        version: IBC_VERSION.to_string(),
    }))
}

/// This function connects two IBC channels and saves their configuration.
///
/// Arguments:
///
/// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It is used to interact
/// with the blockchain state and perform operations such as reading and writing to storage, querying
/// the current block height, and sending messages to other contracts.
/// * `_env`: _env is an input parameter of type `Env` which represents the current blockchain
/// environment. It contains information such as the current block height, time, and chain ID.
/// * `msg`: The `msg` parameter is of type `IbcChannelConnectMsg`, which contains information about the
/// channel to be connected, including the channel order, counterparty version, source endpoint, and
/// destination endpoint.
///
/// Returns:
///
/// a `Result` with either an `IbcBasicResponse` or a `ContractError`.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: CwChannelConnectMsg,
) -> Result<CwBasicResponse, ContractError> {
    let mut service = CwIbcConnection::default();
    let res = service.on_channel_connect(deps, msg)?;
    Ok(CwBasicResponse::new()
        .add_attributes(res.attributes)
        .add_events(res.events))
}

/// This Rust function handles closing an IBC channel and resets its state.
///
/// Arguments:
///
/// * `_deps`: DepsMut is a mutable dependency container that provides access to the necessary
/// dependencies required for executing the contract code. These dependencies include the storage, API,
/// and other modules required for the contract to function properly.
/// * `_env`: _env is an object of type `Env` which represents the current execution environment of the
/// contract. It contains information such as the current block height, time, and chain ID.
/// * `msg`: The `msg` parameter is of type `IbcChannelCloseMsg`, which is a struct that contains
/// information about the channel being closed. It includes the channel endpoint, which contains the
/// channel ID and the port ID, as well as any additional attributes that were included in the close
/// message.
///
/// Returns:
///
/// a `Result` object that contains either an `IbcBasicResponse` or a `ContractError`.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    msg: CwChannelCloseMsg,
) -> Result<CwBasicResponse, ContractError> {
    let service = CwIbcConnection::default();
    let res = service.on_channel_close(msg)?;

    Ok(CwBasicResponse::new()
        .add_attributes(res.attributes)
        .add_events(res.events))
}

/// This function receives an IBC packet and returns a response or an error message.
///
/// Arguments:
///
/// * `deps`: `deps` is a mutable dependency injector that provides access to the necessary dependencies
/// for executing the function. It is used to access the necessary modules and traits required for the
/// function to execute properly.
/// * `env`: `env` is an object of type `Env` which contains information about the current blockchain
/// environment, such as the block height, time, and chain ID. It is used in the `ibc_packet_receive`
/// function to access this information when processing IBC packets.
/// * `msg`: The `msg` parameter is of type `IbcPacketReceiveMsg` and contains the data of the IBC
/// packet being received. It includes information such as the source and destination chain IDs, the
/// packet sequence, and the actual packet data.
///
/// Returns:
///
/// The function `ibc_packet_receive` returns a `Result` with either an `IbcReceiveResponse` if the
/// `do_ibc_packet_receive` function call is successful, or a `Never` type if there is an error. If
/// there is an error, the function returns an `IbcReceiveResponse` with an error message and a failed
/// acknowledgement.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: CwPacketReceiveMsg,
) -> Result<CwReceiveResponse, Never> {
    let call_service = CwIbcConnection::default();
    let _channel = msg.packet.dest.channel_id.clone();
    cw_println!(deps,"[IBCConnection]: Packet Received");
    let result = call_service.do_packet_receive(deps, msg.packet, msg.relayer);

    match result {
        Ok(response) => Ok(response),
        Err(error) => Ok(CwReceiveResponse::new()
            .add_attribute("method", "ibc_packet_receive")
            .add_attribute("error", error.to_string())
            .set_ack(make_ack_fail(error.to_string()))),
    }
}

/// This function handles the acknowledgement of an IBC packet in Rust.
///
/// Arguments:
///
/// * `_deps`: _deps is a mutable dependency injector that provides access to the necessary dependencies
/// for the function to execute, such as the storage and API interfaces.
/// * `_env`: _env is a variable of type Env which represents the current execution environment of the
/// contract. It contains information such as the current block height, the sender address, and the
/// current time. It is passed as a parameter to the ibc_packet_ack function.
/// * `ack`: `ack` is a parameter of type `IbcPacketAckMsg`, which represents the acknowledgement
/// message for an IBC packet. It contains information about the original packet that was sent and the
/// acknowledgement data received in response.
///
/// Returns:
///
/// a `Result` object with an `IbcBasicResponse` on success or a `ContractError` on failure.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    ack: CwPacketAckMsg,
) -> Result<CwBasicResponse, ContractError> {
    let call_service = CwIbcConnection::default();
    let res = call_service.on_packet_ack(deps, ack)?;
    Ok(CwBasicResponse::new()
        .add_attributes(res.attributes)
        .add_events(res.events))
}

/// This Rust function handles a timeout for an IBC packet and sends a reply message with an error code.
///
/// Arguments:
///
/// * `_deps`: _deps is a mutable dependency injector that provides access to the necessary dependencies
/// for executing the function. It is typically used to access the storage, API, and other modules
/// required for the function to execute.
/// * `_env`: _env is an object of type Env which contains information about the current blockchain
/// environment, such as the block height, time, and chain ID. It is used in this function to provide
/// context for the IBC packet timeout message.
/// * `_msg`: The _msg parameter is of type IbcPacketTimeoutMsg, which represents a message indicating
/// that a previously sent IBC packet has timed out and failed to be acknowledged by the receiving
/// chain.
///
/// Returns:
///
/// a `Result` with an `IbcBasicResponse` on success or a `ContractError` on failure.
#[cfg_attr(feature = "native_ibc", entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: CwPacketTimeoutMsg,
) -> Result<CwBasicResponse, ContractError> {
    let call_service = CwIbcConnection::default();
    let res = call_service.on_packet_timeout(deps, msg)?;
    Ok(CwBasicResponse::new().add_attributes(res.attributes))
}
