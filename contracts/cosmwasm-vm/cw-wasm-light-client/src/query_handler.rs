use common::ibc::Height;
use cosmwasm_std::{Order, StdResult};
use cw_light_client_common::{constants::PROCESSED_HEIGHTS, traits::IQueryHandler, ContractError};
use cw_storage_plus::Endian;

use crate::{
    msg::GenesisMetadata,
    utils::{
        decode_client_state, decode_consensus_state, get_client_state_key, get_consensus_state_key,
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
    ) -> Vec<GenesisMetadata> {
        let heights = PROCESSED_HEIGHTS
            .prefix(client_id.to_string())
            .keys(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<u64>>>()
            .unwrap();
        let mut gm: Vec<GenesisMetadata> = Vec::<GenesisMetadata>::new();
        for h in heights {
            let processed_height =
                Self::get_processed_blocknumber_at_height(storage, client_id, h).unwrap();
            let processed_time = Self::get_processed_time_at_height(storage, client_id, h).unwrap();
            let ibc_height = Height::new(0, h).unwrap();
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
        gm
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
        let ibc_height = Height::new(0, height).unwrap();
        let any_bytes = storage.get(&get_consensus_state_key(ibc_height)).ok_or(
            ContractError::ConsensusStateNotFound {
                height,
                client_id: client_id.to_string(),
            },
        )?;
        decode_consensus_state(&any_bytes)
    }
}
