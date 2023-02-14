use std::collections::HashSet;

use cosmwasm_schema::cw_serde;

use super::address::Address;

#[cw_serde]
pub struct Admins(HashSet<Address>);

impl Default for Admins {
    fn default() -> Self {
        Self::new()
    }
}

impl Admins {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn add(&mut self, address: Address) {
        self.0.insert(address);
    }
    pub fn remove(&mut self, address: &Address) {
        self.0.remove(&address);
    }

    pub fn contains(&self, address: &Address) -> bool {
        self.0.contains(address)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
