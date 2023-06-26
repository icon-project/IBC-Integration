use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Reply, Response};

use crate::{
    error::ContractError,
    events::event_call_executed,
    state::CwCallService,
    types::{
        message::CallServiceMessage,
        response::{CallServiceMessageResponse, CallServiceResponseType},
    },
};

impl<'a> CwCallService<'a> {
    /// This function executes a call message to a smart contract and returns a response with a
    /// submessage.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, API, and querier. It is used to interact with the blockchain and other
    /// contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender address, the amount of funds sent with the message, and the
    /// gas limit.
    /// * `request_id`: `request_id` is a unique identifier for a specific request made by a user. It is
    /// used to retrieve the details of the request from the contract's storage and execute the
    /// corresponding action.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    pub fn execute_call(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        request_id: u128,
    ) -> Result<Response, ContractError> {
        let proxy_requests = self
            .query_message_request(deps.storage, request_id)
            .unwrap();

        self.ensure_request_not_null(request_id, &proxy_requests)
            .unwrap();

        let sub_msg = self.call_dapp_handle_message(
            info,
            proxy_requests.to(),
            proxy_requests.from(),
            proxy_requests.data().unwrap().to_vec(),
            proxy_requests.protocols().clone(),
        )?;

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn execute_call_reply(
        &self,
        deps: Deps,
        env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.last_request_id().load(deps.storage)?;
        let request = self.message_request().load(deps.storage, req_id)?;

        let responses = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                let code = 0;

                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseSuccess,
                    "success",
                );
                let event = event_call_executed(req_id, code, message_response.message());
                (message_response, event)
            }
            cosmwasm_std::SubMsgResult::Err(err) => {
                let code = -1;
                let error_message = format!("CallService Reverted : {err}");
                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                );
                let event = event_call_executed(req_id, code, &error_message);
                (message_response, event)
            }
        };

        if !request.rollback() {
            let message: CallServiceMessage = responses.0.into();

            #[cfg(feature = "native_ibc")]
            {
                let packet = self.create_packet_response(deps, env, to_binary(&message).unwrap());

                return Ok(Response::new()
                    .add_attribute("action", "call_message")
                    .add_attribute("method", "execute_callback")
                    .add_message(packet));
            }
        }

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_callback")
            .add_event(responses.1))
    }
}
