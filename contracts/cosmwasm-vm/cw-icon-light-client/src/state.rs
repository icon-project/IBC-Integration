use std::cell::RefCell;

use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::utils::keccak256;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::Storage;
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
const PROCESSED_HEIGHTS: Map<(ClientId, u64), u64> = Map::new("PROCESSED_HEIGHTS");

const CONFIG: Item<Config> = Item::new("CONFIG");

pub struct CwContext<'a> {
    pub deps_mut: RefCell<DepsMut<'a>>,

    pub env: Env,
}

impl<'a> CwContext<'a> {
    pub fn new(deps_mut: RefCell<DepsMut<'a>>, env: Env) -> Self {
        return Self { deps_mut, env };
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
            .map_err(|_e| ContractError::FailedToSaveClientState)
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, Self::Error> {
        return QueryHandler::get_consensus_state(
            self.deps_mut.borrow().storage,
            client_id,
            height,
        );
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
            .map_err(|_e| ContractError::FailedToSaveClientState)
    }

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error> {
        return QueryHandler::get_timestamp_at_height(
            self.deps_mut.borrow().storage,
            client_id,
            height,
        );
    }

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self
            .deps_mut
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
        return QueryHandler::get_config(self.deps_mut.borrow().storage);
    }

    fn insert_config(&self, config: &Config) -> Result<(), Self::Error> {
        return CONFIG
            .save(self.deps_mut.borrow_mut().storage, config)
            .map_err(|_e| ContractError::FailedToSaveConfig);
    }

    fn insert_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<(), Self::Error> {
        let time = self.env.block.time.nanos();
        PROCESSED_TIMES
            .save(
                self.deps_mut.borrow_mut().storage,
                (client_id.to_string(), height),
                &time,
            )
            .map_err(|_e| ContractError::FailedToSaveProcessedTime)
    }

    fn insert_blocknumber_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<(), Self::Error> {
        let block_height = self.env.block.height;
        PROCESSED_HEIGHTS
            .save(
                self.deps_mut.borrow_mut().storage,
                (client_id.to_string(), height),
                &block_height,
            )
            .map_err(|_e| ContractError::FailedToSaveProcessedTime)
    }

    fn get_current_block_time(&self) -> u64 {
        self.env.block.time.nanos()
    }

    fn get_current_block_height(&self) -> u64 {
        self.env.block.height
    }

    fn get_processed_time_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, Self::Error> {
        QueryHandler::get_processed_time_at_height(
            self.deps_mut.borrow().storage,
            client_id,
            height,
        )
    }

    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, Self::Error> {
        QueryHandler::get_processed_blocknumber_at_height(
            self.deps_mut.borrow().storage,
            client_id,
            height,
        )
    }
}

pub struct QueryHandler {}

impl QueryHandler {
    pub fn get_consensus_state(
        storage: &dyn Storage,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, ContractError> {
        CONSENSUS_STATES
            .load(storage, (client_id.to_string(), height))
            .map_err(|_e| ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            })
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
        CLIENT_STATES
            .load(storage, client_id.to_string())
            .map_err(|_e| ContractError::ClientStateNotFound(client_id.to_string()))
    }

