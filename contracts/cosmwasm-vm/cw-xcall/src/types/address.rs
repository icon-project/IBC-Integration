use super::*;

#[cw_serde]
#[derive(Hash, Eq)]
pub struct Address(String);

impl Address {
    pub fn from_string(str: String) -> Address {
        Address(str)
    }
    pub fn from_bytes(address: &[u8]) -> Result<Address, StdError> {
        let address = String::from_vec(address.to_vec())?;
        Ok(Address(address))
    }
    pub fn new(adr: String) -> Self {
        Address(adr)
    }
}
