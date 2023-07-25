use crate::{
    state::{CLIENT_STATES, CONFIG, CONSENSUS_STATES, PROCESSED_HEIGHTS, PROCESSED_TIMES},
    traits::Config,
    ContractError,
};
use common::{
    icon::icon::{
        lightclient::v1::{ClientState, ConsensusState},
        types::v1::MerkleNode,
    },
    traits::AnyTypes,
    utils::{calculate_root, keccak256},
};
use cosmwasm_std::Storage;
use cw_common::hex_string::HexString;
use debug_print::debug_println;
use prost::Message;

pub struct QueryHandler {}

impl QueryHandler {
    pub fn get_consensus_state(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, ContractError> {
        let data = CONSENSUS_STATES
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            })?;
        let state = ConsensusState::decode(data.as_slice()).map_err(ContractError::DecodeError)?;
        Ok(state)
    }

    pub fn get_timestamp_at_height(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError> {
        PROCESSED_TIMES
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::TimestampNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }

    pub fn get_client_state(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<ClientState, ContractError> {
        let data = CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(|_e| ContractError::ClientStateNotFound(client_id.to_string()))?;
        let state = ClientState::decode(data.as_slice()).map_err(ContractError::DecodeError)?;
        Ok(state)
    }

    pub fn get_config(storage: &dyn Storage) -> Result<Config, ContractError> {
        CONFIG
            .load(storage)
            .map_err(|_e| ContractError::ConfigNotFound)
    }

    pub fn get_client_state_any(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_client_state(storage, client_id)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    pub fn get_consensus_state_any(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_consensus_state(storage, client_id, height)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    pub fn get_latest_height(storage: &dyn Storage, client_id: &str) -> Result<u64, ContractError> {
        let state = Self::get_client_state(storage, client_id)?;

        Ok(state.latest_height)
    }

    pub fn get_processed_time_at_height(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError> {
        PROCESSED_TIMES
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::ProcessedTimeNotFound {
                client_id: client_id.to_string(),
                height,
            })
    }
    pub fn get_processed_blocknumber_at_height(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError> {
        PROCESSED_HEIGHTS
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::ProcessedHeightNotFound {
                client_id: client_id.to_string(),
                height,
            })
    }

    /**
     * @dev verifyMembership is a generic proof verification method which verifies a proof of the existence of a value at a given CommitmentPath at the specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    pub fn verify_membership(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
        _delay_time_period: u64,
        _delay_block_period: u64,
        proof: &[MerkleNode],
        value: &[u8],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        debug_println!(
            "[LightClient]: Path Bytes  {:?}",
            HexString::from_bytes(path)
        );
        debug_println!(
            "[LightClient]: Value Bytes  {:?}",
            HexString::from_bytes(value)
        );
        let path = keccak256(path).to_vec();
        debug_println!("[LightClient]: client id is: {:?}", client_id);

        let state = Self::get_client_state(storage, client_id)?;

        if state.frozen_height != 0 && height > state.frozen_height {
            return Err(ContractError::ClientStateFrozen(state.frozen_height));
        }

        let mut value_hash = value.to_vec();
        if !value.is_empty() {
            value_hash = keccak256(value).to_vec();
        }

        // let _ =
        //     self.validate_delay_args(client_id, height, delay_time_period, delay_block_period)?;
        let consensus_state: ConsensusState =
            Self::get_consensus_state(storage, client_id, height)?;
        debug_println!(
            "[LightClient]: Path Hash {:?}",
            HexString::from_bytes(&path)
        );
        debug_println!(
            "[LightClient]: Value Hash {:?}",
            HexString::from_bytes(&value_hash)
        );
        let leaf = keccak256(&[path, value_hash].concat());
        debug_println!(
            "[LightClient]: Leaf Value {:?}",
            HexString::from_bytes(&leaf)
        );

        let message_root = calculate_root(leaf, proof);
        debug_println!(
            "[LightClient]: Stored Message Root {:?} ",
            hex::encode(consensus_state.message_root.clone())
        );
        debug_println!(
            "[LightClient]: Calculated Message Root : {:?}",
            HexString::from_bytes(&message_root)
        );
        if consensus_state.message_root != message_root {
            return Err(ContractError::InvalidMessageRoot(hex::encode(message_root)));
        }

        Ok(true)
    }

    /**
     * @dev verifyNonMembership is a generic proof verification method which verifies the absence of a given CommitmentPath at a specified height.
     * The caller is expected to construct the full CommitmentPath from a CommitmentPrefix and a standardized path (as defined in ICS 24).
     */
    pub fn verify_non_membership(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[MerkleNode],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        Self::verify_membership(
            storage,
            client_id,
            height,
            delay_time_period,
            delay_block_period,
            proof,
            &[],
            path,
        )
    }
}
