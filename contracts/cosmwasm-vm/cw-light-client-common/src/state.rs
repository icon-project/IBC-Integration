use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;

use common::utils::keccak256;
use cosmwasm_std::Api;

use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::Storage;

use cw_storage_plus::{Item, Map};
use debug_print::debug_eprintln;

use prost::Message;

use crate::query_handler::QueryHandler;
use crate::traits::Config;
use crate::traits::IContext;
use crate::ContractError;
type ClientId = String;
pub const CLIENT_STATES: Map<String, Vec<u8>> = Map::new("CLIENT_STATES");
pub const CONSENSUS_STATES: Map<(ClientId, u64), Vec<u8>> = Map::new("CONSENSUS_STATES");
pub const PROCESSED_TIMES: Map<(ClientId, u64), u64> = Map::new("PROCESSED_TIMES");
pub const PROCESSED_HEIGHTS: Map<(ClientId, u64), u64> = Map::new("PROCESSED_HEIGHTS");

pub const CONFIG: Item<Config> = Item::new("CONFIG");

pub struct CwContext<'a> {
    pub storage: &'a mut dyn Storage,
    pub api: &'a dyn Api,
    pub env: Env,
}

impl<'a> CwContext<'a> {
    pub fn new(deps_mut: DepsMut<'a>, env: Env) -> Self {
        Self {
            storage: deps_mut.storage,
            api: deps_mut.api,
            env,
        }
    }
}

impl<'a> IContext for CwContext<'a> {
    type Error = ContractError;
    fn get_client_state(&self, client_id: &str) -> Result<ClientState, Self::Error> {
        QueryHandler::get_client_state(self.storage, client_id)
    }

    fn insert_client_state(
        &mut self,
        client_id: &str,
        state: ClientState,
    ) -> Result<(), Self::Error> {
        let data = state.encode_to_vec();
        CLIENT_STATES
            .save(self.storage, client_id.to_string(), &data)
            .map_err(|_e| ContractError::FailedToSaveClientState)
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<ConsensusState, Self::Error> {
        QueryHandler::get_consensus_state(self.storage, client_id, height)
    }

    fn insert_consensus_state(
        &mut self,
        client_id: &str,
        height: u64,
        state: ConsensusState,
    ) -> Result<(), Self::Error> {
        let data = state.encode_to_vec();
        CONSENSUS_STATES
            .save(self.storage, (client_id.to_string(), height), &data)
            .map_err(|_e| ContractError::FailedToSaveClientState)
    }

    fn get_timestamp_at_height(&self, client_id: &str, height: u64) -> Result<u64, Self::Error> {
        QueryHandler::get_timestamp_at_height(self.storage, client_id, height)
    }

    fn recover_signer(&self, msg: &[u8], signature: &[u8]) -> Option<[u8; 20]> {
        if signature.len() != 65 {
            return None;
        }
        let mut rs = [0u8; 64];
        rs[..].copy_from_slice(&signature[..64]);
        let v = signature[64];
        let pubkey = self.api.secp256k1_recover_pubkey(msg, &rs, v).unwrap();
        let pubkey_hash = keccak256(&pubkey[1..]);
        let address: Option<[u8; 20]> = pubkey_hash.as_slice()[12..].try_into().ok();
        address
    }

    fn recover_icon_signer(&self, msg: &[u8], signature: &[u8]) -> Option<Vec<u8>> {
        self.recover_signer(msg, signature)
            .map(|addr| addr.to_vec())
    }

    fn get_config(&self) -> Result<Config, Self::Error> {
        QueryHandler::get_config(self.storage)
    }

    fn insert_config(&mut self, config: &Config) -> Result<(), Self::Error> {
        CONFIG
            .save(self.storage, config)
            .map_err(|_e| ContractError::FailedToSaveConfig)
    }

    fn insert_timestamp_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), Self::Error> {
        let time = self.env.block.time.nanos();
        PROCESSED_TIMES
            .save(self.storage, (client_id.to_string(), height), &time)
            .map_err(|_e| ContractError::FailedToSaveProcessedTime)
    }

