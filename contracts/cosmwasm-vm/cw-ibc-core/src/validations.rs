use common::ibc::{
    core::ics03_connection::{connection::State, error::ConnectionError},
    Height,
};
use cw_common::ibc_types::{IbcConnectionEnd, IbcConnectionId};

use crate::ContractError;

pub fn ensure_consensus_height_valid(
    host_height: &Height,
    consensus_height: &Height,
) -> Result<(), ContractError> {
    if consensus_height > host_height {
        return Err(ContractError::IbcConnectionError {
            error: ConnectionError::InvalidConsensusHeight {
                target_height: *consensus_height,
                current_height: *host_height,
            },
        });
    }
    Ok(())
}

pub fn ensure_connection_state(
    connection_id: &IbcConnectionId,
    connection: &IbcConnectionEnd,
    state: &State,
) -> Result<(), ContractError> {
    if !connection.state_matches(state) {
        return Err(ContractError::IbcConnectionError {
            error: ConnectionError::ConnectionMismatch {
                connection_id: connection_id.clone(),
            },
        });
    }
    Ok(())
}
