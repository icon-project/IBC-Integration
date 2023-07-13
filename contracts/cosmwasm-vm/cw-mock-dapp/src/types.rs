use super::*;

#[cw_serde]
pub struct InstantiateMsg {
    pub address: String,
}

#[cw_serde]
pub enum StorageKey {
    SequenceNo,
    Address,
    Request,
    RollBack,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Address => "admin",
            StorageKey::Request => "message_request",
            StorageKey::SequenceNo => "sequenceno",
            StorageKey::RollBack => "rollback",
        }
    }
}

#[cw_serde]
pub struct RollbackData {
    pub id: u64,
    pub rollback: Vec<u8>,
}
