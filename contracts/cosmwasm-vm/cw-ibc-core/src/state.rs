use super::*;
pub struct CwIbcStore<'a> {
    client_registry: Map<'a, ClientType, String>,
    client_types: Map<'a, ClientId, ClientType>,
    client_impls: Map<'a, ClientId, String>,
    next_sequence_send: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_recv: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_ack: Map<'a, (PortId, ChannelId), Sequence>,
    next_client_sequence: Item<'a, u128>,
    next_connection_sequence: Item<'a, u128>,
    next_channel_sequence: Item<'a, u128>,
    client_connections: Map<'a, ClientId, ConnectionId>,
    connections: Map<'a, ConnectionId, Vec<u8>>,
    channels: Map<'a, (PortId, ChannelId), ChannelEnd>,
    port_to_moudle: Map<'a, PortId, ModuleId>,
}

impl<'a> Default for CwIbcStore<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcStore<'a> {
    pub fn client_registry(&self) -> &Map<'a, ClientType, String> {
        &self.client_registry
    }
    pub fn client_types(&self) -> &Map<'a, ClientId, ClientType> {
        &self.client_types
    }
    pub fn client_impls(&self) -> &Map<'a, ClientId, String> {
        &self.client_impls
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

    pub fn next_client_sequence(&self) -> &Item<'a, u128> {
        &self.next_client_sequence
    }
    pub fn next_connection_sequence(&self) -> &Item<'a, u128> {
        &self.next_connection_sequence
    }
    pub fn next_channel_sequence(&self) -> &Item<'a, u128> {
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
    pub fn port_to_moulde(&self) -> &Map<'a, PortId, ModuleId> {
        &self.port_to_moudle
    }

    pub fn new() -> Self {
        Self {
            client_registry: Map::new(StorageKey::ClientRegistry.as_str()),
            client_types: Map::new(StorageKey::ClientTypes.as_str()),
            client_impls: Map::new(StorageKey::ClientImpls.as_str()),
            next_sequence_send: Map::new(StorageKey::NextSequenceSend.as_str()),
            next_sequence_recv: Map::new(StorageKey::NextSequenceReceieve.as_str()),
            next_sequence_ack: Map::new(StorageKey::NextSequenceAcknowledgement.as_str()),
            next_client_sequence: Item::new(StorageKey::NextClientSequence.as_str()),
            next_connection_sequence: Item::new(StorageKey::NextConnectionSequence.as_str()),
            next_channel_sequence: Item::new(StorageKey::NextChannelSequence.as_str()),
            connections: Map::new(StorageKey::Connections.as_str()),
            client_connections: Map::new(StorageKey::ClientConnection.as_str()),
            channels: Map::new(StorageKey::Channels.as_str()),
            port_to_moudle: Map::new(StorageKey::PortToModule.as_str()),
        }
    }
}
