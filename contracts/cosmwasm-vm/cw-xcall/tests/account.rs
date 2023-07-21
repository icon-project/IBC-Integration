use std::fmt::Display;

use common::rlp::{Encodable, self, Decodable};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::KeyDeserialize;
#[cw_serde]
pub struct Address(String);

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address(value.to_string())
    }
}

impl From<&String> for Address {
    fn from(value: &String) -> Self {
        Address(value.to_string())
    }
}

impl From<&[u8]> for Address {
    fn from(value: &[u8]) -> Self {
        let address = String::from_vec(value.to_vec()).unwrap();
        Address(address)
    }
}
impl Encodable for Address {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(1).append(&self.0);
    }
}

impl Decodable for Address {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self(rlp.val_at(0)?))
    }
}

impl Address {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn alice() -> Address {
    Address::from("alice")
}

pub fn bob() -> Address {
    Address::from("bob")
}

pub fn admin_one() -> Address {
    Address::from("adminone")
}

pub fn admin_two() -> Address {
    Address::from("admintwo")
}
