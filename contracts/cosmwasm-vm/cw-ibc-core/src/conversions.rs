use std::str::FromStr;

use common::ibc::{core::ics04_channel::timeout::TimeoutHeight, Height};
use cw_common::{
    ibc_types::{ChannelEnd, ChannelError, IbcChannelId, IbcPortId, IbcTimestamp},
    raw_types::{channel::RawChannel, RawHeight},
};

use crate::ContractError;

pub fn to_ibc_port_id(port_id: &str) -> Result<IbcPortId, ContractError> {
    let port_id =
        IbcPortId::from_str(port_id).map_err(|e| ContractError::IbcValidationError { error: e })?;
    Ok(port_id)
}

pub fn to_ibc_channel_id(channel_id: &str) -> Result<IbcChannelId, ContractError> {
    let channel_id = IbcChannelId::from_str(channel_id)
        .map_err(|e| ContractError::IbcValidationError { error: e })?;
    Ok(channel_id)
}

pub fn to_ibc_height(height: RawHeight) -> Result<Height, ContractError> {
    let height =
        Height::try_from(height).map_err(|e| ContractError::IbcClientError { error: e })?;
    Ok(height)
}

pub fn to_ibc_timeout_height(height: Option<RawHeight>) -> Result<TimeoutHeight, ContractError> {
    let height =
        TimeoutHeight::try_from(height).map_err(|e| ContractError::IbcClientError { error: e })?;
    Ok(height)
}

pub fn to_ibc_timestamp(nanoseconds: u64) -> Result<IbcTimestamp, ContractError> {
    let time_stamp =
        IbcTimestamp::from_nanoseconds(nanoseconds).map_err(|e| ContractError::IbcPacketError {
            error: cw_common::ibc_types::PacketError::InvalidPacketTimestamp(e),
        })?;
    Ok(time_stamp)
}

pub fn to_ibc_channel(channel: Option<RawChannel>) -> Result<ChannelEnd, ContractError> {
    let chan = match channel {
        Some(chan) => ChannelEnd::try_from(chan),
        None => Err(ChannelError::MissingChannel),
    };
    chan.map_err(|e| ContractError::IbcChannelError { error: e })
}
