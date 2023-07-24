use common::ibc::core::ics04_channel::channel::{ChannelEnd, State};
use common::ibc::core::ics24_host::identifier::ChannelId;
use cw_common::ibc_types::ChannelError;

use crate::ContractError;

pub fn ensure_channel_not_closed(
    channel_id: &ChannelId,
    channel_end: &ChannelEnd,
) -> Result<(), ContractError> {
    if channel_end.state_matches(&State::Closed) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelClosed {
                channel_id: channel_id.clone(),
            },
        });
    }
    Ok(())
}

pub fn ensure_channel_state(
    channel_id: &ChannelId,
    channel_end: &ChannelEnd,
    state: &State,
) -> Result<(), ContractError> {
    if !channel_end.state_matches(&state) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidChannelState {
                channel_id: channel_id.clone(),
                state: channel_end.state,
            },
        });
    }
    Ok(())
}

pub fn validate_connection_length(channel_end: &ChannelEnd) -> Result<(), ContractError> {
    if channel_end.connection_hops().len() != 1 {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: channel_end.connection_hops().len(),
            },
        });
    }

    Ok(())
}
