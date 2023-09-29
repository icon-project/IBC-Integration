use common::{
    ibc::Height,
    icon::icon::lightclient::v1::{ClientState, ConsensusState},
    traits::AnyTypes,
};
use cosmwasm_std::{Api, Env, Storage};
use cw_common::raw_types::Any;
use cw_light_client_common::{traits::IContext, ContractError};
use ics07_tendermint_cw::ics23::FakeInner;
use ics08_wasm::client_state::ClientState as WasmClientState;
use prost::Message;
use tendermint_proto::Protobuf;
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

pub fn get_consensus_state_key(height: Height) -> Vec<u8> {
    [
        "consensusStates/".to_string().into_bytes(),
        format!("{height}").into_bytes(),
    ]
    .concat()
}

pub fn to_wasm_client_state(
    client_state: ClientState,
    old_state: Vec<u8>,
) -> Result<Vec<u8>, ContractError> {
    use ibc::Height;
    let any = Any::decode(&*old_state).unwrap();
    let mut wasm_client_state =
        WasmClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(&any.value).unwrap();
    wasm_client_state.data = client_state.to_any().encode_to_vec();
    wasm_client_state.latest_height = Height::new(0, client_state.latest_height);
    let vec1 = wasm_client_state.to_any().encode_to_vec();
    Ok(vec1)
}

impl<'a> IContext for CwContext<'a> {
    fn get_client_state(
        &self,
        client_id: &str,
    ) -> Result<
        common::icon::icon::lightclient::v1::ClientState,
        cw_light_client_common::ContractError,
    > {
        let any_bytes = self
            .storage
            .get(&"clientState".to_string().into_bytes())
            .ok_or(ContractError::ClientStateNotFound(client_id.to_string()))?;
        let any_state = Any::decode(any_bytes.as_slice()).unwrap();
        ClientState::from_any(any_state).map_err(ContractError::DecodeError)
    }

    fn insert_client_state(
        &mut self,
        _client_id: &str,
        _state: common::icon::icon::lightclient::v1::ClientState,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn get_consensus_state(
        &self,
        client_id: &str,
        height: u64,
    ) -> Result<
        common::icon::icon::lightclient::v1::ConsensusState,
        cw_light_client_common::ContractError,
    > {
        let ibc_height = Height::new(0, height).unwrap();
        let any_bytes = self
            .storage
            .get(&get_consensus_state_key(ibc_height))
            .ok_or(ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            })?;
        let any_state = Any::decode(any_bytes.as_slice()).unwrap();
        ConsensusState::from_any(any_state).map_err(ContractError::DecodeError)
    }

    fn insert_consensus_state(
        &mut self,
        _client_id: &str,
        _height: u64,
        _state: common::icon::icon::lightclient::v1::ConsensusState,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn get_timestamp_at_height(
        &self,
        _client_id: &str,
        _height: u64,
    ) -> Result<u64, cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn insert_timestamp_at_height(
        &mut self,
        _client_id: &str,
        _height: u64,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn insert_blocknumber_at_height(
        &mut self,
        _client_id: &str,
        _height: u64,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
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
        unimplemented!()
    }

    fn get_current_block_height(&self) -> u64 {
        unimplemented!()
    }

    fn get_processed_time_at_height(
        &self,
        _client_id: &str,
        _height: u64,
    ) -> Result<u64, cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn get_processed_block_at_height(
        &self,
        _client_id: &str,
        _height: u64,
    ) -> Result<u64, cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn ensure_owner(
        &self,
        _caller: cosmwasm_std::Addr,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn ensure_ibc_host(
        &self,
        _caller: cosmwasm_std::Addr,
    ) -> Result<(), cw_light_client_common::ContractError> {
        unimplemented!()
    }

    fn api(&self) -> &dyn Api {
        unimplemented!()
    }
}
