use crate::ibc::prelude::*;

use super::{
    ics02_client::error::ClientError,
    ics03_connection::error::ConnectionError,
    ics04_channel::error::{ChannelError, PacketError},
};
use core::time::Duration;

use ibc_proto::google::protobuf::Any;

use crate::ibc::core::ics02_client::client_state::ClientState;
use crate::ibc::core::ics02_client::client_type::ClientType;
use crate::ibc::core::ics02_client::consensus_state::ConsensusState;
use crate::ibc::core::ics03_connection::connection::ConnectionEnd;
use crate::ibc::core::ics03_connection::version::{
    get_compatible_versions, pick_version, Version as ConnectionVersion,
};
use crate::ibc::core::ics04_channel::channel::ChannelEnd;
use crate::ibc::core::ics04_channel::commitment::{AcknowledgementCommitment, PacketCommitment};

use crate::ibc::core::ics04_channel::msgs::{ChannelMsg, PacketMsg};
use crate::ibc::core::ics04_channel::packet::{Receipt, Sequence};
use crate::ibc::core::ics05_port::error::PortError::UnknownPort;
use crate::ibc::core::ics23_commitment::commitment::CommitmentPrefix;
use crate::ibc::core::ics24_host::identifier::{ConnectionId, PortId};
use crate::ibc::core::ics24_host::path::{
    AckPath, ChannelEndPath, ClientConnectionPath, ClientConsensusStatePath, ClientStatePath,
    ClientTypePath, CommitmentPath, ConnectionPath, ReceiptPath, SeqAckPath, SeqRecvPath,
    SeqSendPath,
};
use crate::ibc::core::ics26_routing::context::{Module, ModuleId};
use crate::ibc::core::{
    ics02_client::msgs::ClientMsg,
    ics03_connection::msgs::ConnectionMsg,
    ics24_host::identifier::ClientId,
    ics26_routing::{error::RouterError, msgs::MsgEnvelope},
};
use crate::ibc::events::IbcEvent;
use crate::ibc::timestamp::Timestamp;
use crate::ibc::Height;

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

pub trait Router {
    /// Returns a reference to a `Module` registered against the specified `ModuleId`
    fn get_route(&self, module_id: &ModuleId) -> Option<&dyn Module>;

    /// Returns a mutable reference to a `Module` registered against the specified `ModuleId`
    fn get_route_mut(&mut self, module_id: &ModuleId) -> Option<&mut dyn Module>;

    /// Returns true if the `Router` has a `Module` registered against the specified `ModuleId`
    fn has_route(&self, module_id: &ModuleId) -> bool;

    /// Return the module_id associated with a given port_id
    fn lookup_module_by_port(&self, port_id: &PortId) -> Option<ModuleId>;

    fn lookup_module_channel(&self, msg: &ChannelMsg) -> Result<ModuleId, ChannelError> {
        let port_id = match msg {
            ChannelMsg::OpenInit(msg) => &msg.port_id_on_a,
            ChannelMsg::OpenTry(msg) => &msg.port_id_on_b,
            ChannelMsg::OpenAck(msg) => &msg.port_id_on_a,
            ChannelMsg::OpenConfirm(msg) => &msg.port_id_on_b,
            ChannelMsg::CloseInit(msg) => &msg.port_id_on_a,
            ChannelMsg::CloseConfirm(msg) => &msg.port_id_on_b,
        };
        let module_id = self
            .lookup_module_by_port(port_id)
            .ok_or(ChannelError::Port(UnknownPort {
                port_id: port_id.clone(),
            }))?;
        Ok(module_id)
    }

    fn lookup_module_packet(&self, msg: &PacketMsg) -> Result<ModuleId, ChannelError> {
        let port_id = match msg {
            PacketMsg::Recv(msg) => &msg.packet.port_id_on_b,
            PacketMsg::Ack(msg) => &msg.packet.port_id_on_a,
            PacketMsg::Timeout(msg) => &msg.packet.port_id_on_a,
            PacketMsg::TimeoutOnClose(msg) => &msg.packet.port_id_on_a,
        };
        let module_id = self
            .lookup_module_by_port(port_id)
            .ok_or(ChannelError::Port(UnknownPort {
                port_id: port_id.clone(),
            }))?;
        Ok(module_id)
    }
}

