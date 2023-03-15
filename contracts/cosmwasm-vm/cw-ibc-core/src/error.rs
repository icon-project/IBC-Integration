use super::*;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
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
}
