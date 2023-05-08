use super::*;

#[cw_serde]
pub enum StorageKey {
    SequenceNo,
    RequestNo,
    Owner,
    Admin,
    MessageRequest,
    IbcConfig,
    FeeHandler,
    Balance,
    Fee,
    IbcHost,
    TimeoutHeight,
    XCallHost,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::MessageRequest => "message_request",
            StorageKey::SequenceNo => "sequenceno",
            StorageKey::RequestNo => "requestno",
            StorageKey::IbcConfig => "ibcconfig",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::Fee => "fee",
            StorageKey::IbcHost => "ibc_host",
            StorageKey::TimeoutHeight => "timeout_height",
            StorageKey::XCallHost => "xcall_host",
        }
    }
}
