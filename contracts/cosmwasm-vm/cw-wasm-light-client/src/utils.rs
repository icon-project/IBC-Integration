use common::{
    consensus_state::IConsensusState,
    icon::icon::lightclient::v1::{ClientState, ConsensusState},
    traits::AnyTypes,
};
use cw_common::raw_types::Any;
use cw_light_client_common::ContractError;
use ibc::Height;
use ics07_tendermint_cw::ics23::FakeInner;
use ics08_wasm::client_state::ClientState as WasmClientState;
use prost::Message;
use tendermint_proto::Protobuf;
use ics08_wasm::client_message::Header as WasmHeader;
use common::icon::icon::types::v1::SignedHeader;
pub fn get_consensus_state_key(height: Height) -> Vec<u8> {
    [
        "consensusStates/".to_string().into_bytes(),
        format!("{height}").into_bytes(),
    ]
    .concat()
}

pub fn get_client_state_key() -> Vec<u8> {
    "clientState".to_string().into_bytes()
}

pub fn to_wasm_client_state(
    client_state: ClientState,
    old_wasm_state: Vec<u8>,
) -> Result<Vec<u8>, ContractError> {
    let any = any_from_byte(&old_wasm_state)?;
    let mut wasm_client_state = WasmClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
        &any.value,
    )
    .map_err(|e| ContractError::OtherError {
        error: e.to_string(),
    })?;
    wasm_client_state.data = client_state.to_any().encode_to_vec();
    wasm_client_state.latest_height = to_ibc_height(client_state.latest_height);
    let vec1 = wasm_client_state.to_any().encode_to_vec();
    Ok(vec1)
}

pub fn to_wasm_consensus_state(consensus_state: ConsensusState) -> Vec<u8> {
    let wasm_consensus_state = ics08_wasm::consensus_state::ConsensusState {
        data: consensus_state.to_any().encode_to_vec(),
        timestamp: consensus_state.timestamp().nanoseconds(),
        inner: Box::new(FakeInner),
    };
    wasm_consensus_state.to_any().encode_to_vec()
}

pub fn decode_client_state(data: &[u8]) -> Result<ClientState, ContractError> {
    let any = Any::decode(data).map_err(ContractError::DecodeError)?;
    let wasm_state =
        ics08_wasm::client_state::ClientState::<FakeInner, FakeInner, FakeInner>::decode_vec(
            &any.value,
        )
        .map_err(|e| ContractError::OtherError {
            error: e.to_string(),
        })?;
    let any = Any::decode(&*wasm_state.data).map_err(ContractError::DecodeError)?;
    let state = ClientState::from_any(any).map_err(ContractError::DecodeError)?;
    Ok(state)
}

pub fn decode_consensus_state(value: &[u8]) -> Result<ConsensusState, ContractError> {
    let any = Any::decode(&mut &*value).map_err(ContractError::DecodeError)?;
    let wasm_consensus_state =
        ics08_wasm::consensus_state::ConsensusState::<FakeInner>::decode_vec(&any.value).map_err(
            |e| ContractError::OtherError {
                error: e.to_string(),
            },
        )?;
    let any =
        Any::decode(&mut &wasm_consensus_state.data[..]).map_err(ContractError::DecodeError)?;
    let any_consensus_state = ConsensusState::from_any(any).map_err(ContractError::DecodeError)?;
    Ok(any_consensus_state)
}

pub fn to_ibc_height(height: u64) -> Height {
    Height::new(0, height)
}

pub fn any_from_byte(bytes: &[u8]) -> Result<Any, ContractError> {
    let any = Any::decode(bytes).map_err(ContractError::DecodeError)?;
    Ok(any)
}

pub fn to_wasm_header(signed_header:&SignedHeader)->WasmHeader<FakeInner>{
    let header_any: Any = signed_header.to_any();
    let block_height = signed_header.header.clone().unwrap().main_height;
    let wasm_header = WasmHeader::<FakeInner> {
        inner: Box::new(FakeInner),
        data: header_any.encode_to_vec(),
        height: to_ibc_height(block_height),
    };
    wasm_header
}
