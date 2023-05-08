use super::*;

impl<'a> CwIbcConnection<'a> {
    /// This function queries and returns the last sequence number stored in a given storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be persisted. In the context of smart
    /// contract development on the CosmWasm platform, `store` is typically provided by the runtime
    /// environment and is
    ///
    /// Returns:
    ///
    /// a `Result` containing a `u128` value or a `ContractError` if an error occurs. The `u128` value
    /// represents the last sequence number stored in the contract's storage.
    pub fn query_last_sequence_no(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        let last_sequence = self.last_sequence_no().load(store)?;

        Ok(last_sequence)
    }

    /// The function increments the last sequence number stored in a contract's storage and returns the
    /// updated value.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data between contract executions. The
    /// `increment_last_sequence_no` function updates the value of the last sequence number stored in
    /// the contract's storage by incrementing it
    ///
    /// Returns:
    ///
    /// a `Result` containing an unsigned 128-bit integer (`u128`) or a `ContractError` if an error
    /// occurs.
    pub fn increment_last_sequence_no(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence_no =
            self.last_sequence_no()
                .update(store, |mut seq| -> Result<_, ContractError> {
                    seq += 1;

                    Ok(seq)
                })?;

        Ok(sequence_no)
    }

    /// This function sets the last sequence number in a storage and returns the updated value.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data between contract executions. The `dyn`
    /// keyword indicates that the type of the object implementing the `Storage` trait is not known at
    /// compile time and will
    /// * `sequence`: The `sequence` parameter is an unsigned 128-bit integer representing the last
    /// sequence number to be set. This function updates the last sequence number stored in the
    /// contract's storage with the provided value.
    ///
    /// Returns:
    ///
    /// a `Result` containing a `u128` value or a `ContractError` if an error occurs.
    pub fn set_last_sequence_no(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_sequence_no()
                .update(store, |mut seq| -> Result<_, ContractError> {
                    seq.clone_from(&sequence);
                    Ok(seq)
                })?;

        Ok(req_id)
    }

    /// This function queries the last request ID from a storage and returns it as a result.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to interact
    /// with the contract's storage and retrieve the value of the `last_request_id` variable. The `load`
    /// method is called on `last_request_id()` to retrieve the value from storage.
    ///
    /// Returns:
    ///
    /// a `Result` containing either a `u128` value representing the last request ID or a `ContractError` if
    /// there was an error while loading the last request ID from the storage.
    pub fn query_last_request_id(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        let last_req_id = self.last_request_id().load(store)?;

        Ok(last_req_id)
    }

    /// The function increments the last request ID stored in a storage object and returns the updated
    /// ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data between contract invocations. The
    /// `increment_last_request_id` function updates the value of the last request ID stored in the
    /// contract's storage by incrementing
    ///
    /// Returns:
    ///
    /// a `Result` containing an unsigned 128-bit integer (`u128`) or a `ContractError` if an error
    /// occurs during the execution of the function.
    pub fn increment_last_request_id(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_request_id()
                .update(store, |mut req_id| -> Result<_, ContractError> {
                    req_id += 1;

                    Ok(req_id)
                })?;

        Ok(req_id)
    }

    /// This function sets the last request ID and returns the updated ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data between contract executions. The
    /// `Storage` trait defines methods for reading and writing data to the contract's storage.
    /// * `request_id`: The `request_id` parameter is a 128-bit unsigned integer that represents the ID
    /// of the last request made to the contract. This function sets the value of the last request ID to
    /// the provided `request_id` value.
    ///
    /// Returns:
    ///
    /// a `Result` containing a `u128` value or a `ContractError` if an error occurs.
    pub fn set_last_request_id(
        &self,
        store: &mut dyn Storage,
        request_id: u128,
    ) -> Result<u128, ContractError> {
        let req_id =
            self.last_request_id()
                .update(store, |mut req_id| -> Result<_, ContractError> {
                    req_id.clone_from(&request_id);
                    Ok(req_id)
                })?;

        Ok(req_id)
    }

    /// This function sets a call request in storage for a given sequence number.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data between contract executions.
    /// * `sequence`: The `sequence` parameter is an unsigned 128-bit integer that represents the sequence
    /// number of the `CallRequest`. It is used to uniquely identify the `CallRequest` within the contract's
    /// storage.
    /// * `call_request`: `call_request` is a struct that represents a request to call another contract. It
    /// contains information such as the address of the contract to call, the amount of tokens to transfer,
    /// and the input data to pass to the contract.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either `Ok(())` if the call request was successfully saved in
    /// the storage or `Err(ContractError::Std(err))` if there was an error while saving the call request.
    // pub fn set_call_request(
    //     &self,
    //     store: &mut dyn Storage,
    //     sequence: u128,
    //     call_request: CallRequest,
    // ) -> Result<(), ContractError> {
    //     match self.call_requests().save(store, sequence, &call_request) {
    //         Ok(_) => Ok(()),
    //         Err(err) => Err(ContractError::Std(err)),
    //     }
    // }

