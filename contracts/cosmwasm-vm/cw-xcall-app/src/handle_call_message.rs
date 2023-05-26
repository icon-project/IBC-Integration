use common::utils::keccak256;

use crate::ack::acknowledgement_data_on_success;

use super::*;

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

        let message = XCallMessage {
            data: proxy_requests.data()?.to_vec(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: proxy_requests.to().to_string(),
            msg: to_binary(&message).unwrap(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_CALL_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

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
        info: MessageInfo,
        sequence_no: u128,
    ) -> Result<Response, ContractError> {
        let call_request = self.query_request(deps.storage, sequence_no)?;

        self.ensure_call_request_not_null(sequence_no, &call_request)
            .unwrap();
        self.ensure_rollback_enabled(call_request.enabled())
            .unwrap();

        let message = XCallMessage {
            data: call_request.rollback().to_vec(),
        };
        let call_message: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: call_request.to().to_string(),
            msg: to_binary(&message).unwrap(),
            funds: info.funds,
        });

        let sub_msg: SubMsg = SubMsg::reply_on_success(call_message, EXECUTE_ROLLBACK_ID);

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_call")
            .add_submessage(sub_msg))
    }

    /// This function receives packet data, decodes it, and then handles either a request or a response
    /// based on the message type.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is short for "dependencies mutable". It is a
    /// struct that provides access to the dependencies needed by the contract to execute its logic.
    /// These dependencies include the storage, the API to interact with the blockchain, and the querier
    /// to query data
    /// * `message`: The `message` parameter is of type `IbcPacket` and represents the packet received
    /// by the contract from another chain. It contains the data sent by the sender chain and metadata
    /// about the packet, such as the sender and receiver addresses, the sequence number, and the
    /// timeout height.
    ///
    /// Returns:
    ///
    /// a `Result` object with either an `IbcReceiveResponse` or a `ContractError`.
    pub fn receive_packet_data(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let call_service_message: CallServiceMessage = CallServiceMessage::try_from(message)?;

        match call_service_message.message_type() {
            CallServiceMessageType::CallServiceRequest => {
                self.hanadle_request(deps, info, call_service_message.payload())
            }
            CallServiceMessageType::CallServiceResponse => {
                self.handle_response(deps, info, call_service_message.payload())
            }
        }
    }

    /// This function handles a request by incrementing the last request ID, parsing a message request,
    /// inserting the request into storage, and returning an acknowledgement response.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may need
    /// to interact with.
    /// * `data`: `data` is a slice of bytes that contains the serialized `CallServiceMessageRequest`
    /// message sent by the client. This message contains information about the service call to be made,
    /// such as the sender, recipient, sequence number, and data payload.
    /// * `packet`: `packet` is an IBC packet received by the contract. It contains information about the
    /// sender, receiver, and the data being transmitted.
    ///
    /// Returns:
    ///
    /// an `IbcReceiveResponse` object wrapped in a `Result` with a possible `ContractError`.
    pub fn hanadle_request(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        data: &[u8],
    ) -> Result<Response, ContractError> {
        let request_id = self.increment_last_request_id(deps.storage)?;
        let message_request: CallServiceMessageRequest = data.try_into()?;

        let from = message_request.from();
        let to = message_request.to();

        let request = CallServiceMessageRequest::new(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            message_request.protocols().clone(),
            message_request.rollback(),
            message_request.data()?.into(),
        );

        if request.protocols().len() > 1 {
            let key = keccak256(data).to_vec();
            let caller = info.sender;
            self.save_pending_requests(deps.storage, key.clone(), caller.to_string())?;
            let registered =
                self.get_pending_requests_by_hash(deps.as_ref().storage, key.clone())?;

            if registered.len() != request.protocols().len() {
                return Ok(Response::new());
            }

            self.remove_pending_request_by_hash(deps.storage, key)?;
        }

        self.insert_request(deps.storage, request_id, request)?;

        let event = event_call_message(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            request_id,
        );
        let acknowledgement_data = to_binary(&cw_common::client_response::XcallPacketAck {
            acknowledgement: make_ack_success().to_vec(),
        })
        .map_err(ContractError::Std)?;

        Ok(Response::new()
            .add_attribute("action", "call_service")
            .add_attribute("method", "handle_response")
            .set_data(acknowledgement_data)
            .add_event(event))
    }

    /// This function handles the response received from a call to an external service.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// need to interact with.
    /// * `data`: `data` is a slice of bytes that contains the response message received from the
    /// external service provider. It is converted into a `CallServiceMessageResponse` struct using the
    /// `try_into()` method.
    /// * `packet`: `packet` is an IBC packet that was received by the contract and triggered the
    /// `handle_response` function. It contains information about the source and destination chains, as
    /// well as the data payload that was sent.
    ///
    /// Returns:
    ///
    /// a `Result` containing an `IbcReceiveResponse` on success or a `ContractError` on failure.
    pub fn handle_response(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        data: &[u8],
    ) -> Result<Response, ContractError> {
        let message: CallServiceMessageResponse = data.try_into()?;
        let response_sequence_no = message.sequence_no();

        let mut call_request = self.query_request(deps.storage, response_sequence_no)?;

        if call_request.is_null() {
            let acknowledgement_data = to_binary(&cw_common::client_response::XcallPacketAck {
                acknowledgement: make_ack_fail(format!(
                    "handle_resposne: no request for {response_sequence_no}"
                ))
                .to_vec(),
            })
            .map_err(ContractError::Std)?;
            return Ok(Response::new()
                .add_attribute("action", "call_service")
                .add_attribute("method", "handle_response")
                .add_attribute(
                    "message",
                    format!("handle_resposne: no request for {response_sequence_no}"),
                )
                .set_data(acknowledgement_data));
        }

        if call_request.protocols().len() > 1 {
            let key = keccak256(data).to_vec();
            let caller = info.sender;
            self.save_pending_responses(deps.storage, key.clone(), caller.to_string())?;
            let registered =
                self.get_pending_responses_by_hash(deps.as_ref().storage, key.clone())?;

            if registered.len() != call_request.protocols().len() {
                return Ok(Response::new());
            }

            self.remove_pending_responses_by_hash(deps.storage, key)?;
        }

        match message.response_code() {
            CallServiceResponseType::CallServiceResponseSuccess => {
                let event = match message.message().is_empty() {
                    true => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        "",
                    ),
                    false => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        message.message(),
                    ),
                };
                self.cleanup_request(deps.storage, response_sequence_no);
                Ok(Response::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_data(acknowledgement_data_on_success()?)
                    .add_event(event))
            }
            _ => {
                self.ensure_rollback_length(call_request.rollback())
                    .unwrap();
                call_request.set_enabled();
                self.set_call_request(deps.storage, response_sequence_no, call_request)?;

                let event = event_rollback_message(response_sequence_no);

                Ok(Response::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .set_data(acknowledgement_data_on_success()?)
                    .add_event(event))
            }
        }
    }

    /// The function removes a call request from storage based on a given sequence number.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This means
    /// that the function can accept any object that implements the `Storage` trait. The `Storage` trait
    /// defines methods for storing and retrieving data in a persistent storage, such as a database or a
    /// file system.
    /// * `sequence_no`: `sequence_no` is an unsigned 128-bit integer that represents the sequence
    /// number of a call request that needs to be cleaned up. It is used as an identifier to locate and
    /// remove the corresponding call request from the storage.
    pub fn cleanup_request(&self, store: &mut dyn Storage, sequence_no: u128) {
        self.remove_call_request(store, sequence_no);
    }
}
