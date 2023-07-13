use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::icon::icon::types::v1::MerkleNode;
use common::icon::icon::types::v1::SignedHeader;
use cosmwasm_std::Addr;
use cw_common::hex_string::HexString;
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
}

impl Config {
    pub fn new(owner: Addr) -> Self {
        Self { owner }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            owner: Addr::unchecked("test"),
        }
    }
}

pub trait ILightClient {
    type Error;
    /**
     * @dev createClient creates a new client with the given state.
     * If succeeded, it returns a commitment for the initial state.
     */
    fn create_client(
        &mut self,
        client_id: &str,
        client_state: ClientState,
        consensus_state: ConsensusState,
    ) -> Result<ConsensusStateUpdate, Self::Error>;

    /**
     * @dev updateClient updates the client corresponding to `clientId`.
     * If succeeded, it returns a commitment for the updated state.
     * If there are no updates for consensus state, this function should returns an empty array as `updates`.
     *
     * NOTE: updateClient is intended to perform the followings:
     * 1. verify a given client message(e.g. header)
     * 2. check misbehaviour such like duplicate block height
     * 3. if misbehaviour is found, update state accordingly and return
     * 4. update state(s) with the client message
     * 5. persist the state(s) on the host
     */
    fn update_client(
        &mut self,
        client_id: &str,
        header: SignedHeader,
    ) -> Result<ConsensusStateUpdate, Self::Error>;

    /**
     * @dev verifyMembership is a generic proof verification method which verifies a proof of the existence of a value at a given CommitmentPath at the specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    fn verify_membership(
        &self,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &Vec<MerkleNode>,
        value: &[u8],
        path: &[u8],
    ) -> Result<bool, Self::Error>;

    /**
     * @dev verifyNonMembership is a generic proof verification method which verifies the absence of a given CommitmentPath at a specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    fn verify_non_membership(
        &self,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &Vec<MerkleNode>,
        path: &[u8],
    ) -> Result<bool, Self::Error>;
}

pub trait IStoreReader {}
pub trait IContext {
    type Error;

    fn get_client_state(&self, client_id: &str) -> Result<ClientState, Self::Error>;
    fn insert_client_state(
        &mut self,
        client_id: &str,
        state: ClientState,
    ) -> Result<(), Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, Self::Error>;
    fn insert_consensus_state(
        &mut self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), Self::Error>;

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error>;
    fn insert_timestamp_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), Self::Error>;
    fn insert_blocknumber_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), Self::Error>;

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]>;
    fn recover_icon_signer(&self, msg: &[u8], signature: &[u8]) -> Option<Vec<u8>>;

    fn get_config(&self) -> Result<Config, Self::Error>;

    fn insert_config(&mut self, config: &Config) -> Result<(), Self::Error>;

    fn get_current_block_time(&self) -> u64;
    fn get_current_block_height(&self) -> u64;
    fn get_processed_time_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, Self::Error>;
    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, Self::Error>;
}
