pub mod client_msg;
pub mod client_response;
pub mod commitment;
pub mod constants;
pub mod core_msg;
pub mod cw_types;
pub mod errors;
pub mod hex_string;
pub mod ibc_types;
pub mod raw_types;
pub mod types;
pub mod xcall_app_msg;
pub mod xcall_connection_msg;
pub mod xcall_msg;
pub mod xcall_payloads;
pub mod query_helpers;
use cosmwasm_std::{from_binary, Binary, StdError};
use serde::de::DeserializeOwned;

pub use prost::Message as ProstMessage;

pub fn from_binary_response<T: DeserializeOwned>(res: &[u8]) -> Result<T, StdError> {
    let start = 0x7b;
    let start_index = res.iter().position(|&x| x == start).unwrap_or(0);
    let slice = &res[(start_index)..(res.len())];
    from_binary::<T>(&Binary(slice.to_vec()))
}
