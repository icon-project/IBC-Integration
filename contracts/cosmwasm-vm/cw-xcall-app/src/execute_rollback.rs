use cosmwasm_std::DepsMut;
use cosmwasm_std::MessageInfo;
use cosmwasm_std::{Deps, Env, Reply, Response};
use schemars::_serde_json::to_string;

use crate::error::ContractError;
use crate::events::event_rollback_executed;
use crate::state::{CwCallService, EXECUTE_ROLLBACK_ID};
use crate::types::response::{CallServiceMessageResponse, CallServiceResponseType};

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

        self.ensure_call_request_not_null(sequence_no, &call_request)
            .unwrap();
        self.ensure_rollback_enabled(call_request.enabled())
            .unwrap();
        let from = self.get_network_address(deps.as_ref().storage, &env)?;

        let sub_msg = self.call_dapp_handle_message(
            info,
            call_request.to(),
            &from,
            call_request.rollback().to_vec(),
            call_request.protocols().clone(),
            EXECUTE_ROLLBACK_ID,
        )?;
        self.store_execute_rollback_id(deps.storage, sequence_no)?;

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    /// This function handles the response of a call to a service and generates a response with an
    /// event.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an instance of the `Deps` struct, which provides access to the contract's
    /// dependencies such as storage, API, and context.
    /// * `msg`: `msg` is a `Reply` struct that contains the result of a sub-message that was sent by
    /// the contract to another contract or external system. It is used to construct a
    /// `CallServiceMessageResponse` that will be returned as part of the `Response` to the original
    /// message that triggered the
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to be
    /// returned by the contract and `ContractError` is an enum representing any errors that may occur
    /// during contract execution.
    pub fn execute_rollback_reply(
        &self,
        deps: Deps,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let sn = self.get_execute_rollback_id(deps.storage)?;

        let response = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => CallServiceMessageResponse::new(
                sn,
                CallServiceResponseType::CallServiceResponseSuccess,
                "",
            ),
            cosmwasm_std::SubMsgResult::Err(err) => {
                let error_message = format!("CallService Reverted : {err}");
                CallServiceMessageResponse::new(
                    sn,
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                )
            }
        };

        let event = event_rollback_executed(
            sn,
            (response.response_code().clone()).into(),
            &to_string(response.message()).unwrap(),
        );

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_rollback")
            .add_event(event))
    }
}