// pub trait ValidationContext: Router {
//     /// Validation entrypoint.
//     fn validate(&self, msg: MsgEnvelope) -> Result<(), RouterError>
//     where
//         Self: Sized,
//     {
//         match msg {
//             MsgEnvelope::Client(msg) => match msg {
//                 ClientMsg::CreateClient(msg) => create_client::validate(self, msg),
//                 ClientMsg::UpdateClient(msg) => update_client::validate(self, msg),
//                 ClientMsg::Misbehaviour(msg) => misbehaviour::validate(self, msg),
//                 ClientMsg::UpgradeClient(msg) => upgrade_client::validate(self, msg),
//             }
//             .map_err(RouterError::ContextError),
//             MsgEnvelope::Connection(msg) => match msg {
//                 ConnectionMsg::OpenInit(msg) => conn_open_init::validate(self, msg),
//                 ConnectionMsg::OpenTry(msg) => conn_open_try::validate(self, msg),
//                 ConnectionMsg::OpenAck(msg) => conn_open_ack::validate(self, msg),
//                 ConnectionMsg::OpenConfirm(ref msg) => conn_open_confirm::validate(self, msg),
//             }
//             .map_err(RouterError::ContextError),
//             MsgEnvelope::Channel(msg) => {
//                 let module_id = self
//                     .lookup_module_channel(&msg)
//                     .map_err(ContextError::from)?;
//                 if !self.has_route(&module_id) {
//                     return Err(ChannelError::RouteNotFound)
//                         .map_err(ContextError::ChannelError)
//                         .map_err(RouterError::ContextError);
//                 }

//                 match msg {
//                     ChannelMsg::OpenInit(msg) => chan_open_init_validate(self, module_id, msg),
//                     ChannelMsg::OpenTry(msg) => chan_open_try_validate(self, module_id, msg),
//                     ChannelMsg::OpenAck(msg) => chan_open_ack_validate(self, module_id, msg),
//                     ChannelMsg::OpenConfirm(msg) => {
//                         chan_open_confirm_validate(self, module_id, msg)
//                     }
//                     ChannelMsg::CloseInit(msg) => chan_close_init_validate(self, module_id, msg),
//                     ChannelMsg::CloseConfirm(msg) => {
//                         chan_close_confirm_validate(self, module_id, msg)
//                     }
//                 }
//                 .map_err(RouterError::ContextError)
//             }
//             MsgEnvelope::Packet(msg) => {
//                 let module_id = self
//                     .lookup_module_packet(&msg)
//                     .map_err(ContextError::from)?;
//                 if !self.has_route(&module_id) {
//                     return Err(ChannelError::RouteNotFound)
//                         .map_err(ContextError::ChannelError)
//                         .map_err(RouterError::ContextError);
//                 }

//                 match msg {
//                     PacketMsg::Recv(msg) => recv_packet_validate(self, msg),
//                     PacketMsg::Ack(msg) => acknowledgement_packet_validate(self, module_id, msg),
//                     PacketMsg::Timeout(msg) => {
//                         timeout_packet_validate(self, module_id, TimeoutMsgType::Timeout(msg))
//                     }
//                     PacketMsg::TimeoutOnClose(msg) => timeout_packet_validate(
//                         self,
//                         module_id,
//                         TimeoutMsgType::TimeoutOnClose(msg),
//                     ),
//                 }
//                 .map_err(RouterError::ContextError)
//             }
//         }
//     }

//     /// Returns the ClientState for the given identifier `client_id`.
//     fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>, ContextError>;

//     /// Tries to decode the given `client_state` into a concrete light client state.
//     fn decode_client_state(&self, client_state: Any) -> Result<Box<dyn ClientState>, ContextError>;

//     /// Retrieve the consensus state for the given client ID at the specified
//     /// height.
//     ///
//     /// Returns an error if no such state exists.
//     fn consensus_state(
//         &self,
//         client_cons_state_path: &ClientConsensusStatePath,
//     ) -> Result<Box<dyn ConsensusState>, ContextError>;

