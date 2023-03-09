use super::*;

#[cw_serde]

pub enum StorageKey {
    SequenceNo,
    RequestNo,
    Owner,
    Admin,
    MessageRequest,
    Requests,
    IbcConfig,
    FeeHandler,
    Balance,
    Fee,
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
            StorageKey::IbcConfig => "ibcconfig",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::Fee => "fee",
        }
    }
}
