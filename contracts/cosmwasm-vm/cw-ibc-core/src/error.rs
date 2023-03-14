use cosmwasm_std::StdError;
use thiserror::Error;
use super::*;

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
    MissingNextRecvSeq{
        port_id: PortId,
        channel_id: ChannelId,
    },
}