//     /// Search for the lowest consensus state higher than `height`.
//     fn next_consensus_state(
//         &self,
//         client_id: &ClientId,
//         height: &Height,
//     ) -> Result<Option<Box<dyn ConsensusState>>, ContextError>;

//     /// Search for the highest consensus state lower than `height`.
//     fn prev_consensus_state(
//         &self,
//         client_id: &ClientId,
//         height: &Height,
//     ) -> Result<Option<Box<dyn ConsensusState>>, ContextError>;

//     /// Returns the current height of the local chain.
//     fn host_height(&self) -> Result<Height, ContextError>;

//     /// Returns the current timestamp of the local chain.
//     fn host_timestamp(&self) -> Result<Timestamp, ContextError>;

//     /// Returns the `ConsensusState` of the host (local) chain at a specific height.
//     fn host_consensus_state(
//         &self,
//         height: &Height,
//     ) -> Result<Box<dyn ConsensusState>, ContextError>;

//     /// Returns a natural number, counting how many clients have been created
//     /// thus far. The value of this counter should increase only via method
//     /// `ExecutionContext::increase_client_counter`.
//     fn client_counter(&self) -> Result<u64, ContextError>;

//     /// Returns the ConnectionEnd for the given identifier `conn_id`.
//     fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd, ContextError>;

//     /// Validates the `ClientState` of the client (a client referring to host) stored on the counterparty chain against the host's internal state.
//     ///
//     /// For more information on the specific requirements for validating the
//     /// client state of a host chain, please refer to the [ICS24 host
//     /// requirements](https://github.com/cosmos/ibc/tree/main/spec/core/ics-024-host-requirements#client-state-validation)
//     ///
//     /// Additionally, implementations specific to individual chains can be found
//     /// in the [hosts](crate::hosts) module.
//     fn validate_self_client(
//         &self,
//         client_state_of_host_on_counterparty: Any,
//     ) -> Result<(), ContextError>;

//     /// Returns the prefix that the local chain uses in the KV store.
//     fn commitment_prefix(&self) -> CommitmentPrefix;

//     /// Returns a counter on how many connections have been created thus far.
//     fn connection_counter(&self) -> Result<u64, ContextError>;

//     /// Function required by ICS 03. Returns the list of all possible versions that the connection
//     /// handshake protocol supports.
//     fn get_compatible_versions(&self) -> Vec<ConnectionVersion> {
//         get_compatible_versions()
//     }

//     /// Function required by ICS 03. Returns one version out of the supplied list of versions, which the
//     /// connection handshake protocol prefers.
//     fn pick_version(
//         &self,
//         supported_versions: &[ConnectionVersion],
//         counterparty_candidate_versions: &[ConnectionVersion],
//     ) -> Result<ConnectionVersion, ContextError> {
//         pick_version(supported_versions, counterparty_candidate_versions)
//             .map_err(ContextError::ConnectionError)
//     }

//     /// Returns the ChannelEnd for the given `port_id` and `chan_id`.
//     fn channel_end(&self, channel_end_path: &ChannelEndPath) -> Result<ChannelEnd, ContextError>;

//     fn get_next_sequence_send(&self, seq_send_path: &SeqSendPath)
//         -> Result<Sequence, ContextError>;

//     fn get_next_sequence_recv(&self, seq_recv_path: &SeqRecvPath)
//         -> Result<Sequence, ContextError>;

//     fn get_next_sequence_ack(&self, seq_ack_path: &SeqAckPath) -> Result<Sequence, ContextError>;

//     fn get_packet_commitment(
//         &self,
//         commitment_path: &CommitmentPath,
//     ) -> Result<PacketCommitment, ContextError>;

//     fn get_packet_receipt(&self, receipt_path: &ReceiptPath) -> Result<Receipt, ContextError>;

//     fn get_packet_acknowledgement(
//         &self,
//         ack_path: &AckPath,
//     ) -> Result<AcknowledgementCommitment, ContextError>;

//     /// Returns the time when the client state for the given [`ClientId`] was updated with a header for the given [`Height`]
//     fn client_update_time(
//         &self,
//         client_id: &ClientId,
//         height: &Height,
//     ) -> Result<Timestamp, ContextError>;

