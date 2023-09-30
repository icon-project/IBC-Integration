use common::ibc::Height;
use cw_light_client_common::{traits::IQueryHandler, ContractError};

use crate::utils::{
    decode_client_state, decode_consensus_state, get_client_state_key, get_consensus_state_key,
};

pub struct QueryHandler;
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
        return decode_client_state(&any_bytes);
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
        return decode_consensus_state(&any_bytes);
    }
}
