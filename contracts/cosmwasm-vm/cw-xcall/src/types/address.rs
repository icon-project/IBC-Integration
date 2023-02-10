use std::error::Error;

use cosmwasm_std::StdError;
use cw_storage_plus::KeyDeserialize;

pub struct Address(String);

impl Address {
    pub fn from_bytes(address: &[u8]) -> Result<Address, StdError> {
        let address = String::from_vec(address.to_vec())?;
        Ok(Address(address))
    }
}
