use cosmwasm_std::Binary;
use super::*;

#[cw_serde]
pub struct FeeHandler {
    address: Address,
}

impl FeeHandler {
    pub fn new(address: Address) -> Self {
        Self { address }
    }
    pub fn address(&self) -> &Address {
        &self.address
    }
}
