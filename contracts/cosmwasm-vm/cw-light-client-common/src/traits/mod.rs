pub mod trait_context;
pub mod trait_light_client;
pub mod trait_query_handler;

pub use trait_context::IContext;
pub use trait_light_client::ILightClient;
pub use trait_query_handler::IQueryHandler;

use std::marker::PhantomData;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;

use common::icon::icon::types::v1::MerkleNode;
use common::icon::icon::types::v1::SignedHeader;
use common::utils::calculate_root;
use common::utils::keccak256;
use cosmwasm_std::Addr;

use cosmwasm_std::Api;

use cosmwasm_std::Deps;
use cosmwasm_std::Order;
use cosmwasm_std::StdResult;
use cosmwasm_std::Storage;
use cw_common::cw_println;
use cw_common::hex_string::HexString;
use cw_storage_plus::Bound;
use serde::Deserialize;
use serde::Serialize;

use crate::constants::CLIENT_STATES;
use crate::constants::CONFIG;
use crate::constants::CONSENSUS_STATES;
use crate::constants::PROCESSED_HEIGHTS;
use crate::constants::PROCESSED_TIMES;
use crate::ContractError;
use common::traits::AnyTypes;
use prost::Message;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusStateUpdate {
    // commitment for updated consensusState
    pub consensus_state_commitment: [u8; 32],
    pub client_state_commitment: [u8; 32],
    pub consensus_state_bytes: Vec<u8>,
    pub client_state_bytes: Vec<u8>,
    // updated height
    pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    pub owner: Addr,
    pub ibc_host: Addr,
}

impl Config {
    pub fn new(owner: Addr, ibc_host: Addr) -> Self {
        Self { owner, ibc_host }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            owner: Addr::unchecked("test"),
            ibc_host: Addr::unchecked("ibc_host"),
        }
    }
}