//     /// Returns the height when the client state for the given [`ClientId`] was updated with a header for the given [`Height`]
//     fn client_update_height(
//         &self,
//         client_id: &ClientId,
//         height: &Height,
//     ) -> Result<Height, ContextError>;

//     /// Returns a counter on the number of channel ids have been created thus far.
//     /// The value of this counter should increase only via method
//     /// `ExecutionContext::increase_channel_counter`.
//     fn channel_counter(&self) -> Result<u64, ContextError>;

//     /// Returns the maximum expected time per block
//     fn max_expected_time_per_block(&self) -> Duration;

//     /// Calculates the block delay period using the connection's delay period and the maximum
//     /// expected time per block.
//     fn block_delay(&self, delay_period_time: &Duration) -> u64 {
//         calculate_block_delay(delay_period_time, &self.max_expected_time_per_block())
//     }
// }

// pub trait ExecutionContext: ValidationContext {
//     /// Execution entrypoint
//     fn execute(&mut self, msg: MsgEnvelope) -> Result<(), RouterError>
//     where
//         Self: Sized,
//     {
//         match msg {
//             MsgEnvelope::Client(msg) => match msg {
//                 ClientMsg::CreateClient(msg) => create_client::execute(self, msg),
//                 ClientMsg::UpdateClient(msg) => update_client::execute(self, msg),
//                 ClientMsg::Misbehaviour(msg) => misbehaviour::execute(self, msg),
//                 ClientMsg::UpgradeClient(msg) => upgrade_client::execute(self, msg),
//             }
//             .map_err(RouterError::ContextError),
//             MsgEnvelope::Connection(msg) => match msg {
//                 ConnectionMsg::OpenInit(msg) => conn_open_init::execute(self, msg),
//                 ConnectionMsg::OpenTry(msg) => conn_open_try::execute(self, msg),
//                 ConnectionMsg::OpenAck(msg) => conn_open_ack::execute(self, msg),
//                 ConnectionMsg::OpenConfirm(ref msg) => conn_open_confirm::execute(self, msg),
//             }
//             .map_err(RouterError::ContextError),
//             MsgEnvelope::Channel(msg) => {
//                 let module_id = self
//                     .lookup_module_channel(&msg)
//                     .map_err(ContextError::from)?;
//                 if !self.has_route(&module_id) {
//                     return Err(ChannelError::RouteNotFound)
//                         .map_err(ContextError::ChannelError)
//                         .map_err(RouterError::ContextError);
//                 }

//                 match msg {
//                     ChannelMsg::OpenInit(msg) => chan_open_init_execute(self, module_id, msg),
//                     ChannelMsg::OpenTry(msg) => chan_open_try_execute(self, module_id, msg),
//                     ChannelMsg::OpenAck(msg) => chan_open_ack_execute(self, module_id, msg),
//                     ChannelMsg::OpenConfirm(msg) => chan_open_confirm_execute(self, module_id, msg),
//                     ChannelMsg::CloseInit(msg) => chan_close_init_execute(self, module_id, msg),
//                     ChannelMsg::CloseConfirm(msg) => {
//                         chan_close_confirm_execute(self, module_id, msg)
//                     }
//                 }
//                 .map_err(RouterError::ContextError)
//             }
//             MsgEnvelope::Packet(msg) => {
//                 let module_id = self
//                     .lookup_module_packet(&msg)
//                     .map_err(ContextError::from)?;
//                 if !self.has_route(&module_id) {
//                     return Err(ChannelError::RouteNotFound)
//                         .map_err(ContextError::ChannelError)
//                         .map_err(RouterError::ContextError);
//                 }

//                 match msg {
//                     PacketMsg::Recv(msg) => recv_packet_execute(self, module_id, msg),
//                     PacketMsg::Ack(msg) => acknowledgement_packet_execute(self, module_id, msg),
//                     PacketMsg::Timeout(msg) => {
//                         timeout_packet_execute(self, module_id, TimeoutMsgType::Timeout(msg))
//                     }
//                     PacketMsg::TimeoutOnClose(msg) => {
//                         timeout_packet_execute(self, module_id, TimeoutMsgType::TimeoutOnClose(msg))
//                     }
//                 }
//                 .map_err(RouterError::ContextError)
//             }
//         }
//     }

