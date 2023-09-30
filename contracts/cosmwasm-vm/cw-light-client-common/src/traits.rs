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

pub trait ILightClient {
    type Error;
    /**
     * @dev createClient creates a new client with the given state.
     * If succeeded, it returns a commitment for the initial state.
     */
    fn create_client(
        &mut self,
        caller: Addr,
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
        caller: Addr,
        client_id: &str,
        header: SignedHeader,
    ) -> Result<ConsensusStateUpdate, Self::Error>;

    fn verify_header(
        &mut self,
        caller: &Addr,
        client_id: &str,
        header: &SignedHeader,
    ) -> Result<(), Self::Error>;
}

pub trait IQueryHandler {
    fn get_consensus_state(
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

    fn get_client_state(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<ClientState, ContractError> {
        let data = CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(|_e| ContractError::ClientStateNotFound(client_id.to_string()))?;
        let state = ClientState::decode(data.as_slice()).map_err(ContractError::DecodeError)?;
        Ok(state)
    }

    fn get_latest_consensus_state(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<ConsensusState, ContractError> {
        let state = CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(ContractError::Std)?;
        let client_state =
            ClientState::decode(state.as_slice()).map_err(ContractError::DecodeError)?;

        let consensus_state =
            Self::get_consensus_state(storage, client_id, client_state.latest_height)?;
        Ok(consensus_state)
    }

    fn get_timestamp_at_height(
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

    fn get_config(storage: &dyn Storage) -> Result<Config, ContractError> {
        CONFIG
            .load(storage)
            .map_err(|_e| ContractError::ConfigNotFound)
    }

    fn get_client_state_any(
        storage: &dyn Storage,
        client_id: &str,
    ) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_client_state(storage, client_id)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    fn get_consensus_state_any(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_consensus_state(storage, client_id, height)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    fn get_latest_height(storage: &dyn Storage, client_id: &str) -> Result<u64, ContractError> {
        let state = Self::get_client_state(storage, client_id)?;

        Ok(state.latest_height)
    }

    fn get_processed_time_at_height(
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
    fn get_processed_blocknumber_at_height(
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
    fn verify_membership(
        deps: Deps,
        client_id: &str,
        height: u64,
        delay_time_period: u64,
        delay_block_period: u64,
        proof: &[MerkleNode],
        value: &[u8],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        let client_state = Self::get_client_state(deps.storage, client_id)?;
        let consensus_state: ConsensusState =
            Self::get_consensus_state(deps.storage, client_id, height)?;
        Self::verify_membership_inner(
            deps,
            client_id,
            client_state,
            consensus_state,
            height,
            delay_time_period,
            delay_block_period,
            proof,
            value,
            path,
        )
    }

    fn verify_membership_inner(
        deps: Deps,
        client_id: &str,
        client_state: ClientState,
        consensus_state: ConsensusState,
        height: u64,
        _delay_time_period: u64,
        _delay_block_period: u64,
        proof: &[MerkleNode],
        value: &[u8],
        path: &[u8],
    ) -> Result<bool, ContractError> {
        cw_println!(
            deps.api,
            "[LightClient]: Path Bytes  {:?}",
            HexString::from_bytes(path)
        );
        cw_println!(
            deps.api,
            "[LightClient]: Value Bytes  {:?}",
            HexString::from_bytes(value)
        );
        let path = keccak256(path).to_vec();
        cw_println!(deps.api, "[LightClient]: client id is: {:?}", client_id);

        if client_state.frozen_height != 0 && height > client_state.frozen_height {
            return Err(ContractError::ClientStateFrozen(client_state.frozen_height));
        }

        let mut value_hash = value.to_vec();
        if !value.is_empty() {
            value_hash = keccak256(value).to_vec();
        }

        // let _ =
        //     self.validate_delay_args(client_id, height, delay_time_period, delay_block_period)?;

        cw_println!(
            deps.api,
            "[LightClient]: Path Hash {:?}",
            HexString::from_bytes(&path)
        );
        cw_println!(
            deps.api,
            "[LightClient]: Value Hash {:?}",
            HexString::from_bytes(&value_hash)
        );
        let leaf = keccak256(&[path, value_hash].concat());
        cw_println!(
            deps.api,
            "[LightClient]: Leaf Value {:?}",
            HexString::from_bytes(&leaf)
        );

        let message_root = calculate_root(leaf, proof);
        cw_println!(
            deps.api,
            "[LightClient]: Stored Message Root {:?} ",
            hex::encode(consensus_state.message_root.clone())
        );
        cw_println!(
            deps.api,
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
    fn verify_non_membership(
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

    fn get_previous_consensus(
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
pub trait IStoreReader {}
pub trait IContext {
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, ContractError>;

    fn insert_client_state(
        &mut self,
        client_id: &str,
        state: ClientState,
    ) -> Result<(), ContractError>;

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, ContractError>;
    fn insert_consensus_state(
        &mut self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), ContractError>;

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, ContractError>;
    fn insert_timestamp_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError>;
    fn insert_blocknumber_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError>;

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self.api().secp256k1_recover_pubkey(msg, &rs, v).unwrap();
        let pubkey_hash = keccak256(&pubkey[1..]);
        let address: Option<[u8; 20]> = pubkey_hash.as_slice()[12..].try_into().ok();
        address
    }

    fn recover_icon_signer(&self, msg: &[u8], signature: &[u8]) -> Option<Vec<u8>> {
        self.recover_signer(msg, signature)
            .map(|addr| addr.to_vec())
    }

    fn get_config(&self) -> Result<Config, ContractError>;

    fn insert_config(&mut self, config: &Config) -> Result<(), ContractError>;

    fn get_current_block_time(&self) -> u64;
    fn get_current_block_height(&self) -> u64;
    fn get_processed_time_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError>;
    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError>;

    fn ensure_owner(&self, caller: Addr) -> Result<(), ContractError>;
    fn ensure_ibc_host(&self, caller: &Addr) -> Result<(), ContractError>;
    fn api(&self) -> &dyn Api;
}
