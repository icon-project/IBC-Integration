use super::*;

#[cw_serde]
pub struct InstantiateMsg {
    pub relayer: String,
    pub xcall_address: String,
    pub denom: String,
}

#[cw_serde]
pub enum StorageKey {
    MessageFee,
    ResponseFee,
    Receipts,
    XCall,
    Admin,
    ConnSn,
    Denom,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::MessageFee => "message_fee",
            StorageKey::ResponseFee => "response_fee",
            StorageKey::Receipts => "receipts",
            StorageKey::XCall => "xcall",
            StorageKey::Admin => "admin",
            StorageKey::ConnSn => "conn_sn",
            StorageKey::Denom => "denom",
        }
    }
}
