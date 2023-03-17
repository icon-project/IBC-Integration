use super::*;

#[cw_serde]

pub enum StorageKey {
    ClientRegistry,
    ClientTypes,
    ClientImpls,
    NextSequenceSend,
    NextSequenceReceieve,
    NextSequenceAcknowledgement,
    NextClientSequence,
    NextConnectionSequence,
    NextChannelSequence,
    Connections,
    ClientConnection,
    Channels,
    Router,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::ClientRegistry => "client_registry",
            StorageKey::ClientTypes => "client_types",
            StorageKey::ClientImpls => "client_impls",
            StorageKey::NextSequenceSend => "next_sequence_send",
            StorageKey::NextSequenceReceieve => "next_sequence_recv",
            StorageKey::NextSequenceAcknowledgement => "next_sequence_ack",
            StorageKey::NextClientSequence => "next_client_sequence",
            StorageKey::NextConnectionSequence => "next_connection_sequence",
            StorageKey::NextChannelSequence => "next_channel_sequence",
            StorageKey::Connections => "connections",
            StorageKey::ClientConnection => "client_connections",
            StorageKey::Channels => "channels",
            StorageKey::Router => "router",
        }
    }
}
