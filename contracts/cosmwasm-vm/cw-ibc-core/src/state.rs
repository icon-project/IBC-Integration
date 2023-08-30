use std::marker::PhantomData;

use crate::{ics24_host::LastProcessedOn, light_client::light_client::LightClient};
use cosmwasm_std::Order;
use cw_storage_plus::Bound;
use std::collections::HashMap;

use debug_print::debug_println;

use super::*;

/// The `CwIbcStore` struct stores various data related to the Inter-Blockchain Communication (IBC).
///
/// Properties:
///
/// * `client_registry`: A mapping between client types and their corresponding client identifiers
/// (IDs).
///
/// * `client_types`: A mapping between a client ID and its corresponding client type.
///
/// * `client_states`: A mapping between a client ID and its corresponding state in byte.
///
/// * `consensus_states`: A mapping between a client ID and its corresponding consensus state.
///
/// * `client_implementations`: `client_implementations` is a mapping between `ClientId` and a string
/// representing the implementation of the client. In the context of the Cosmos SDK and the IBC
/// protocol, a client is a module that is responsible for verifying the validity of the state of a
/// remote blockchain. The implementation of a
///
/// * `next_sequence_send`: `next_seq_on_a_send` is a mapping between a tuple of `(PortId, ChannelId)`
/// and a `seq_on_a` number. It stores the next seq_on_a number that should be used when sending a
/// packet on the given channel. This is used to ensure that packets are sent in order and to prevent
/// replay
///
/// * `next_seq_on_a_recv`: `next_seq_on_a_recv` is a mapping between a tuple of `(PortId, ChannelId)`
/// and a `seq_on_a` value. It stores the next expected seq_on_a number for a packet to be received on a
/// particular channel. This is used to ensure that packets are received in the correct order and to
/// detect
///
/// * `next_seq_on_a_ack`: `next_seq_on_a_ack` is a mapping between a tuple of `(PortId, ChannelId)` and
/// a `seq_on_a` value. It stores the next expected seq_on_a number for an acknowledgement message to be
/// received on a particular channel. This is used to ensure that acknowledgement messages are received
/// in the correct order and
///
/// * `next_client_sequence`: `next_client_sequence` is an `Item` that stores the next available
/// seq_on_a number for creating a new client. It is likely used to ensure that each new client created
/// has a unique seq_on_a number.
///
/// * `next_connection_sequence`: `next_connection_sequence` is an `Item` that stores the next available
/// seq_on_a number for creating a new connection. It is used to ensure that each new connection has a
/// unique identifier.
///
/// * `next_channel_sequence`: `next_channel_sequence` is an `Item` that stores the next available
/// seq_on_a number for a channel. It is used to ensure that each channel has a unique seq_on_a number
/// when it is created.
///
/// * `client_connections`: A mapping between a client ID and its associated connection ID. This is used
/// to keep track of the connection associated with each client.
///
/// * `connections`: `connections` is a mapping between `ConnectionId` and a byte vector (`Vec<u8>`). It
/// stores the connection state associated with each connection identifier. This state can include
/// information such as the connection version, the connection status, and any associated metadata.
///
/// * `channels`: The `channels` property is a map that stores the channel end information for each
/// channel identified by a tuple of `(PortId, ChannelId)`. The `ChannelEnd` struct contains information
/// such as the channel state, ordering, and counterparty channel information.
///
/// * `port_to_module`: `port_to_module` is a mapping between `PortId` and `IbcModuleId`. It stores the
/// module identifier for each port. This is useful for routing packets between different modules in the
/// IBC protocol.
///
/// * `capabilities`: The `capabilities` property is a map that stores addresses based on capability
/// names. In the context of the Cosmos SDK and IBC (Inter-Blockchain Communication) protocol,
/// capabilities are used to grant permissions to modules to perform certain actions. This map allows
/// for easy lookup of addresses associated with specific capabilities.
///
/// * `commitments`: The `commitments` property is a map that stores commitments based on keys. The keys
/// can be PacketCommitment, AckCommitment, Connection, Channel, or Client. The values are byte arrays
/// that represent the commitments. This map is used to keep track of the commitments made during the
/// IBC
///
/// * `expected_time_per_block`: The expected time duration of a block in the blockchain network. This
/// is used to calculate the timeout for certain operations in the IBC protocol.
///
/// * `packet_receipts`: The `packet_receipts` property is a map that stores packet receipts based on
/// the PortId, ChannelId, and seq_on_a. It maps a tuple of `(String, String, u64)` to a `u64` value,
/// where the first two elements of the tuple represent the PortId and Channel Id
///
/// * `last_processed_on`: The `last_processed_on` property is a map that stores last processed block time and height for each client.
///
/// * `callback_data`: Map of reply id to bytes that can be used as context when callback returns.
///
pub struct CwIbcStore<'a> {
    client_registry: Map<'a, IbcClientType, String>,
    client_types: Map<'a, &'a IbcClientId, IbcClientType>,
    client_implementations: Map<'a, &'a IbcClientId, LightClient>,
    next_sequence_send: Map<'a, (&'a PortId, &'a ChannelId), Sequence>,
    next_sequence_recv: Map<'a, (&'a PortId, &'a ChannelId), Sequence>,
    next_sequence_ack: Map<'a, (&'a PortId, &'a ChannelId), Sequence>,
    next_client_sequence: Item<'a, u64>,
    next_connection_sequence: Item<'a, u64>,
    next_channel_sequence: Item<'a, u64>,
    client_connections: Map<'a, &'a IbcClientId, IbcConnectionId>,
    connections: Map<'a, &'a IbcConnectionId, Vec<u8>>,
    channels: Map<'a, (&'a PortId, &'a ChannelId), ChannelEnd>,
    port_to_module: Map<'a, &'a PortId, IbcModuleId>,
    /// Stores address based on the capability names
    capabilities: Map<'a, Vec<u8>, String>,
    /// store commitments based on keys (PacketCommitment,AckCommitment,Connection,Channel,Client)
    commitments: Map<'a, Vec<u8>, Vec<u8>>,
    /// Stores block duration
    expected_time_per_block: Item<'a, u64>,
    /// Stores packet receipts based on PortId,ChannelId and sequence
    packet_receipts: Map<'a, (&'a PortId, &'a ChannelId, u64), u64>,
    last_processed_on: Map<'a, &'a IbcClientId, LastProcessedOn>,
    // Stores data by replyid to be used later on reply from cross contract call
    callback_data: Map<'a, u64, Vec<u8>>,
    sent_packets: Map<'a, (&'a PortId, &'a ChannelId, u64), u64>,
}

