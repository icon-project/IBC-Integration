use cosmwasm_std::DepsMut;
use cosmwasm_std::MessageInfo;
use cosmwasm_std::ReplyOn;

use cosmwasm_std::{Env, Response};

use crate::error::ContractError;
use crate::events::event_rollback_executed;
use crate::state::{CwCallService, EXECUTE_ROLLBACK_ID};

impl<'a> CwCallService<'a> {
    /// This function executes a rollback operation for a previously made call request.
    ///
    /// Arguments:
    ///
    /// * `deps`: A mutable reference to the dependencies of the contract, which includes access to the
    /// storage and other modules.
    /// * `info`: `info` is a struct that contains information about the message sender, such as their
    /// address and the amount of funds they are sending with the message. It is of type `MessageInfo`.
    /// * `sequence_no`: The sequence number is a unique identifier assigned to each XCall request made
    /// by the user. It is used to track the status of the request and to ensure that the correct request
    /// is being executed or rolled back.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing possible errors that can occur
    /// during contract execution.
    pub fn execute_rollback(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        sequence_no: u128,
    ) -> Result<Response, ContractError> {
        let call_request = self.get_call_request(deps.storage, sequence_no)?;
        self.cleanup_request(deps.storage, sequence_no);

        self.ensure_call_request_not_null(sequence_no, &call_request)
            .unwrap();
        self.ensure_rollback_enabled(call_request.enabled())
            .unwrap();
        let from = self.get_own_network_address(deps.as_ref().storage, &env)?;

        let mut sub_msg = self.call_dapp_handle_message(
            info,
            // the original caller is stored as from in call request
            call_request.from().clone(),
            from,
            call_request.rollback().to_vec(),
            call_request.protocols().clone(),
            EXECUTE_ROLLBACK_ID,
        )?;
        sub_msg.reply_on = ReplyOn::Never;

        let event = event_rollback_executed(sequence_no);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_rollback")
            .add_event(event)
            .add_submessage(sub_msg))
    }
}
