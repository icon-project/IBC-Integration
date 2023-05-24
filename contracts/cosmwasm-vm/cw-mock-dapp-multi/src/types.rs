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
pub enum ExecuteMsg {
    SendCallMessage {
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    },
    HandleCallMessage {
        from: String,
        data: Vec<u8>,
    },
    XCallMessage {
        data: Vec<u8>,
    },
    SuccessCall {},
    FailureCall {},
    TestCall {
        success_addr: String,
        fail_addr: String,
    },
}

#[cw_serde]
pub struct RollbackData {
    pub id: u64,
    pub rollback: Vec<u8>,
}
