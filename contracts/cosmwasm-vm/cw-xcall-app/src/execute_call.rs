use std::str::FromStr;

use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsg};
use cw_common::xcall_types::network_address::NetworkAddress;

use crate::{
    error::ContractError,
    events::event_call_executed,
    state::{CwCallService, EXECUTE_CALL_ID},
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
        let proxy_requests = self.get_proxy_request(deps.storage, request_id).unwrap();

        self.ensure_request_not_null(request_id, &proxy_requests)
            .unwrap();

        let sub_msg = self.call_dapp_handle_message(
            info,
            proxy_requests.to().clone(),
            proxy_requests.from().clone(),
            proxy_requests.data().unwrap().to_vec(),
            proxy_requests.protocols().clone(),
            EXECUTE_CALL_ID,
        )?;
        self.remove_proxy_request(deps.storage, request_id);
        self.store_execute_request_id(deps.storage, request_id)?;

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    pub fn execute_call_reply(
        &self,
        deps: Deps,
        _env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.get_execute_request_id(deps.storage)?;

        let request = self.get_proxy_request(deps.storage, req_id)?;

        let (response, event) = match msg.result {
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
                let code = CallServiceResponseType::CallServiceResponseFailure;
                let error_message = format!("CallService Reverted : {err}");
                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    code.clone(),
                    &error_message,
                );
                let event = event_call_executed(req_id, code.into(), &error_message);
                (message_response, event)
            }
        };
        let mut submsgs: Vec<SubMsg> = vec![];
        let sn: i64 = -(request.sequence_no() as i64);
        if request.rollback() {
            let message: CallServiceMessage = response.into();
            let mut reply_address = request.protocols().clone();
            let from = request.from().clone();
            if request.protocols().is_empty() {
                let default_connection =
                    self.get_default_connection(deps.storage, from.get_nid().as_str())?;
                reply_address = vec![default_connection.to_string()];
            }
            submsgs = reply_address
                .iter()
                .map(|to| {
                    return self.call_connection_send_message(
                        to,
                        vec![],
                        from.clone(),
                        sn,
                        &message,
                    );
                })
                .collect::<Result<Vec<SubMsg>, ContractError>>()?;
        }

        Ok(Response::new()
            .add_submessages(submsgs)
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_callback")
            .add_event(event))
    }
}
