use crate::ibc::prelude::*;

use core::{
    fmt::{Debug, Display, Error as FmtError, Formatter},
    str::FromStr,
};
use prost::alloc::borrow::{Borrow, Cow};

//use crate::ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement;

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidModuleId;

#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(s: Cow<'_, str>) -> Result<Self, InvalidModuleId> {
        if !s.trim().is_empty() && s.chars().all(char::is_alphanumeric) {
            Ok(Self(s.into_owned()))
        } else {
            Err(InvalidModuleId)
        }
    }
}

impl Display for ModuleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ModuleId {
    type Err = InvalidModuleId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(Cow::Borrowed(s))
    }
}

impl Borrow<str> for ModuleId {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

// pub trait Module: Debug {
//     #[allow(clippy::too_many_arguments)]
//     fn on_chan_open_init_validate(
//         &self,
//         order: Order,
//         connection_hops: &[ConnectionId],
//         port_id: &PortId,
//         channel_id: &ChannelId,
//         counterparty: &Counterparty,
//         version: &Version,
//     ) -> Result<Version, ChannelError>;

//     #[allow(clippy::too_many_arguments)]
//     fn on_chan_open_init_execute(
//         &mut self,
//         order: Order,
//         connection_hops: &[ConnectionId],
//         port_id: &PortId,
//         channel_id: &ChannelId,
//         counterparty: &Counterparty,
//         version: &Version,
//     ) -> Result<(ModuleExtras, Version), ChannelError>;

//     #[allow(clippy::too_many_arguments)]
//     fn on_chan_open_try_validate(
//         &self,
//         order: Order,
//         connection_hops: &[ConnectionId],
//         port_id: &PortId,
//         channel_id: &ChannelId,
//         counterparty: &Counterparty,
//         counterparty_version: &Version,
//     ) -> Result<Version, ChannelError>;

//     #[allow(clippy::too_many_arguments)]
//     fn on_chan_open_try_execute(
//         &mut self,
//         order: Order,
//         connection_hops: &[ConnectionId],
//         port_id: &PortId,
//         channel_id: &ChannelId,
//         counterparty: &Counterparty,
//         counterparty_version: &Version,
//     ) -> Result<(ModuleExtras, Version), ChannelError>;

//     fn on_chan_open_ack_validate(
//         &self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//         _counterparty_version: &Version,
//     ) -> Result<(), ChannelError> {
//         Ok(())
//     }

//     fn on_chan_open_ack_execute(
//         &mut self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//         _counterparty_version: &Version,
//     ) -> Result<ModuleExtras, ChannelError> {
//         Ok(ModuleExtras::empty())
//     }

//     fn on_chan_open_confirm_validate(
//         &self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<(), ChannelError> {
//         Ok(())
//     }

//     fn on_chan_open_confirm_execute(
//         &mut self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<ModuleExtras, ChannelError> {
//         Ok(ModuleExtras::empty())
//     }

//     fn on_chan_close_init_validate(
//         &self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<(), ChannelError> {
//         Ok(())
//     }

//     fn on_chan_close_init_execute(
//         &mut self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<ModuleExtras, ChannelError> {
//         Ok(ModuleExtras::empty())
//     }

//     fn on_chan_close_confirm_validate(
//         &self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<(), ChannelError> {
//         Ok(())
//     }

//     fn on_chan_close_confirm_execute(
//         &mut self,
//         _port_id: &PortId,
//         _channel_id: &ChannelId,
//     ) -> Result<ModuleExtras, ChannelError> {
//         Ok(ModuleExtras::empty())
//     }

//     fn on_recv_packet_execute(
//         &mut self,
//         packet: &Packet,
//         relayer: &Signer,
//     ) -> (ModuleExtras, Acknowledgement);

//     fn on_acknowledgement_packet_validate(
//         &self,
//         _packet: &Packet,
//         _acknowledgement: &Acknowledgement,
//         _relayer: &Signer,
//     ) -> Result<(), PacketError>;

//     fn on_acknowledgement_packet_execute(
//         &mut self,
//         _packet: &Packet,
//         _acknowledgement: &Acknowledgement,
//         _relayer: &Signer,
//     ) -> (ModuleExtras, Result<(), PacketError>);

//     /// Note: `MsgTimeout` and `MsgTimeoutOnClose` use the same callback

//     fn on_timeout_packet_validate(
//         &self,
//         packet: &Packet,
//         relayer: &Signer,
//     ) -> Result<(), PacketError>;

//     /// Note: `MsgTimeout` and `MsgTimeoutOnClose` use the same callback

//     fn on_timeout_packet_execute(
//         &mut self,
//         packet: &Packet,
//         relayer: &Signer,
//     ) -> (ModuleExtras, Result<(), PacketError>);
// }
