use std::marker::PhantomData;

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
use cosmwasm_std::{Deps, Order, StdResult, Storage};
use cw_common::{cw_println, hex_string::HexString};
use cw_storage_plus::Bound;

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

    pub fn get_latest_consensus_state(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<ConsensusState, ContractError> {
        let state = CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(ContractError::Std)?;
        let client_state =
            ClientState::decode(state.as_slice()).map_err(ContractError::DecodeError)?;

        let consensus_state =
            QueryHandler::get_consensus_state(storage, client_id, client_state.latest_height)?;
        Ok(consensus_state)
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
        deps: Deps,
        client_id: &str,
        height: u64,
        _delay_time_period: u64,
        _delay_block_period: u64,
        proof: &[MerkleNode],
        value: &[u8],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        cw_println!(
            deps,
            "[LightClient]: Path Bytes  {:?}",
            HexString::from_bytes(path)
        );
        cw_println!(
            deps,
            "[LightClient]: Value Bytes  {:?}",
            HexString::from_bytes(value)
        );
        let path = keccak256(path).to_vec();
        cw_println!(deps, "[LightClient]: client id is: {:?}", client_id);

        let state = Self::get_client_state(deps.storage, client_id)?;

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
            Self::get_consensus_state(deps.storage, client_id, height)?;
        cw_println!(
            deps,
            "[LightClient]: Path Hash {:?}",
            HexString::from_bytes(&path)
        );
        cw_println!(
            deps,
            "[LightClient]: Value Hash {:?}",
            HexString::from_bytes(&value_hash)
        );
        let leaf = keccak256(&[path, value_hash].concat());
        cw_println!(
            deps,
            "[LightClient]: Leaf Value {:?}",
            HexString::from_bytes(&leaf)
        );

        let message_root = calculate_root(leaf, proof);
        cw_println!(
            deps,
            "[LightClient]: Stored Message Root {:?} ",
            hex::encode(consensus_state.message_root.clone())
        );
        cw_println!(
            deps,
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
        deps: Deps,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[MerkleNode],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        Self::verify_membership(
            deps,
            client_id,
            height,
            delay_time_period,
            delay_block_period,
            proof,
            &[],
            path,
        )
    }

    pub fn get_previous_consensus(
        storage: &dyn Storage,
        height: u64,
        client_id: String,
    ) -> Result<Vec<u64>, ContractError> {
        let key = (client_id, height);
        let bound = Bound::Exclusive::<(String, u64)>((key, PhantomData));

        let result = CONSENSUS_STATES
            .range(storage, None, Some(bound), Order::Descending)
            .take(1)
            .collect::<StdResult<Vec<((String, u64), Vec<u8>)>>>()
            .map_err(ContractError::Std)?;

        let keys = result.into_iter().map(|t| t.0 .1).collect::<Vec<u64>>();
        Ok(keys)
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::MockStorage;

    use crate::state::CONSENSUS_STATES;

    use super::QueryHandler;

    #[test]
    fn test_previous_consensus() {
        let mut store = MockStorage::new();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 100), &vec![1, 2, 4, 5])
            .unwrap();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 80), &vec![1, 2, 4, 5])
            .unwrap();
        CONSENSUS_STATES
            .save(&mut store, ("test".to_string(), 70), &vec![1, 2, 4, 5])
            .unwrap();

        let result = QueryHandler::get_previous_consensus(&store, 110, "test".to_string()).unwrap();

        println!("{result:?}");
    }
}