//     /// Called upon successful client creation
//     fn store_client_type(
//         &mut self,
//         client_type_path: ClientTypePath,
//         client_type: ClientType,
//     ) -> Result<(), ContextError>;

//     /// Called upon successful client creation and update
//     fn store_client_state(
//         &mut self,
//         client_state_path: ClientStatePath,
//         client_state: Box<dyn ClientState>,
//     ) -> Result<(), ContextError>;

//     /// Called upon successful client creation and update
//     fn store_consensus_state(
//         &mut self,
//         consensus_state_path: ClientConsensusStatePath,
//         consensus_state: Box<dyn ConsensusState>,
//     ) -> Result<(), ContextError>;

//     /// Called upon client creation.
//     /// Increases the counter which keeps track of how many clients have been created.
//     /// Should never fail.
//     fn increase_client_counter(&mut self);

//     /// Called upon successful client update.
//     /// Implementations are expected to use this to record the specified time as the time at which
//     /// this update (or header) was processed.
//     fn store_update_time(
//         &mut self,
//         client_id: ClientId,
//         height: Height,
//         timestamp: Timestamp,
//     ) -> Result<(), ContextError>;

//     /// Called upon successful client update.
//     /// Implementations are expected to use this to record the specified height as the height at
//     /// at which this update (or header) was processed.
//     fn store_update_height(
//         &mut self,
//         client_id: ClientId,
//         height: Height,
//         host_height: Height,
//     ) -> Result<(), ContextError>;

//     /// Stores the given connection_end at path
//     fn store_connection(
//         &mut self,
//         connection_path: &ConnectionPath,
//         connection_end: ConnectionEnd,
//     ) -> Result<(), ContextError>;

//     /// Stores the given connection_id at a path associated with the client_id.
//     fn store_connection_to_client(
//         &mut self,
//         client_connection_path: &ClientConnectionPath,
//         conn_id: ConnectionId,
//     ) -> Result<(), ContextError>;

//     /// Called upon connection identifier creation (Init or Try process).
//     /// Increases the counter which keeps track of how many connections have been created.
//     /// Should never fail.
//     fn increase_connection_counter(&mut self);

//     fn store_packet_commitment(
//         &mut self,
//         commitment_path: &CommitmentPath,
//         commitment: PacketCommitment,
//     ) -> Result<(), ContextError>;

//     fn delete_packet_commitment(
//         &mut self,
//         commitment_path: &CommitmentPath,
//     ) -> Result<(), ContextError>;

//     fn store_packet_receipt(
//         &mut self,
//         receipt_path: &ReceiptPath,
//         receipt: Receipt,
//     ) -> Result<(), ContextError>;

//     fn store_packet_acknowledgement(
//         &mut self,
//         ack_path: &AckPath,
//         ack_commitment: AcknowledgementCommitment,
//     ) -> Result<(), ContextError>;

//     fn delete_packet_acknowledgement(&mut self, ack_path: &AckPath) -> Result<(), ContextError>;

//     /// Stores the given channel_end at a path associated with the port_id and channel_id.
//     fn store_channel(
//         &mut self,
//         channel_end_path: &ChannelEndPath,
//         channel_end: ChannelEnd,
//     ) -> Result<(), ContextError>;

//     fn store_next_sequence_send(
//         &mut self,
//         seq_send_path: &SeqSendPath,
//         seq: Sequence,
//     ) -> Result<(), ContextError>;

//     fn store_next_sequence_recv(
//         &mut self,
//         seq_recv_path: &SeqRecvPath,
//         seq: Sequence,
//     ) -> Result<(), ContextError>;

//     fn store_next_sequence_ack(
//         &mut self,
//         seq_ack_path: &SeqAckPath,
//         seq: Sequence,
//     ) -> Result<(), ContextError>;

//     /// Called upon channel identifier creation (Init or Try message processing).
//     /// Increases the counter which keeps track of how many channels have been created.
//     /// Should never fail.
//     fn increase_channel_counter(&mut self);

//     /// Ibc events
//     fn emit_ibc_event(&mut self, event: IbcEvent);

//     /// Logging facility
//     fn log_message(&mut self, message: String);
// }
