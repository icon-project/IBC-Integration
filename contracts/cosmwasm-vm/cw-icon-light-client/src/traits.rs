use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::utils::keccak256;
use ibc_proto::{google::protobuf::Any, ibc::core::client::v1::Height};
use prost::{DecodeError, Message};

#[derive(Debug, Clone)]
pub struct ConsensusStateUpdate {
    // commitment for updated consensusState
    pub consensus_state_commitment: [u8; 32],
    // updated height
    pub height: u64,
}

pub trait ILightClient {
    type Error;
    /**
     * @dev createClient creates a new client with the given state.
     * If succeeded, it returns a commitment for the initial state.
     */
    fn create_client(
        &self,
        client_id: &str,
        client_state_bytes: Any,
        consensus_state_bytes: Any,
    ) -> Result<(Vec<u8>, ConsensusStateUpdate), Self::Error>;

    /**
     * @dev getTimestampAtHeight returns the timestamp of the consensus state at the given height.
     */
    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error>;

    /**
     * @dev getLatestHeight returns the latest height of the client state corresponding to `clientId`.
     */
    fn get_latest_height(&self, client_id: &str) -> Result<u64, Self::Error>;

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
        &self,
        client_id: &str,
        header: Any,
    ) -> Result<(Vec<u8>, Vec<ConsensusStateUpdate>, bool), Self::Error>;

    /**
     * @dev verifyMembership is a generic proof verification method which verifies a proof of the existence of a value at a given CommitmentPath at the specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    fn verify_membership(
        &self,
        client_id: &str,
        height: &Height,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[u8],
        prefix: &[u8],
        path: &[u8],
        value: &[u8],
    ) -> Result<bool, Self::Error>;

    /**
     * @dev verifyNonMembership is a generic proof verification method which verifies the absence of a given CommitmentPath at a specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    fn verify_non_membership(
        &self,
        client_id: &str,
        height: &Height,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[u8],
        prefix: &[u8],
        path: &[u8],
    ) -> Result<bool, Self::Error>;

    /**
     * @dev getClientState returns the clientState corresponding to `clientId`.
     *      If it's not found, the function returns false.
     */
    fn get_client_state(&self, client_id: &str) -> Result<Vec<u8>, Self::Error>;

    fn get_consensus_state(&self, client_id: &str, height: u64) -> Result<Vec<u8>, Self::Error>;
}

pub trait IContext {
    type Error;
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, Self::Error>;
    fn insert_client_state(&self, client_id: &str, state: ClientState) -> Result<(), Self::Error>;

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, Self::Error>;
    fn insert_consensus_state(
        &self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), Self::Error>;

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error>;

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]>;
}

pub trait AnyTypes: Message + Default {
    fn get_type_url() -> String;

    fn get_type_url_hash() -> [u8; 32] {
        keccak256(Self::get_type_url().as_bytes())
    }

    fn from_any(any: Any) -> Result<Self, DecodeError> {
        if Self::get_type_url_hash() != keccak256(any.type_url.as_bytes()) {
            return Err(DecodeError::new("Invalid typ"));
        }
        Self::decode(any.value.as_slice())
    }

    fn to_any(&self) -> Any {
        return Any {
            type_url: Self::get_type_url(),
            value: self.encode_to_vec(),
        };
    }
}