impl<'a> Default for CwIbcStore<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcStore<'a> {
    pub fn new() -> Self {
        Self {
            client_registry: Map::new(StorageKey::ClientRegistry.as_str()),
            client_types: Map::new(StorageKey::ClientTypes.as_str()),
            client_implementations: Map::new(StorageKey::ClientImplementations.as_str()),
            next_sequence_send: Map::new(StorageKey::NextSequenceSend.as_str()),
            next_sequence_recv: Map::new(StorageKey::NextSequenceReceive.as_str()),
            next_sequence_ack: Map::new(StorageKey::NextSequenceAcknowledgement.as_str()),
            next_client_sequence: Item::new(StorageKey::NextClientSequence.as_str()),
            next_connection_sequence: Item::new(StorageKey::NextConnectionSequence.as_str()),
            next_channel_sequence: Item::new(StorageKey::NextChannelSequence.as_str()),
            connections: Map::new(StorageKey::Connections.as_str()),
            client_connections: Map::new(StorageKey::ClientConnection.as_str()),
            channels: Map::new(StorageKey::Channels.as_str()),
            port_to_module: Map::new(StorageKey::PortToModule.as_str()),
            capabilities: Map::new(StorageKey::Capabilities.as_str()),
            commitments: Map::new(StorageKey::Commitments.as_str()),
            expected_time_per_block: Item::new(StorageKey::BlockTime.as_str()),
            packet_receipts: Map::new(StorageKey::PacketReceipts.as_str()),
            last_processed_on: Map::new(StorageKey::LastProcessedOn.as_str()),
            callback_data: Map::new(StorageKey::CallbackData.as_str()),
            sent_packets: Map::new(StorageKey::SentPackets.as_str()),
        }
    }
    pub fn client_registry(&self) -> &Map<'a, IbcClientType, String> {
        &self.client_registry
    }
    pub fn client_types(&self) -> &Map<'a, &'a ClientId, IbcClientType> {
        &self.client_types
    }
    pub fn client_implementations(&self) -> &Map<'a, &'a ClientId, LightClient> {
        &self.client_implementations
    }
    pub fn next_sequence_send(&self) -> &Map<'a, (&'a PortId, &'a ChannelId), Sequence> {
        &self.next_sequence_send
    }
    pub fn next_sequence_recv(&self) -> &Map<'a, (&'a PortId, &'a ChannelId), Sequence> {
        &self.next_sequence_recv
    }

    pub fn next_sequence_ack(&self) -> &Map<'a, (&'a PortId, &'a ChannelId), Sequence> {
        &self.next_sequence_ack
    }

    pub fn next_client_sequence(&self) -> &Item<'a, u64> {
        &self.next_client_sequence
    }
    pub fn next_connection_sequence(&self) -> &Item<'a, u64> {
        &self.next_connection_sequence
    }
    pub fn next_channel_sequence(&self) -> &Item<'a, u64> {
        &self.next_channel_sequence
    }
    pub fn connections(&self) -> &Map<'a, &'a ConnectionId, Vec<u8>> {
        &self.connections
    }
    pub fn client_connections(&self) -> &Map<'a, &'a ClientId, ConnectionId> {
        &self.client_connections
    }
    pub fn channels(&self) -> &Map<'a, (&'a PortId, &'a ChannelId), ChannelEnd> {
        &self.channels
    }
    pub fn port_to_module(&self) -> &Map<'a, &'a PortId, IbcModuleId> {
        &self.port_to_module
    }
    pub fn capabilities(&self) -> &Map<'a, Vec<u8>, String> {
        &self.capabilities
    }
    pub fn get_commitments(&self) -> &Map<'a, Vec<u8>, Vec<u8>> {
        &self.commitments
    }
    pub fn expected_time_per_block(&self) -> &Item<'a, u64> {
        &self.expected_time_per_block
    }
    pub fn packet_receipts(&self) -> &Map<'a, (&'a PortId, &'a ChannelId, u64), u64> {
        &self.packet_receipts
    }

    pub fn last_processed_on(&self) -> &Map<'a, &'a IbcClientId, LastProcessedOn> {
        &self.last_processed_on
    }

    // pub fn client_states(&self) -> &Map<'a, &'a IbcClientId, Vec<u8>> {
    //     &self.client_states
    // }

    // pub fn consensus_states(&self) -> &Map<'a, &'a IbcClientId, Vec<u8>> {
    //     &self.consensus_states
    // }

    pub fn callback_data(&self) -> &Map<'a, u64, Vec<u8>> {
        &self.callback_data
    }
    pub fn sent_packets(&self) -> &Map<'a, (&'a PortId, &'a ChannelId, u64), u64> {
        &self.sent_packets
    }

    pub fn clear_storage(&self, store: &mut dyn Storage) {
        let keys: Vec<_> = store
            .range(None, None, Order::Ascending)
            .map(|(k, _)| k)
            .collect();
        for k in keys {
            debug_println!("Removing Key {:?}", k);
            store.remove(&k);
        }
    }

    pub fn store_sent_packet(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        seq: u64,
        height: u64,
    ) -> Result<(), ContractError> {
        self.sent_packets()
            .save(store, (port_id, channel_id, seq), &height)
            .map_err(ContractError::Std)
    }

    pub fn get_packet_heights(
        &self,
        store: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        start_seq: u64,
        end_seq: u64,
    ) -> Result<HashMap<u64, u64>, ContractError> {
        let min_key = (port_id, channel_id, start_seq);
        let max_key = (port_id, channel_id, end_seq);
        let min_bound = Bound::Inclusive::<(&PortId, &ChannelId, u64)>((min_key, PhantomData));
        let max_bound = Bound::Inclusive::<(&PortId, &ChannelId, u64)>((max_key, PhantomData));

        let result: HashMap<u64, u64> = self
            .sent_packets()
            .range(store, Some(min_bound), Some(max_bound), Order::Ascending)
            .filter_map(|p| {
                p.ok().map(|r| {
                    if r.1 == 0 {
                        return None;
                    }
                    Some((r.0 .2, r.1))
                })
            })
            .flatten()
            .collect();

        Ok(result)
    }

    pub fn get_missing_packet_receipts(
        &self,
        store: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        start_seq: u64,
        end_seq: u64,
    ) -> Result<Vec<u64>, ContractError> {
        let mut missing: Vec<u64> = Vec::new();
        for i in start_seq..end_seq {
            let exists = self
                .packet_receipts()
                .load(store, (port_id, channel_id, i))
                .is_ok();
            if !exists {
                missing.push(i)
            }
        }

        Ok(missing)
    }

    pub fn save_commitment(
        &self,
        store: &mut dyn Storage,
        key: Vec<u8>,
        data: &[u8],
    ) -> StdResult<()> {
        return self.get_commitments().save_raw(store, key, data);
    }

    pub fn load_commitment(&self, store: &dyn Storage, key: Vec<u8>) -> Option<Vec<u8>> {
        return self.get_commitments().load_raw(store, key);
    }

    pub fn remove_commitment(&self, store: &mut dyn Storage, key: Vec<u8>) {
        return self.get_commitments().remove(store, key);
    }
}
