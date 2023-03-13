
use super::*;

pub struct IbcStore<'a> {
    client_registry: Map<'a, ClientType, String>,
    client_types: Map<'a, ClientId, ClientType>,
    client_impls: Map<'a, ClientId, String>,
    next_sequence_send: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_recv: Map<'a, (PortId, ChannelId), Sequence>,
    next_sequence_ack: Map<'a, (PortId, ChannelId), Sequence>,
    next_client_sequence: Item<'a, u128>,
    next_connection_sequence: Item<'a, u128>,
    next_channel_sequence: Item<'a, u128>,
    connections : Map<'a, ConnectionId , ConnectionEnd> ,
}


impl<'a> IbcStore<'a> {
    pub fn client_registry(&self) -> &Map<'a, ClientType, String> {
        &self.client_registry
    }
    pub fn client_types(&self) -> &Map<'a, ClientId, ClientType> {
        &self.client_types
    }
    pub fn client_impls(&self) -> &Map<'a, ClientId, String> {
        &&self.client_impls
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
    pub fn connections(&self) -> &Map<'a, ConnectionId, ConnectionEnd>{
        &self.connections
    }
}
