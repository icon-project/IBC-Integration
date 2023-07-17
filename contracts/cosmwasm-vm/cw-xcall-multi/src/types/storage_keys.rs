use super::*;

#[cw_serde]
pub enum StorageKey {
    Sn,
    RequestNo,
    Owner,
    Admin,
    MessageRequest,
    Requests,
    FeeHandler,
    Balance,
    ProtocolFee,
    DefaultConnections,
    Connections,
    PendingRequests,
    PendingResponses,
    SuccessfulResponses,
    Config,
    ExecuteReqId,
    ExecuteRollbackId,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::MessageRequest => "message_request",
            StorageKey::Sn => "sn",
            StorageKey::RequestNo => "requestno",
            StorageKey::Requests => "requests",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::ProtocolFee => "protocol_fee",
            StorageKey::DefaultConnections => "default_connections",
            StorageKey::Connections => "connections",
            StorageKey::PendingRequests => "pending_requests",
            StorageKey::PendingResponses => "pending_responses",
            StorageKey::PendingResponses => "pending_responses",
            StorageKey::SuccessfulResponses => "successful_responses",
            StorageKey::Config => "config",
            StorageKey::ExecuteReqId => "execute_request_id",
            StorageKey::ExecuteRollbackId => "execute_rollback_id",
        }
    }
}
