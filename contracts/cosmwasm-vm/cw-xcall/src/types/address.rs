use std::fmt::Display;

use common::rlp::Encodable;

use super::*;

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
