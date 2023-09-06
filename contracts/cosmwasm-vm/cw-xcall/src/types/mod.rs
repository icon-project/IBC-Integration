pub mod call_request;
pub mod config;
pub mod message;
pub mod request;
pub mod response;
pub mod storage_keys;

pub const LOG_PREFIX: &str = "[xcall_app]:";

use crate::error::ContractError;
pub use common::rlp;
use common::rlp::{Decodable, Encodable};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary};
use request::CSMessageRequest;
use response::CSMessageResponse;
