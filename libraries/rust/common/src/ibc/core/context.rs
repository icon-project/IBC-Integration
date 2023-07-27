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

// pub trait Router {
//     /// Returns a reference to a `Module` registered against the specified `ModuleId`
//     fn get_route(&self, module_id: &ModuleId) -> Option<&dyn Module>;

//     /// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
//     fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module>;

//     /// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
//     fn has_route(&self, module_id: &ModuleId) -> bool;

//     /// Return the module_id associated with a given port_id
//     fn lookup_module_by_port(&self, port_id: &PortId) -> Option<ModuleId>;

//     fn lookup_module_channel(&self, msg: &ChannelMsg) -> Result<ModuleId, ChannelError> {
//         let port_id = match msg {
//             ChannelMsg::OpenInit(msg) => &msg.port_id_on_a,
//             ChannelMsg::OpenTry(msg) => &msg.port_id_on_b,
//             ChannelMsg::OpenAck(msg) => &msg.port_id_on_a,
//             ChannelMsg::OpenConfirm(msg) => &msg.port_id_on_b,
//             ChannelMsg::CloseInit(msg) => &msg.port_id_on_a,
//             ChannelMsg::CloseConfirm(msg) => &msg.port_id_on_b,
//         };
//         let module_id = self
//             .lookup_module_by_port(port_id)
//             .ok_or(ChannelError::Port(UnknownPort {
//                 port_id: port_id.clone(),
//             }))?;
//         Ok(module_id)
//     }

//     fn lookup_module_packet(&self, msg: &PacketMsg) -> Result<ModuleId, ChannelError> {
//         let port_id = match msg {
//             PacketMsg::Recv(msg) => &msg.packet.port_id_on_b,
//             PacketMsg::Ack(msg) => &msg.packet.port_id_on_a,
//             PacketMsg::Timeout(msg) => &msg.packet.port_id_on_a,
//             PacketMsg::TimeoutOnClose(msg) => &msg.packet.port_id_on_a,
//         };
//         let module_id = self
//             .lookup_module_by_port(port_id)
//             .ok_or(ChannelError::Port(UnknownPort {
//                 port_id: port_id.clone(),
//             }))?;
//         Ok(module_id)
//     }
// }
