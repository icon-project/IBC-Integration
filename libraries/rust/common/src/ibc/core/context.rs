use crate::ibc::prelude::*;

use super::{
    ics02_client::error::ClientError,
    ics03_connection::error::ConnectionError,
    ics04_channel::error::{ChannelError, PacketError},
};

//use crate::ibc::core::ics04_channel::msgs::{ChannelMsg, PacketMsg};

use displaydoc::Display;

#[derive(Debug, Display)]
pub enum ContextError {
    /// ICS02 Client error
    ClientError(ClientError),
    /// ICS03 Connection error
    ConnectionError(ConnectionError),
    /// Ics04 Channel error
    ChannelError(ChannelError),
    /// ICS04 Packet error
    PacketError(PacketError),
}

impl From<ClientError> for ContextError {
    fn from(err: ClientError) -> ContextError {
        Self::ClientError(err)
    }
}

impl From<ConnectionError> for ContextError {
    fn from(err: ConnectionError) -> ContextError {
        Self::ConnectionError(err)
    }
}

impl From<ChannelError> for ContextError {
    fn from(err: ChannelError) -> ContextError {
        Self::ChannelError(err)
    }
}

impl From<PacketError> for ContextError {
    fn from(err: PacketError) -> ContextError {
        Self::PacketError(err)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Self::ClientError(e) => Some(e),
            Self::ConnectionError(e) => Some(e),
            Self::ChannelError(e) => Some(e),
            Self::PacketError(e) => Some(e),
        }
    }
}