    pub fn get_config(storage: &dyn Storage) -> Result<Config, ContractError> {
        return CONFIG
            .load(storage)
            .map_err(|_e| ContractError::ConfigNotFound);
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
                height: height,
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
                height: height,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockStorage},
        StdResult,
    };
    use hex_literal::hex;
    use ibc_proto::google::protobuf::Any;
    use prost::Message;

    #[test]
    fn test_get_consensus_state() {
        let mut storage = MockStorage::new();
        let client_id = "test_client";
        let height = 10;
        let consensus_state = ConsensusState::default();
        CONSENSUS_STATES
            .save(
                &mut storage,
                (client_id.to_string(), height),
                &consensus_state,
            )
            .unwrap();

        let result = QueryHandler::get_consensus_state(&storage, client_id, height).unwrap();
        assert_eq!(result, consensus_state);

        let non_existent_height = height + 1;
        let error = QueryHandler::get_consensus_state(&storage, client_id, non_existent_height)
            .unwrap_err();
        assert_eq!(
            error,
            ContractError::ConsensusStateNotFound {
                client_id: client_id.to_string(),
                height: non_existent_height
            }
        );
    }

    #[test]
    fn test_get_timestamp_at_height() {
        let mut storage = MockStorage::new();
        let client_id = "test_client";
        let height = 10;
        let timestamp = 1619012345;
        PROCESSED_TIMES
            .save(&mut storage, (client_id.to_string(), height), &timestamp)
            .unwrap();

        let result = QueryHandler::get_timestamp_at_height(&storage, client_id, height).unwrap();
        assert_eq!(result, timestamp);

        let non_existent_height = height + 1;
        let error = QueryHandler::get_timestamp_at_height(&storage, client_id, non_existent_height)
            .unwrap_err();
        assert_eq!(
            error,
            ContractError::TimestampNotFound {
                client_id: client_id.to_string(),
                height: non_existent_height
            }
        );
    }

    #[test]
    fn test_get_client_state() {
        let mut storage = MockStorage::new();
        let client_id = "test_client";
        let client_state = ClientState::default();
        CLIENT_STATES
            .save(&mut storage, client_id.to_string(), &client_state)
            .unwrap();

        let result = QueryHandler::get_client_state(&storage, client_id).unwrap();
        assert_eq!(result, client_state);

        let non_existent_client_id = "non_existent_client";
        let error = QueryHandler::get_client_state(&storage, non_existent_client_id).unwrap_err();
        assert_eq!(
            error,
            ContractError::ClientStateNotFound(non_existent_client_id.to_string())
        );
    }

    #[test]
    fn test_get_config() {
        let mut storage = MockStorage::new();
        let config = Config::default();
        CONFIG.save(&mut storage, &config).unwrap();

        let result = QueryHandler::get_config(&storage).unwrap();
        assert_eq!(result, config);

        CONFIG.remove(&mut storage);
        let error = QueryHandler::get_config(&storage).unwrap_err();
        assert_eq!(error, ContractError::ConfigNotFound);
    }

    #[test]
    fn test_get_client_state_any() {
        let mut storage = MockStorage::new();
        let client_id = "test_client";
        let client_state = ClientState::default();
        CLIENT_STATES
            .save(&mut storage, client_id.to_string(), &client_state)
            .unwrap();

        let result = QueryHandler::get_client_state_any(&storage, client_id).unwrap();
        let decoded: Any = Message::decode(result.as_slice()).unwrap();
        let decoded_client_state = ClientState::from_any(decoded).unwrap();
        assert_eq!(client_state, decoded_client_state)
    }

    #[test]
    fn test_cwcontext_get_client_state() {
        let mut deps = mock_dependencies();

        // Store client state
        let client_id = "my-client";
        let client_state = ClientState::default();
        CwContext::new(RefCell::new(deps.as_mut()), mock_env())
            .insert_client_state(&client_id, client_state.clone())
            .unwrap();

        // Retrieve client state
        let context = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        let result = context.get_client_state(&client_id).unwrap();
        assert_eq!(client_state, result);
    }

    #[test]
    fn test_cwcontext_get_consensus_state() {
        let mut deps = mock_dependencies();

        // Store consensus state
        let client_id = "my-client";
        let height = 1;
        let consensus_state = ConsensusState::default();
        CwContext::new(RefCell::new(deps.as_mut()), mock_env())
            .insert_consensus_state(&client_id, height, consensus_state.clone())
            .unwrap();

        // Retrieve consensus state
        let context = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        let result = context.get_consensus_state(&client_id, height).unwrap();
        assert_eq!(consensus_state, result);
    }

    #[test]
    fn test_cwcontext_get_timestamp_at_height() {
        let mut deps = mock_dependencies();

        // Store processed time
        let client_id = "my-client";
        let height = 1;
        let time = 1571797419879305533;
        CwContext::new(RefCell::new(deps.as_mut()), mock_env())
            .insert_timestamp_at_height(&client_id, height)
            .unwrap();

        // Retrieve processed time
        let context = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        let result = context.get_timestamp_at_height(&client_id, height).unwrap();
        assert_eq!(time, result);
    }

    #[test]
    fn test_cwcontext_recover_signer() {
        let mut deps = mock_dependencies();

        // Recover signer
        let msg = keccak256(b"test message");
        let address = hex!("5c42b6096c4601ceabacdb471cb1cdfe6bc46586");
        let signature = hex!("c8b2b5eeb7b54620a0246b2355e42ce6d3bdf1648cd8eae298ebbbe5c3bacc197d5e8bfddb0f1e33778b7fc558c54d35e47c88daa24fff243aa743088e5503d701");
        let context = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        let result = context.recover_signer(msg.as_slice(), &signature);
        assert_eq!(address, result.unwrap());
    }

    #[test]
    fn test_cwcontext_get_config() {
        let mut deps = mock_dependencies();
        let info = mock_info("alice", &[]);
        // Store config
        let config = Config::new("my-config".to_string(), 1, 1, info.sender);
        CONFIG.save(deps.as_mut().storage, &config).unwrap();

        // Retrieve config
        let context = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        let result = context.get_config().unwrap();
        assert_eq!(config, result);
    }

    #[test]
    fn insert_client_state_should_save_state() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let client_id = "client";
        let state = ClientState::default();
        let ctx = CwContext::new(RefCell::new(deps.as_mut()), env);
        ctx.insert_client_state(client_id, state.clone()).unwrap();

        let loaded = CLIENT_STATES.load(deps.as_ref().storage, client_id.to_string())?;
        assert_eq!(state, loaded);
        Ok(())
    }

    #[test]
    fn insert_consensus_state_should_save_state() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let client_id = "client";
        let height = 100;
        let state = ConsensusState::default();
        let ctx = CwContext::new(RefCell::new(deps.as_mut()), env);
        ctx.insert_consensus_state(client_id, height, state.clone())
            .unwrap();

        let loaded =
            CONSENSUS_STATES.load(deps.as_ref().storage, (client_id.to_string(), height))?;
        assert_eq!(state, loaded);
        Ok(())
    }

    #[test]
    fn insert_timestamp_at_height_should_save_time() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let client_id = "client";
        let height = 100;
        let ctx = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        ctx.insert_timestamp_at_height(client_id, height).unwrap();

        let loaded =
            PROCESSED_TIMES.load(deps.as_ref().storage, (client_id.to_string(), height))?;
        assert_eq!(mock_env().block.time.nanos(), loaded);
        Ok(())
    }
    #[test]
    fn insert_blocknumber_at_height_should_save_blocknumber() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let client_id = "client";
        let height = 100;
        let ctx = CwContext::new(RefCell::new(deps.as_mut()), mock_env());
        ctx.insert_blocknumber_at_height(client_id, height).unwrap();

        let loaded =
            PROCESSED_HEIGHTS.load(deps.as_ref().storage, (client_id.to_string(), height))?;
        assert_eq!(mock_env().block.height, loaded);
        Ok(())
    }
}
