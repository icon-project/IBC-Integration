use cosmwasm_std::{Api, Env, Storage};

use crate::query_handler::QueryHandler;
use cw_light_client_common::traits::IQueryHandler;
use cw_light_client_common::{
    constants::{PROCESSED_HEIGHTS, PROCESSED_TIMES},
    traits::IContext,
    ContractError,
};

use crate::utils::{
    get_client_state_key, get_consensus_state_key, to_ibc_height, to_wasm_client_state,
    to_wasm_consensus_state,
};
pub struct CwContext<'a> {
    pub storage: &'a mut dyn Storage,
    pub api: &'a dyn Api,
    pub env: Env,
}

impl<'a> CwContext<'a> {
    pub fn new(deps_mut: cosmwasm_std::DepsMut<'a>, env: Env) -> Self {
        Self {
            storage: deps_mut.storage,
            api: deps_mut.api,
            env,
        }
    }
}

impl<'a> IContext for CwContext<'a> {
    fn get_client_state(
        &self,
        client_id: &str,
    ) -> Result<
        common::icon::icon::lightclient::v1::ClientState,
        cw_light_client_common::ContractError,
    > {
        QueryHandler::get_client_state(self.storage, client_id)
    }

    fn insert_client_state(
        &mut self,
        client_id: &str,
        client_state: common::icon::icon::lightclient::v1::ClientState,
    ) -> Result<(), cw_light_client_common::ContractError> {
        let old_state = self
            .storage
            .get(&get_client_state_key())
            .ok_or(ContractError::ClientStateNotFound(client_id.to_string()))?;
        let new_state = to_wasm_client_state(client_state, old_state)?;
        self.storage.set(&get_client_state_key(), &new_state);
        Ok(())
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<
        common::icon::icon::lightclient::v1::ConsensusState,
        cw_light_client_common::ContractError,
    > {
        QueryHandler::get_consensus_state(self.storage, client_id, height)
    }

    fn insert_consensus_state(
        &mut self,
        _client_id: &str,
        height: u64,
        consensus_state: common::icon::icon::lightclient::v1::ConsensusState,
    ) -> Result<(), cw_light_client_common::ContractError> {
        let ibc_height = to_ibc_height(height);
        let wasm_consensus_state = to_wasm_consensus_state(consensus_state);
        self.storage
            .set(&get_consensus_state_key(ibc_height), &wasm_consensus_state);
        Ok(())
    }

    fn get_timestamp_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, cw_light_client_common::ContractError> {
        QueryHandler::get_processed_time_at_height(self.storage, client_id, height)
    }

    fn insert_timestamp_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError> {
        let time = self.env.block.time.nanos();
        PROCESSED_TIMES
            .save(self.storage, (client_id.to_string(), height), &time)
            .map_err(|_e| ContractError::FailedToSaveProcessedTime)
    }

    fn insert_blocknumber_at_height(
        &mut self,
        client_id: &str,
        height: u64,
    ) -> Result<(), ContractError> {
        let block_height = self.env.block.height;
        PROCESSED_HEIGHTS
            .save(self.storage, (client_id.to_string(), height), &block_height)
            .map_err(|_e| ContractError::FailedToSaveProcessedTime)
    }

    fn get_config(
        &self,
    ) -> Result<cw_light_client_common::traits::Config, cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn insert_config(
        &mut self,
        _config: &cw_light_client_common::traits::Config,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
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
    ) -> Result<u64, ContractError> {
        QueryHandler::get_processed_time_at_height(self.storage, client_id, height)
    }

    fn get_processed_block_at_height(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<u64, ContractError> {
        QueryHandler::get_processed_blocknumber_at_height(self.storage, client_id, height)
    }

    fn ensure_owner(
        &self,
        _caller: cosmwasm_std::Addr,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn ensure_ibc_host(
        &self,
        _caller: &cosmwasm_std::Addr,
    ) -> Result<(), cw_light_client_common::ContractError> {
        Ok(())
    }

    fn api(&self) -> &dyn Api {
        self.api
    }
}
