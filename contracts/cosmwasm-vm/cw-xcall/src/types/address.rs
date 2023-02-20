use std::fmt::Display;

use super::*;

#[cw_serde]
pub struct Address(String);

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Address {
    pub fn from_str(str: &str) -> Address {
        Address(str.to_string())
    }
    pub fn from_bytes(address: &[u8]) -> Result<Address, StdError> {
        let address = String::from_vec(address.to_vec())?;
        Ok(Address(address))
    }
    pub fn new(adr: String) -> Self {
        Address(adr)
    }
}
