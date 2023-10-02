use cosmwasm_std::{to_binary, Binary, Deps, Order, StdResult};
use cw_light_client_common::{constants::PROCESSED_HEIGHTS, traits::IQueryHandler, ContractError};
use ibc::Height;

use crate::{
    constants::CLIENT_ID,
    msg::{GenesisMetadata, QueryResponse},
    utils::{
        decode_client_state, decode_consensus_state, get_client_state_key, get_consensus_state_key,
        to_ibc_height,
    },
};

pub struct QueryHandler;

impl QueryHandler {
    pub fn processed_time_key(height: &Height, prefix: &mut Vec<u8>) -> Vec<u8> {
        prefix.append(&mut "consensusStates/".to_string().into_bytes());
        prefix.append(&mut format!("{height}").into_bytes());
        prefix.append(&mut "/processedTime".to_string().into_bytes());
        prefix.clone()
    }

    pub fn processed_height_key(height: &Height, prefix: &mut Vec<u8>) -> Vec<u8> {
        prefix.append(&mut "consensusStates/".to_string().into_bytes());
        prefix.append(&mut format!("{height}").into_bytes());
        prefix.append(&mut "/processedHeight".to_string().into_bytes());
        prefix.clone()
    }
    pub fn get_genesis_metadata(
        storage: &dyn cosmwasm_std::Storage,
        client_id: &str,
    ) -> Result<Vec<GenesisMetadata>, ContractError> {
        let heights = PROCESSED_HEIGHTS
            .prefix(client_id.to_string())
            .keys(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<u64>>>()
            .unwrap();
        let mut gm: Vec<GenesisMetadata> = Vec::<GenesisMetadata>::new();
        for h in heights {
            let processed_height =
                Self::get_processed_blocknumber_at_height(storage, client_id, h)?;
            let processed_time = Self::get_processed_time_at_height(storage, client_id, h)?;
            let ibc_height = to_ibc_height(h);
            let processed_height_key = Self::processed_height_key(&ibc_height, &mut Vec::new());
            let processed_time_key = Self::processed_time_key(&ibc_height, &mut Vec::new());
            gm.push(GenesisMetadata {
                key: processed_height_key.clone(),
                value: processed_height.to_be_bytes().to_vec(),
            });

            gm.push(GenesisMetadata {
                key: processed_time_key.clone(),
                value: processed_time.to_be_bytes().to_vec(),
            });
        }
        Ok(gm)
    }

    pub fn get_client_status(deps: Deps) -> StdResult<Binary> {
        let client_state = QueryHandler::get_client_state(deps.storage, CLIENT_ID);
        if client_state.is_err() {
            return to_binary(&QueryResponse::status("Unknown".to_string()));
        }
        let client_state = client_state.unwrap();
        if client_state.frozen_height > 0 {
            return to_binary(&QueryResponse::status("Frozen".to_string()));
        }

        to_binary(&QueryResponse::status("Active".to_string()))
    }
}
impl IQueryHandler for QueryHandler {
    fn get_client_state(
        storage: &dyn cosmwasm_std::Storage,
        client_id: &str,
    ) -> Result<
        common::icon::icon::lightclient::v1::ClientState,
        cw_light_client_common::ContractError,
    > {
        let any_bytes = storage
            .get(&get_client_state_key())
            .ok_or(ContractError::ClientStateNotFound(client_id.to_string()))?;
        decode_client_state(&any_bytes)
    }

    fn get_consensus_state(
        storage: &dyn cosmwasm_std::Storage,
        client_id: &str,
        height: u64,
    ) -> Result<common::icon::icon::lightclient::v1::ConsensusState, ContractError> {
        let ibc_height = to_ibc_height(height);
        let any_bytes = storage.get(&get_consensus_state_key(ibc_height)).ok_or(
            ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            },
        )?;
        decode_consensus_state(&any_bytes)
    }
}
