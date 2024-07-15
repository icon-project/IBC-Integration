pub mod context;
pub mod light_client;
pub mod query_handler;

pub use context::IContext;
pub use light_client::ILightClient;
pub use query_handler::IQueryHandler;

use cosmwasm_std::Addr;

use serde::Deserialize;
use serde::Serialize;

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