    fn insert_blocknumber_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), Self::Error> {
        let block_height = self.env.block.height;
        PROCESSED_HEIGHTS
            .save(self.storage, (client_id.to_string(), height), &block_height)
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
        QueryHandler::get_processed_time_at_height(self.storage, client_id, height)
    }

    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, Self::Error> {
        QueryHandler::get_processed_blocknumber_at_height(self.storage, client_id, height)
    }

    fn ensure_ibc_host(&self, caller: cosmwasm_std::Addr) -> Result<(), Self::Error> {
        let config = self.get_config()?;
        if caller != config.ibc_host {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }
    fn ensure_owner(&self, caller: cosmwasm_std::Addr) -> Result<(), Self::Error> {
        let config = self.get_config()?;
        debug_eprintln!("owner {:?} caller {}", config.owner, caller.to_string());
        if caller != config.owner {
            return Err(ContractError::Unauthorized {});
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use common::{
        constants::{DEFAULT_NETWORK_TYPE_ID, DEFAULT_SRC_NETWORK_ID},
        icon::icon::types::v1::SignedHeader,
        traits::AnyTypes,
    };
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info, MockStorage},
        Addr, StdResult,
    };
    use cw_common::raw_types::Any;
    use hex_literal::hex;
    use prost::Message;

    use test_utils::get_test_signed_headers;

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
                &consensus_state.encode_to_vec(),
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
            .save(
                &mut storage,
                client_id.to_string(),
                &client_state.encode_to_vec(),
            )
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
            .save(
                &mut storage,
                client_id.to_string(),
                &client_state.encode_to_vec(),
            )
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
        CwContext::new(deps.as_mut(), mock_env())
            .insert_client_state(client_id, client_state.clone())
            .unwrap();

        // Retrieve client state
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context.get_client_state(client_id).unwrap();
        assert_eq!(client_state, result);
    }

    #[test]
    fn test_cwcontext_get_consensus_state() {
        let mut deps = mock_dependencies();

        // Store consensus state
        let client_id = "my-client";
        let height = 1;
        let consensus_state = ConsensusState::default();
        CwContext::new(deps.as_mut(), mock_env())
            .insert_consensus_state(client_id, height, consensus_state.clone())
            .unwrap();

        // Retrieve consensus state
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context.get_consensus_state(client_id, height).unwrap();
        assert_eq!(consensus_state, result);
    }

    #[test]
    fn test_cwcontext_get_timestamp_at_height() {
        let mut deps = mock_dependencies();

        // Store processed time
        let client_id = "my-client";
        let height = 1;
        let time = 1571797419879305533;
        CwContext::new(deps.as_mut(), mock_env())
            .insert_timestamp_at_height(client_id, height)
            .unwrap();

        // Retrieve processed time
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context.get_timestamp_at_height(client_id, height).unwrap();
        assert_eq!(time, result);
    }

    #[test]
    fn test_cwcontext_recover_signer() {
        let mut deps = mock_dependencies();

        // Recover signer
        let msg = keccak256(b"test message");
        let address = "8efcaf2c4ebbf88bf07f3bb44a2869c4c675ad7a";
        let signature = hex!("c8b2b5eeb7b54620a0246b2355e42ce6d3bdf1648cd8eae298ebbbe5c3bacc197d5e8bfddb0f1e33778b7fc558c54d35e47c88daa24fff243aa743088e5503d701");
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context.recover_signer(msg.as_slice(), &signature);
        assert_eq!(address, hex::encode(result.unwrap()));
    }

    #[test]
    fn test_cwcontext_recover_signer_relay_data() {
        let mut deps = mock_dependencies();

        let signed_header: SignedHeader = get_test_signed_headers()[0].clone();
        let btp_header = signed_header.header.clone().unwrap();

        let network_type_section_rlp = btp_header.get_network_type_section_rlp();
        assert_eq!("f842a0d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4a074463d2395972061ca8807d262b0757454ed160bf43bc98d4d7a713647891a0a",hex::encode(network_type_section_rlp));

        let network_type_section_hash = btp_header.get_network_type_section_hash();
        assert_eq!(
            "f9a63040c595b934bc78921cd9b56c5d085df8c7cb79c0f7473f083b7c7c8684",
            hex::encode(network_type_section_hash)
        );

        let msg = btp_header.get_network_type_section_decision_hash(
            DEFAULT_SRC_NETWORK_ID,
            DEFAULT_NETWORK_TYPE_ID,
        );
        let address = "b040bff300eee91f7665ac8dcf89eb0871015306";
        let signature = signed_header.signatures[0].clone();
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context
            .recover_icon_signer(msg.as_slice(), &signature)
            .unwrap();
        assert_eq!(address, hex::encode(result));
    }

    #[test]
    fn test_cwcontext_signed_relay_data() {
        let mut deps = mock_dependencies();

        let signed_headers: Vec<SignedHeader> = get_test_signed_headers();
        for signed_header in signed_headers.into_iter() {
            let btp_header = signed_header.header.clone().unwrap();

            let msg = btp_header.get_network_type_section_decision_hash(
                DEFAULT_SRC_NETWORK_ID,
                DEFAULT_NETWORK_TYPE_ID,
            );
            let address = "b040bff300eee91f7665ac8dcf89eb0871015306";
            let signature = signed_header.signatures[0].clone();
            let context = CwContext::new(deps.as_mut(), mock_env());
            let result = context
                .recover_icon_signer(msg.as_slice(), &signature)
                .unwrap();
            assert_eq!(address, hex::encode(result));
        }
    }

    #[test]
    fn test_cwcontext_get_config() {
        let mut deps = mock_dependencies();
        let _info = mock_info("alice", &[]);
        // Store config
        let config = Config::new(Addr::unchecked("owner"), Addr::unchecked("alice"));
        CONFIG.save(deps.as_mut().storage, &config).unwrap();

        // Retrieve config
        let context = CwContext::new(deps.as_mut(), mock_env());
        let result = context.get_config().unwrap();
        assert_eq!(config, result);
    }

    #[test]
    fn insert_client_state_should_save_state() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let client_id = "client";
        let state = ClientState::default();
        let mut ctx = CwContext::new(deps.as_mut(), env);
        ctx.insert_client_state(client_id, state.clone()).unwrap();

        let loaded = CLIENT_STATES.load(deps.as_ref().storage, client_id.to_string())?;
        assert_eq!(state, ClientState::decode(loaded.as_slice()).unwrap());
        Ok(())
    }

    #[test]
    fn insert_consensus_state_should_save_state() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let client_id = "client";
        let height = 100;
        let state = ConsensusState::default();
        let mut ctx = CwContext::new(deps.as_mut(), env);
        ctx.insert_consensus_state(client_id, height, state.clone())
            .unwrap();

        let loaded =
            CONSENSUS_STATES.load(deps.as_ref().storage, (client_id.to_string(), height))?;
        assert_eq!(state.encode_to_vec(), loaded);
        Ok(())
    }

    #[test]
    fn insert_timestamp_at_height_should_save_time() -> StdResult<()> {
        let mut deps = mock_dependencies();
        let client_id = "client";
        let height = 100;
        let mut ctx = CwContext::new(deps.as_mut(), mock_env());
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
        let mut ctx = CwContext::new(deps.as_mut(), mock_env());
        ctx.insert_blocknumber_at_height(client_id, height).unwrap();

        let loaded =
            PROCESSED_HEIGHTS.load(deps.as_ref().storage, (client_id.to_string(), height))?;
        assert_eq!(mock_env().block.height, loaded);
        Ok(())
    }
}
