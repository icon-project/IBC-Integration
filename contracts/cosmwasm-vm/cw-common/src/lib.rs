pub mod client_msg;
pub mod client_response;
pub mod commitment;
pub mod constants;
pub mod core_msg;
pub mod cw_types;
pub mod errors;
pub mod hex_string;
pub mod ibc_types;
pub mod query_helpers;
pub mod raw_types;
pub mod types;
pub mod xcall_app_msg;
pub mod xcall_connection_msg;
pub mod xcall_msg;
pub mod xcall_payloads;

use bech32::FromBase32;
use cosmwasm_std::{from_binary, Addr, Binary, Deps, StdError};
use serde::de::DeserializeOwned;

pub use prost::Message as ProstMessage;

pub fn from_binary_response<T: DeserializeOwned>(res: &[u8]) -> Result<T, StdError> {
    let start = 0x7b;
    let start_index = res.iter().position(|&x| x == start).unwrap_or(0);
    let slice = &res[(start_index)..(res.len())];
    from_binary::<T>(&Binary(slice.to_vec()))
}

pub fn to_checked_address(deps: Deps, address: &str) -> Addr {
    #[cfg(feature = "test")]
    return Addr::unchecked(address);
    #[cfg(not(feature = "test"))]
    deps.api.addr_validate(address).unwrap()
}

pub fn decode_bech32(addr: &str) -> Vec<u8> {
    println!("Addr Received: {addr}");
    if addr.contains("contract") {
        return addr.as_bytes().to_vec();
    }
    let (_hrp, data, _variant) = bech32::decode(addr).unwrap();

    Vec::<u8>::from_base32(&data).unwrap()
}

pub fn get_address_storage_prefix(addr: &str, storage_key: &str) -> Vec<u8> {
    let prefix = format!(
        "03{}{}",
        hex::encode(decode_bech32(addr)),
        hex::encode(prefix_length_in_big_endian(storage_key.as_bytes().to_vec()))
    );
    prefix.as_bytes().to_vec()
}

fn prefix_length_in_big_endian(input: Vec<u8>) -> Vec<u8> {
    let length = input.len();

    // manually convert the length to a 2-byte array in big endian format
    let length_prefix = vec![((length >> 8) & 0xFF) as u8, (length & 0xFF) as u8];

    // prefix the length to the input array
    let mut result = length_prefix;
    result.extend(input);

    result
}

#[cfg(test)]
mod tests {
    use crate::prefix_length_in_big_endian;

    #[test]
    fn test_fixed_16() {
        let len = prefix_length_in_big_endian("commitments".as_bytes().to_vec());
        assert_eq!("000b636f6d6d69746d656e7473", hex::encode(len));
    }
}
