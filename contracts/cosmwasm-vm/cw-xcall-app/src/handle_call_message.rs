use common::utils::keccak256;

use crate::ack::acknowledgement_data_on_success;

use super::*;

impl<'a> CwCallService<'a> {
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
    pub fn handle_call_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _from: String,
        _sn: Option<i64>,
        message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let call_service_message: CallServiceMessage = CallServiceMessage::try_from(message)?;

        match call_service_message.message_type() {
            CallServiceMessageType::CallServiceRequest => {
                self.handle_request(deps, info, call_service_message.payload())
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
    pub fn handle_request(
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

        self.store_proxy_request(deps.storage, request_id, &request)?;

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

        let mut call_request = self.get_call_request(deps.storage, response_sequence_no)?;

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
                        (message.response_code().clone()).into(),
                        "",
                    ),
                    false => event_response_message(
                        response_sequence_no,
                        (message.response_code().clone()).into(),
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
                self.store_call_request(deps.storage, response_sequence_no, &call_request)?;

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
