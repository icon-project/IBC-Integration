use std::cell::RefCell;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::utils::keccak256;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::StdResult;
use cw_storage_plus::{Item, Map};

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
    pub deps: RefCell<DepsMut<'a>>,
    pub env: Env,
}

impl<'a> CwContext<'a> {
    pub fn new(deps: RefCell<DepsMut<'a>>, env: Env) -> Self {
        Self { deps, env }
    }
}

impl<'a> IContext for CwContext<'a> {
    type Error = ContractError;
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, Self::Error> {
        CLIENT_STATES
            .load(self.deps.borrow().storage, client_id.to_string())
            .map_err(|e| ContractError::ClientStateNotFound(client_id.to_string()))
    }

    fn insert_client_state(&self, client_id: &str, state: ClientState) -> Result<(), Self::Error> {
        CLIENT_STATES
            .save(
                self.deps.borrow_mut().storage,
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
        CONSENSUS_STATES
            .load(self.deps.borrow().storage, (client_id.to_string(), height))
            .map_err(|e| ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }

    fn insert_consensus_state(
        &self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), Self::Error> {
        CONSENSUS_STATES
            .save(
                self.deps.borrow_mut().storage,
                (client_id.to_string(), height),
                &state,
            )
            .map_err(|e| ContractError::FailedToSaveClientState)
    }

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error> {
        PROCESSED_TIMES
            .load(self.deps.borrow().storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::TimestampNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self
            .deps
            .borrow()
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
        return CONFIG
            .load(self.deps.borrow().storage)
            .map_err(|e| ContractError::ConfigNotFound);
    }

    fn insert_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<(), Self::Error> {
        let time = self.env.block.time.nanos();
        PROCESSED_TIMES
            .save(
                self.deps.borrow_mut().storage,
                (client_id.to_string(), height),
                &time,
            )
            .map_err(|e| ContractError::FailedToSaveProcessedTime)
    }
}
