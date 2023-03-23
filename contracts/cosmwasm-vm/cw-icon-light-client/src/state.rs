use std::cell::RefCell;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::utils::keccak256;
use cosmwasm_std::Storage;
use cosmwasm_std::Deps;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::StdResult;
use cw_storage_plus::{Item, Map};
use prost::Message;

use crate::traits::AnyTypes;
use crate::traits::Config;
use crate::traits::IContext;
use crate::ContractError;
type ClientId = String;
const CLIENT_STATES: Map<String, ClientState> = Map::new("CLIENT_STATES");
const CONSENSUS_STATES: Map<(ClientId, u64), ConsensusState> = Map::new("CONSENSUS_STATES");
const PROCESSED_TIMES: Map<(ClientId, u64), u64> = Map::new("PROCESSED_TIMES");
const PROCESSED_HEIGHTS: Map<(ClientId, u64), u128> = Map::new("PROCESSED_HEIGHTS");

const CONFIG: Item<Config> = Item::new("CONFIG");


pub struct CwContext<'a> {
    pub deps_mut: RefCell<DepsMut<'a>>,
    
    pub env: Env,
}

impl<'a> CwContext<'a> {
    pub fn new(deps_mut: RefCell<DepsMut<'a>>, env: Env) -> Self {
      
        return Self { deps_mut, env}
       
    }
}

impl<'a> IContext for CwContext<'a> {
    type Error = ContractError;
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, Self::Error> {
        QueryHandler::get_client_state(self.deps_mut.borrow().storage, client_id)
    }

    fn insert_client_state(&self, client_id: &str, state: ClientState) -> Result<(), Self::Error> {

        CLIENT_STATES
        .save(
            self.deps_mut.borrow_mut().storage,
            client_id.to_string(),
            &state,
        )
        .map_err(|e| ContractError::FailedToSaveClientState)
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, Self::Error> {
        return QueryHandler::get_consensus_state(self.deps_mut.borrow().storage, client_id, height)
    }

    fn insert_consensus_state(
        &self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), Self::Error> {
        CONSENSUS_STATES
            .save(
                self.deps_mut.borrow_mut().storage,
                (client_id.to_string(), height),
                &state,
            )
            .map_err(|e| ContractError::FailedToSaveClientState)
    }

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error> {
        return QueryHandler::get_timestamp_at_height(self.deps_mut.borrow().storage, client_id, height);
    }

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self
            .deps_mut.borrow()
            .api
            .secp256k1_recover_pubkey(msg, &rs, v)
            .unwrap();
        let pubkey_hash = keccak256(&pubkey);
        let address: Option<[u8; 20]> = pubkey_hash.as_slice()[12..]
            .try_into()
            .ok()
            .map(|arr: [u8; 20]| arr.into());
        address
    }

    fn get_config(&self) -> Result<Config, Self::Error> {
        return QueryHandler::get_config(self.deps_mut.borrow().storage);
    }

    fn insert_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<(), Self::Error> {
        let time = self.env.block.time.nanos();
        PROCESSED_TIMES
            .save(
                self.deps_mut.borrow_mut().storage,
                (client_id.to_string(), height),
                &time,
            )
            .map_err(|e| ContractError::FailedToSaveProcessedTime)
    }
}


pub struct QueryHandler{
   
}


impl QueryHandler{
    
   
    pub fn get_consensus_state(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, ContractError> {
        CONSENSUS_STATES
            .load(storage, (client_id.to_string(), height))
            .map_err(|e| ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }

   pub fn get_timestamp_at_height(storage: &dyn Storage, client_id: &str, height: u64) -> Result<u64,ContractError> {
        PROCESSED_TIMES
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::TimestampNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }

    pub fn get_client_state(storage: &dyn Storage, client_id: &str) -> Result<ClientState, ContractError> {
        CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(|e| ContractError::ClientStateNotFound(client_id.to_string()))
    }

    pub fn get_config(storage: &dyn Storage) -> Result<Config, ContractError> {
        return CONFIG
            .load(storage)
            .map_err(|e| ContractError::ConfigNotFound);
    }

    pub fn get_client_state_any(storage: &dyn Storage, client_id: &str) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_client_state(storage,client_id)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    pub fn get_consensus_state_any(storage: &dyn Storage, client_id: &str, height: u64) -> Result<Vec<u8>, ContractError> {
        let state = Self::get_consensus_state(storage,client_id, height)?;
        let any_state = state.to_any();
        Ok(any_state.encode_to_vec())
    }

    pub fn get_latest_height(storage: &dyn Storage, client_id: &str) -> Result<u64,ContractError> {
        let state = Self::get_client_state(storage,client_id)?;

        Ok(state.latest_height)
    }

}


