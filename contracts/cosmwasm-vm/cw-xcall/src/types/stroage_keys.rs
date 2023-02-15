use std::fmt;

use cosmwasm_schema::cw_serde;

#[cw_serde]

pub enum StorageKey {
    Owner,
    Admin,
    MessageRequest,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::MessageRequest => "message_request",
        }
    }
}
