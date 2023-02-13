use super::*;

#[cw_serde]
pub struct Address(String);

impl Address {
    pub fn from_bytes(address: &[u8]) -> Result<Address, StdError> {
        let address = String::from_vec(address.to_vec())?;
        Ok(Address(address))
    }
}
