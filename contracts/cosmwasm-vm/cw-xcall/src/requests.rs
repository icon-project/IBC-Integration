use super::*;

impl<'a> CwCallService<'a> {
    /// This function queries and returns the last sequence number stored in a given storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be persisted. In the context of smart
    /// contract development on the CosmWasm platform, `store` is typically provided by the runtime
    /// environment.
    ///
    /// Returns:
    ///
    /// a `Result` containing a `u128` value or a `ContractError` if an error occurs. The `u128` value
    /// represents the last sequence number stored in the contract's storage.
    pub fn query_last_sequence_no(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        let last_sequence = self.get_current_sn(store)?;

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
        self.get_next_sn(store)
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
        let req_id = self
            .sn()
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
        match self.sn().save(store, &sequence_no) {
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
