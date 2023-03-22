use std::cell::RefCell;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use cosmwasm_std::DepsMut;
use cosmwasm_std::StdResult;
use cw_storage_plus::{Item, Map};

use crate::traits::IContext;
use crate::ContractError;
type ClientId = String;
const CLIENT_STATES: Map<String, ClientState> = Map::new("CLIENT_STATES");
const CONSENSUS_STATES: Map<(ClientId, u128), ConsensusState> = Map::new("CONSENSUS_STATES");
const PROCESSED_TIMES: Map<(ClientId, u128), u64> = Map::new("PROCESSED_TIMES");
const PROCESSED_HEIGHTS: Map<(ClientId, u128), u128> = Map::new("PROCESSED_HEIGHTS");

pub struct CwContext<'a> {
    pub deps: RefCell<DepsMut<'a>>,
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
        height: u128,
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
        height: u128,
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

    fn get_timestamp_at_height(&self, client_id: &str, height: u128) -> Result<u64, Self::Error> {
        PROCESSED_TIMES
            .load(self.deps.borrow().storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::TimestampNotFound {
                height,
                client_id: client_id.to_string(),
            })
    }
}
