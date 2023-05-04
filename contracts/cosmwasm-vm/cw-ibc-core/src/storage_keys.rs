use super::*;

#[cw_serde]

/// This is defining an enumeration called `StorageKey` with 21 possible values. Each value represents a
/// key that can be used to access a specific piece of data in a storage system. The `as_str` method is
/// also defined to convert each value to its corresponding string representation. This code is likely
/// part of a larger system that uses a key-value store to persist data.
pub enum StorageKey {
    ClientRegistry,
    ClientTypes,
    ClientImplementations,
    NextSequenceSend,
    NextSequenceReceive,
    NextSequenceAcknowledgement,
    NextClientSequence,
    NextConnectionSequence,
    NextChannelSequence,
    Connections,
    ClientConnection,
    Channels,
    Router,
    PortToModule,
    Commitments,
    BlockTime,
    BlockHeight,
    Capabilities,
    PacketReceipts,
    Owner,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::ClientRegistry => "client_registry",
            StorageKey::ClientTypes => "client_types",
            StorageKey::ClientImplementations => "client_implementations",
            StorageKey::NextSequenceSend => "next_sequence_send",
            StorageKey::NextSequenceReceive => "next_sequence_recv",
            StorageKey::NextSequenceAcknowledgement => "next_sequence_ack",
            StorageKey::NextClientSequence => "next_client_sequence",
            StorageKey::NextConnectionSequence => "next_connection_sequence",
            StorageKey::NextChannelSequence => "next_channel_sequence",
            StorageKey::Connections => "connections",
            StorageKey::ClientConnection => "client_connections",
            StorageKey::Channels => "channels",
            StorageKey::Router => "router",
            StorageKey::PortToModule => "port_to_module",
            StorageKey::Commitments => "commitments",
            StorageKey::BlockTime => "block_time",
            StorageKey::BlockHeight => "block_height",
            StorageKey::Capabilities => "capabilities",
            StorageKey::PacketReceipts => "packet_receipts",
            StorageKey::Owner => "owner",
        }
    }
}
