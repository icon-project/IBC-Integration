use cosmwasm_std::{Binary, to_binary, attr};
use cw_common::types::Ack;
use cw_common::cw_types::{CwPacket,CwBasicResponse};
use crate::error::ContractError;


/// This Rust function returns a binary representation of a successful Ack result.
///
/// Returns:
///
/// a binary representation of an `Ack` struct with a `Result` variant containing the byte string `"1"`.
pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());

    to_binary(&res).unwrap()
}

/// This Rust function creates a binary representation of an Ack error message with a given error
/// string.
///
/// Arguments:
///
/// * `err`: The `err` parameter is a `String` that represents the error message to be included in the
/// `Ack` enum variant `Error`. This function takes the error message as input and returns a binary
/// representation of the `Ack` enum variant `Error` using the `to_binary` function.
///
/// Returns:
///
/// A binary representation of an `Ack` enum variant `Error` with the provided error message as a
/// parameter.
pub fn make_ack_fail(err: String) -> Binary {
    let res = Ack::Error(err);

    to_binary(&res).unwrap()
}

/// This function handles a successful acknowledgement of an IBC packet and returns a response with
/// relevant attributes.
///
/// Arguments:
///
/// * `packet`: The `packet` parameter is of type `IbcPacket`, which is a struct representing an
/// Inter-Blockchain Communication (IBC) packet. It contains information such as the source and
/// destination chain IDs, the packet data, and the sequence and timeout values.
///
/// Returns:
///
/// a `Result` with either an `IbcBasicResponse` or a `ContractError`.
pub fn on_ack_sucess(_packet: CwPacket) -> Result<CwBasicResponse, ContractError> {
    let attributes = vec![attr("action", "acknowledge"), attr("success", "true")];

    Ok(CwBasicResponse::new().add_attributes(attributes))
}

pub fn on_ack_failure(_packet: CwPacket, error: &str) -> Result<CwBasicResponse, ContractError> {
    Ok(CwBasicResponse::new()
        .add_attribute("action", "acknowledge")
        .add_attribute("success", "false")
        .add_attribute("error", error))
}
