use super::address::Address;
use cosmwasm_std::Binary;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CallRequest {
    from: Address,
    to: String,
    rollback: Binary,
    enabled: bool,
}

impl CallRequest {
    pub fn new(from: Address, to: String, rollback: Binary, enabled: bool) -> Self {
        Self {
            from,
            to,
            rollback,
            enabled,
        }
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> &String {
        &self.to
    }

    pub fn rollback(&self) -> &[u8] {
        &self.rollback
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}