    /// This function checks if a request ID exists in a storage and returns an error if it doesn't.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an interface
    /// that defines methods for reading and writing data to the contract's storage. The
    /// `contains_request` function uses this parameter to check if a specific request ID exists in the
    /// contract's storage.
    /// * `request_id`: `request_id` is an input parameter of type `u128` which represents the unique
    /// identifier of a message request. This function checks if the given `request_id` exists in the
    /// storage or not. If it exists, it returns `Ok(())`, otherwise it returns an error of type `
    ///
    /// Returns:
    ///
    /// The function `contains_request` returns a `Result` type with either an `Ok(())` value indicating
    /// that the request with the given `request_id` exists in the storage, or an `Err` value of type
    /// `ContractError::InvalidRequestId` indicating that the request with the given `request_id` does
    /// not exist in the storage.
    pub fn contains_request(
        &self,
        store: &dyn Storage,
        request_id: u128,
    ) -> Result<(), ContractError> {
        match self.message_request().has(store, request_id) {
            true => Ok(()),
            false => Err(ContractError::InvalidRequestId { id: request_id }),
        }
    }

    /// This function queries a message request from storage and returns it as a result or an error.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and retrieve data that has been previously stored. The
    /// `Storage` trait provides methods for reading and writing data to the contract's storage.
    /// * `request_id`: `request_id` is an input parameter of type `u128` which represents a unique
    /// identifier for a specific message request. This function queries the storage to retrieve the
    /// `CallServiceMessageRequest` associated with the given `request_id`.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `CallServiceMessageRequest` if the message
    /// request with the given `request_id` is found in the `store`, or a `ContractError` if there was
    /// an error while loading the message request from the `store`.
    pub fn query_message_request(
        &self,
        store: &dyn Storage,
        request_id: u128,
    ) -> Result<Vec<u8>, ContractError> {
        match self.message_request().load(store, request_id) {
            Ok(result) => Ok(result),
            Err(err) => Err(ContractError::Std(err)),
        }
    }

    
   

    /// This function removes a message request with a given ID from a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is
    /// likely a storage implementation that allows the code to persist data on-chain. The
    /// `remove_request` function uses this storage to remove a message request with the given
    /// `request_id`.
    /// * `request_id`: `request_id` is an unsigned 128-bit integer that represents the unique
    /// identifier of a message request that needs to be removed from the storage.
    pub fn remove_request(&self, store: &mut dyn Storage, request_id: u128) {
        self.message_request().remove(store, request_id);
    }

    /// This function queries a call request from storage based on a given sequence number.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an interface
    /// that defines methods for reading and writing data to the contract's storage. The `query_request`
    /// function uses this parameter to load a `CallRequest` object from the storage.
    /// * `sequence`: `sequence` is an unsigned 128-bit integer that represents the sequence ID of a call
    /// request. It is used as a unique identifier for each call request made to the contract. The
    /// `query_request` function takes this sequence ID as an input parameter and retrieves the
    /// corresponding call request from the contract's
    ///
    /// Returns:
    ///
    /// The function `query_request` returns a `Result` containing either a `CallRequest` if the call
    /// request with the given `sequence` exists in the storage, or a `ContractError::InvalidSequenceId`
    /// if it does not exist.
    // pub fn query_request(
    //     &self,
    //     store: &dyn Storage,
    //     sequence: u128,
    // ) -> Result<CallRequest, ContractError> {
    //     match self.call_requests().may_load(store, sequence)? {
    //         Some(request) => Ok(request),
    //         None => Err(ContractError::InvalidSequenceId { id: sequence }),
    //     }
    // }

    /// This function removes a call request from the storage based on its sequence number.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// access and modify the storage of the smart contract. The `dyn` keyword indicates that `Storage`
    /// is a dynamic trait object, which means that it can be used to refer to any type that
    /// * `sequence_no`: `sequence_no` is an unsigned 128-bit integer that represents the unique
    /// identifier of a call request that needs to be removed from the storage. The
    /// `remove_call_request` function takes a mutable reference to a `dyn Storage` trait object and
    /// removes the call request with the specified `sequence_no`
    // pub fn remove_call_request(&self, store: &mut dyn Storage, sequence_no: u128) {
    //     self.call_requests().remove(store, sequence_no);
    // }

    /// This function initializes the last sequence number in a storage and returns an error if it
    /// fails.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data between contract executions.
    /// * `sequence_no`: `sequence_no` is an unsigned 128-bit integer representing the last sequence
    /// number used in a transaction. This function initializes the last sequence number to the provided
    /// value in the storage of the smart contract.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` type with either an `Ok(())` value indicating that the
    /// `sequence_no` was successfully saved to the storage, or an `Err` value with a
    /// `ContractError::Std` variant indicating that an error occurred while saving the `sequence_no`.
    pub fn init_last_sequence_no(
        &self,
        store: &mut dyn Storage,
        sequence_no: u128,
    ) -> Result<(), ContractError> {
        match self.last_sequence_no().save(store, &sequence_no) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This function initializes the last request ID in a storage and returns an error if it fails.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `Storage` trait defines methods for reading
    /// and writing data to the contract's storage.
    /// * `request_id`: `request_id` is a 128-bit unsigned integer that represents the unique identifier
    /// of a request. This function takes this `request_id` as an input parameter and saves it to the
    /// contract's storage using the `save` method of the `last_request_id` field. The `store` parameter
    ///
    /// Returns:
    ///
    /// This function returns a `Result` object with either `Ok(())` if the `request_id` was
    /// successfully saved in the storage, or `Err(ContractError::Std(error))` if there was an error
    /// while saving the `request_id`.
    pub fn init_last_request_id(
        &self,
        store: &mut dyn Storage,
        request_id: u128,
    ) -> Result<(), ContractError> {
        match self.last_request_id().save(store, &request_id) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
