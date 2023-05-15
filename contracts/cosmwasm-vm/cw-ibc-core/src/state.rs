use super::*;

/// The `CwIbcStore` struct stores various data related to the Inter-Blockchain Communication (IBC).
///
/// Properties:
///
/// * `client_registry`: A mapping between client types and their corresponding client identifiers
/// (IDs).
/// * `client_types`: A mapping between a client ID and its corresponding client type.
/// * `client_implementations`: `client_implementations` is a mapping between `ClientId` and a string
/// representing the implementation of the client. In the context of the Cosmos SDK and the IBC
/// protocol, a client is a module that is responsible for verifying the validity of the state of a
/// remote blockchain. The implementation of a
/// * `next_sequence_send`: `next_sequence_send` is a mapping between a tuple of `(PortId, ChannelId)`
/// and a `Sequence` number. It stores the next sequence number that should be used when sending a
/// packet on the given channel. This is used to ensure that packets are sent in order and to prevent
/// replay
/// * `next_sequence_recv`: `next_sequence_recv` is a mapping between a tuple of `(PortId, ChannelId)`
/// and a `Sequence` value. It stores the next expected sequence number for a packet to be received on a
/// particular channel. This is used to ensure that packets are received in the correct order and to
/// detect
/// * `next_sequence_ack`: `next_sequence_ack` is a mapping between a tuple of `(PortId, ChannelId)` and
/// a `Sequence` value. It stores the next expected sequence number for an acknowledgement message to be
/// received on a particular channel. This is used to ensure that acknowledgement messages are received
/// in the correct order and
/// * `next_client_sequence`: `next_client_sequence` is an `Item` that stores the next available
/// sequence number for creating a new client. It is likely used to ensure that each new client created
/// has a unique sequence number.
/// * `next_connection_sequence`: `next_connection_sequence` is an `Item` that stores the next available
/// sequence number for creating a new connection. It is used to ensure that each new connection has a
/// unique identifier.
/// * `next_channel_sequence`: `next_channel_sequence` is an `Item` that stores the next available
/// sequence number for a channel. It is used to ensure that each channel has a unique sequence number
/// when it is created.
/// * `client_connections`: A mapping between a client ID and its associated connection ID. This is used
/// to keep track of the connection associated with each client.
/// * `connections`: `connections` is a mapping between `ConnectionId` and a byte vector (`Vec<u8>`). It
/// stores the connection state associated with each connection identifier. This state can include
/// information such as the connection version, the connection status, and any associated metadata.
/// * `channels`: The `channels` property is a map that stores the channel end information for each
/// channel identified by a tuple of `(PortId, ChannelId)`. The `ChannelEnd` struct contains information
/// such as the channel state, ordering, and counterparty channel information. This map is used to keep
/// track of
/// * `port_to_module`: `port_to_module` is a mapping between `PortId` and `IbcModuleId`. It stores the
/// module identifier for each port. This is useful for routing packets between different modules in the
/// IBC protocol.
/// * `capabilities`: The `capabilities` property is a map that stores addresses based on capability
/// names. In the context of the Cosmos SDK and IBC (Inter-Blockchain Communication) protocol,
/// capabilities are used to grant permissions to modules to perform certain actions. This map allows
/// for easy lookup of addresses associated with specific capabilities.
/// * `commitments`: The `commitments` property is a map that stores commitments based on keys. The keys
/// can be PacketCommitment, AckCommitment, Connection, Channel, or Client. The values are byte arrays
/// that represent the commitments. This map is used to keep track of the commitments made during the
/// IBC
/// * `expected_time_per_block`: The expected time duration of a block in the blockchain network. This
/// is used to calculate the timeout for certain operations in the IBC protocol.
/// * `packet_receipts`: The `packet_receipts` property is a map that stores packet receipts based on
/// the PortId, ChannelId, and Sequence. It maps a tuple of `(String, String, u64)` to a `u64` value,
/// where the first two elements of the tuple represent the PortId and
pub struct CwIbcStore<'a> {
    client_registry: Map<'a, ClientType, String>,
    client_types: Map<'a, ClientId, ClientType>,
    client_implementations: Map<'a, ClientId, String>,
    next_sequence_send: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_recv: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_ack: Map<'a, (PortId, ChannelId), Sequence>,
    next_client_sequence: Item<'a, u64>,
    next_connection_sequence: Item<'a, u64>,
    next_channel_sequence: Item<'a, u64>,
    client_connections: Map<'a, ClientId, ConnectionId>,
    connections: Map<'a, ConnectionId, Vec<u8>>,
    channels: Map<'a, (PortId, ChannelId), ChannelEnd>,
    port_to_module: Map<'a, PortId, IbcModuleId>,
    /// Stores address based on the capability names
    capabilities: Map<'a, Vec<u8>, Vec<String>>,
    /// store commitments based on keys (PacketCommitment,AckCommitment,Connection,Channel,Client)
    commitments: Map<'a, Vec<u8>, Vec<u8>>,
    /// Stores block duration
    expected_time_per_block: Item<'a, u64>,
    /// Stores packet receipts based on PortId,ChannelId and Sequence
    packet_receipts: Map<'a, (String, String, u64), u64>,
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
        }
    }
    pub fn client_registry(&self) -> &Map<'a, ClientType, String> {
        &self.client_registry
    }
    pub fn client_types(&self) -> &Map<'a, ClientId, ClientType> {
        &self.client_types
    }
    pub fn client_implementations(&self) -> &Map<'a, ClientId, String> {
        &self.client_implementations
    }
    pub fn next_sequence_send(&self) -> &Map<'a, (PortId, ChannelId), Sequence> {
        &self.next_sequence_send
    }
    pub fn next_sequence_recv(&self) -> &Map<'a, (PortId, ChannelId), Sequence> {
        &self.next_sequence_recv
    }

    pub fn next_sequence_ack(&self) -> &Map<'a, (PortId, ChannelId), Sequence> {
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
    pub fn connections(&self) -> &Map<'a, ConnectionId, Vec<u8>> {
        &self.connections
    }
    pub fn client_connections(&self) -> &Map<'a, ClientId, ConnectionId> {
        &self.client_connections
    }
    pub fn channels(&self) -> &Map<'a, (PortId, ChannelId), ChannelEnd> {
        &self.channels
    }
    pub fn port_to_module(&self) -> &Map<'a, PortId, IbcModuleId> {
        &self.port_to_module
    }
    pub fn capabilities(&self) -> &Map<'a, Vec<u8>, Vec<String>> {
        &self.capabilities
    }
    pub fn commitments(&self) -> &Map<'a, Vec<u8>, Vec<u8>> {
        &self.commitments
    }
    pub fn expected_time_per_block(&self) -> &Item<'a, u64> {
        &self.expected_time_per_block
    }
    pub fn packet_receipts(&self) -> &Map<'a, (String, String, u64), u64> {
        &self.packet_receipts
    }
}
