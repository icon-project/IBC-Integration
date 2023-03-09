use super::address::Address;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::to_binary;

#[cw_serde]
pub struct CallRequest {
    from: Address,
    to: String,
    rollback: Vec<u8>,
    enabled: bool,
}

impl CallRequest {
    pub fn new(from: Address, to: String, rollback: Vec<u8>, enabled: bool) -> Self {
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

    pub fn is_null(&self) -> bool {
        let r = to_binary(self).unwrap();
        r.is_empty()
    }
    pub fn set_enabled(&mut self) {
        self.enabled = true;
    }
}
