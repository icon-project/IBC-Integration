use std::str::FromStr;

use crate::ContractError;
use common::ibc::core::ics03_connection::version::Version;
use common::ibc::{
    core::{
        ics03_connection::{connection::Counterparty, error::ConnectionError},
        ics04_channel::timeout::TimeoutHeight,
    },
    Height,
};
use cosmwasm_std::{IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock};
use cw_common::ibc_types::Sequence;
use cw_common::raw_types::channel::RawPacket;
use cw_common::raw_types::connection::RawCounterparty;
use cw_common::{
    ibc_types::{
        ChannelEnd, ChannelError, IbcChannelId, IbcClientId, IbcConnectionId, IbcPortId,
        IbcTimestamp,
    },
    raw_types::{channel::RawChannel, RawHeight, RawVersion},
};

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

pub fn to_ibc_height(height: Option<RawHeight>) -> Result<Height, ContractError> {
    if let Some(height) = height {
        let height =
            Height::try_from(height).map_err(|e| ContractError::IbcClientError { error: e })?;
        return Ok(height);
    }
    Err(ContractError::InvalidHeight)
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

pub fn to_ibc_client_id(client_id: &str) -> Result<IbcClientId, ContractError> {
    let client_id = IbcClientId::from_str(client_id)
        .map_err(|e| ContractError::IbcValidationError { error: e })?;
    Ok(client_id)
}

pub fn to_ibc_version(version: Option<RawVersion>) -> Result<Version, ContractError> {
    if let Some(version) = version {
        let ibc_version = Version::try_from(version)
            .map_err(|e| ContractError::IbcConnectionError { error: e })?;
        return Ok(ibc_version);
    }
    Err(ContractError::IbcConnectionError {
        error: ConnectionError::EmptyVersions,
    })
}

pub fn to_ibc_counterparty(
    counterparty: Option<RawCounterparty>,
) -> Result<Counterparty, ContractError> {
    if let Some(cp) = counterparty {
        let ibc_counterparty = Counterparty::try_from(cp)
            .map_err(|e| ContractError::IbcConnectionError { error: e })?;
        return Ok(ibc_counterparty);
    }
    Err(ContractError::IbcConnectionError {
        error: ConnectionError::MissingCounterparty,
    })
}

pub fn to_ibc_versions(versions: Vec<RawVersion>) -> Result<Vec<Version>, ContractError> {
    let ibc_versions = versions
        .into_iter()
        .map(Version::try_from)
        .collect::<Result<Vec<Version>, ConnectionError>>()
        .map_err(|e| ContractError::IbcConnectionError { error: e })?;
    Ok(ibc_versions)
}

pub fn to_ibc_connection_id(connection_id: &str) -> Result<IbcConnectionId, ContractError> {
    IbcConnectionId::from_str(connection_id)
        .map_err(|e| ContractError::IbcValidationError { error: e })
}

pub fn to_ibc_timeout_block(packet_timeout_height: &TimeoutHeight) -> IbcTimeoutBlock {
    match packet_timeout_height {
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => IbcTimeoutBlock {
            revision: 1,
            height: 1,
        },
        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => IbcTimeoutBlock {
            revision: x.revision_number(),
            height: x.revision_height(),
        },
    }
}

pub fn to_ibc_packet(packet: RawPacket) -> Result<IbcPacket, ContractError> {
    let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;
    let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;
    let _packet_sequence = Sequence::from(packet.sequence);
    let src = IbcEndpoint {
        port_id: packet.source_port.to_string(),
        channel_id: packet.source_channel.to_string(),
    };
    let dest = IbcEndpoint {
        port_id: packet.destination_port.to_string(),
        channel_id: packet.destination_channel.to_string(),
    };
    let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
    let timestamp = packet_timestamp.nanoseconds();
    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);
    let ibc_packet = IbcPacket::new(packet.data, src, dest, packet.sequence, timeout);
    Ok(ibc_packet)
}
