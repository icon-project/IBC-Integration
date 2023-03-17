use super::*;
use cosmwasm_std::StdError;
use ibc::core::ContextError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("ChannelNotFound {port_id} {channel_id}")]
    ChannelNotFound {
        port_id: PortId,
        channel_id: ChannelId,
    },

    #[error("MissingNextRecvSeq {port_id} {channel_id}")]
    MissingNextRecvSeq {
        port_id: PortId,
        channel_id: ChannelId,
    },

    #[error("MissingNextSendSeq {port_id} {channel_id}")]
    MissingNextSendSeq {
        port_id: PortId,
        channel_id: ChannelId,
    },

    #[error("MissingNextAckSeq {port_id} {channel_id}")]
    MissingNextAckSeq {
        port_id: PortId,
        channel_id: ChannelId,
    },
    #[error("InvalidClientId {client_id}")]
    InvalidClientId { client_id: String },

    #[error("InvalidClientType {client_type}")]
    InvalidClientType { client_type: String },

    #[error("InvalidNextClientSequence")]
    InvalidNextClientSequence {},

    #[error("IbcContextError {error}")]
    IbcContextError { error: ContextError },

    #[error("IbcDecodeError {error}")]
    IbcDecodeError { error: String },
}
