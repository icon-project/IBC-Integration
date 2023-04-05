use super::*;

pub fn channel_close_confirm_validate(
    message: &MsgChannelCloseConfirm,
    chan_end_on_b: &ChannelEnd,
) -> Result<(), ContractError> {
    if !chan_end_on_b.state_matches(&State::Closed) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelClosed {
                channel_id: message.chan_id_on_b.clone(),
            },
        });
    }
    if chan_end_on_b.connection_hops().len() != 1 {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: chan_end_on_b.connection_hops().len(),
            },
        });
    }

    Ok(())
}
