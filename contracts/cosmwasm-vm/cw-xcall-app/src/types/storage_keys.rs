use super::*;

#[cw_serde]
pub enum StorageKey {
    SequenceNo,
    RequestNo,
    Owner,
    Admin,
    MessageRequest,
    Requests,
    FeeHandler,
    Balance,
    Fee,
    ConnectionHost,
    Connections,
    TimeoutHeight,
    PendingRequests,
    PendingResponses,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::MessageRequest => "message_request",
            StorageKey::SequenceNo => "sequenceno",
            StorageKey::RequestNo => "requestno",
            StorageKey::Requests => "requests",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::Fee => "fee",
            StorageKey::ConnectionHost => "connection_host",
            StorageKey::Connections => "connections",
            StorageKey::TimeoutHeight => "timeout_height",
            StorageKey::PendingRequests => "pending_requests",
            StorageKey::PendingResponses => "pending_responses",
        }
    }
}
