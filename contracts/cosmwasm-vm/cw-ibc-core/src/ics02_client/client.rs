use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This method increases the client counter by updating the next client sequence in the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `increase_client_counter` function
    /// updates the storage by incrementing the client sequence number and returns the updated sequence
    /// number.
    ///
    /// Returns:
    ///
    /// a `Result<u64, ContractError>`. If the operation is successful, it will return an `Ok` variant
    /// containing the updated client sequence number as a `u64`. If there is an error, it will return an
    /// `Err` variant containing a `ContractError`.
    pub fn increase_client_counter(&self, store: &mut dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_client_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        ) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(error),
        }
    }

    /// This method retrieves the next client sequence number from the IBC store and returns it as a
    /// result or throws an error if it is invalid.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `next_client_sequence()` function is called
    /// on the `ibc_store()` method of the current instance of the smart contract, and the resulting
    /// sequence number
    ///
    /// Returns:
    ///
    /// The function `client_counter` returns a `Result` that contains either an `u64` value
    /// representing the next client sequence, or a `ContractError` if there was an error retrieving the
    /// sequence from the storage.
    pub fn client_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_client_sequence().may_load(store) {
            Ok(result) => match result {
                Some(sequence) => Ok(sequence),
                None => Err(ContractError::InvalidNextClientSequence {}),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This function initializes a client counter by saving a sequence number to a storage instance.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `init_client_counter` function saves the
    /// `sequence_no` value to the storage using the `save` method of the `next_client_sequence`
    /// * `sequence_no`: `sequence_no` is an unsigned 64-bit integer that represents the sequence number
    /// of the client counter. It is used to initialize the client counter in the storage.
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` where `()` indicates that the function returns nothing on success
    /// and `ContractError` is an error type that can be returned in case of an error.
    pub fn init_client_counter(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_client_sequence()
            .save(store, &sequence_no)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method retrieves the client type of a given client ID from a storage and returns it as an
    /// `IbcClientType` or an error.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to access
    /// the storage of the smart contract. The `Storage` trait defines methods for reading and writing
    /// data to the contract's storage.
    /// * `client_id`: The ID of the client for which we want to retrieve the client type.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either an `IbcClientType` or a `ContractError`. The
    /// `IbcClientType` is the type of the client associated with the given `ClientId`, which is obtained
    /// by loading the client type from the `ibc_store`. If the client type is successfully loaded, the
    /// function returns `Ok(client_type.client_type())`. If the client type
    pub fn get_client_type(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<IbcClientType, ContractError> {
        match self
            .ibc_store()
            .client_types()
            .may_load(store, client_id.clone())
        {
            Ok(result) => match result {
                Some(client_type) => Ok(client_type.client_type()),
                None => Err(ContractError::InvalidClientId {
                    client_id: client_id.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method retrieves a client from a registry and returns it as a string, or returns an error
    /// if the client type is invalid or if there is an error loading the client.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and retrieve data from it.
    /// * `client_type`: A parameter of type `ClientType` which represents the type of client being
    /// requested from the registry.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `String` representing the client ID if the
    /// client is found in the registry, or a `ContractError` if there is an error or the client is not
    /// found.
    pub fn get_client_from_registry(
        &self,
        store: &dyn Storage,
        client_type: ClientType,
    ) -> Result<String, ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .may_load(store, client_type.clone())
        {
            Ok(result) => match result {
                Some(client) => Ok(client),
                None => Err(ContractError::InvalidClientType {
                    client_type: client_type.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method retrieves client implementations from a storage based on a given client ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to interact
    /// with the storage of the smart contract. The `get_client_implementations` function uses this
    /// parameter to load the client implementation data from the storage.
    /// * `client_id`: The `client_id` parameter is of type `ClientId` and represents the identifier of a
    /// client implementation in the IBC (Inter-Blockchain Communication) protocol. It is used to
    /// retrieve the client implementation from the storage.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `String` representing the client
    /// implementation associated with the given `ClientId`, or a `ContractError` if there was an issue
    /// with loading the client implementation from the storage or if the `ClientId` is invalid.
    pub fn get_client_implementations(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<String, ContractError> {
        match self
            .ibc_store()
            .client_implementations()
            .may_load(store, client_id.clone())
        {
            Ok(result) => match result {
                Some(client) => Ok(client),
                None => Err(ContractError::InvalidClientId {
                    client_id: client_id.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method stores the client type for a given client ID in a storage object.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `store` parameter is passed as a reference
    /// to the function so that it can be modified within the function.
    /// * `client_id`: The client ID is a unique identifier for an IBC client. It is used to
    /// differentiate between different clients that are running on the same chain.
    /// * `client_type`: `client_type` is a type of IBC client, which is a software module that connects
    /// to and interacts with other blockchains in the IBC protocol. Examples of client types include
    /// Tendermint and Solo Machine clients. The `store_client_type` function stores the client type for
    /// a given client ID
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` where `()` indicates that the function returns no value on success
    /// and `ContractError` is an error type that can be returned if there is an error while saving the
    /// client type to the storage.
    pub fn store_client_type(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_types()
            .save(store, client_id, &client_type)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method stores a client into a registry
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// interact with the contract's storage and persist data on the blockchain. The `dyn` keyword
    /// indicates that the type of `store` is not known at compile time and will be determined at
    /// runtime.
    /// * `client_type`: The type of the client being stored in the registry. It is of type
    /// `ClientType`, which is likely an enum defined elsewhere in the codebase.
    /// * `client`: The `client` parameter is a `String` representing the identifier of a client that
    /// needs to be stored in the client registry.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either an empty `Ok(())` value if the client was
    /// successfully stored in the registry, or a `ContractError` wrapped in `Err` if there was an error
    /// while saving the client in the registry.
    pub fn store_client_into_registry(
        &self,
        store: &mut dyn Storage,
        client_type: ClientType,
        client: String,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .save(store, client_type, &client)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    /// This method stores client implementations in a storage using a client ID and returns an error if
    /// there is one.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// interact with the contract's storage and persist data.
    /// * `client_id`: The ID of the client implementation being stored.
    /// * `client`: The `client` parameter is a `String` that represents the implementation of a client. It
    /// is being saved in the storage under a specific `client_id`.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either an empty `Ok(())` value if the client implementation
    /// was successfully stored in the provided storage, or a `ContractError::Std` if there was an error
    /// while saving the client implementation.

    pub fn store_client_implementations(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        client: String,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_implementations()
            .save(store, client_id, &client)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    /// The method checks if a client is already registered in the store and returns
    /// an error if it already exists.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and retrieve data from it.
    /// * `client_type`: The `client_type` parameter is a value of the `ClientType` enum, which
    /// represents the type of IBC client being checked for registration. This could be either a
    /// `SoloMachine` or a `Tendermint` client.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` type with either `Ok(())` if the client is not registered, or
    /// an `Err` with a `ContractError` if the client is already registered or if there is an error
    /// while loading the client from the storage.
    pub fn check_client_registered(
        &self,
        store: &dyn Storage,
        client_type: ClientType,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .may_load(store, client_type)
        {
            Ok(result) => match result {
                Some(_) => Err(ContractError::IbcClientError {
                    error: ClientError::Other {
                        description: "Client Implementation Already Exist".to_string(),
                    },
                }),
                None => Ok(()),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }
   /// This method retrieves a client from storage and returns an error if the client is not found.
   /// 
   /// Arguments:
   /// 
   /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is likely an
   /// interface for interacting with some kind of storage system, such as a database or key-value
   /// store. The specific implementation of this trait is not known at compile time and is determined
   /// at runtime.
   /// * `client_id`: The `client_id` parameter is of type `ClientId` and represents the identifier of
   /// the client that needs to be retrieved. It is used to query the storage for the client
   /// implementation associated with this identifier.
   /// 
   /// Returns:
   /// 
   /// a `Result` object that contains either a `String` representing the client or a `ContractError` if
   /// there was an error retrieving the client from the storage or if the client was not found.
    pub fn get_client(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<String, ContractError> {
        let client = self.get_client_implementations(store, client_id.clone())?;

        if client.is_empty() {
            return Err(ContractError::IbcClientError {
                error: ClientError::ClientNotFound {
                    client_id: client_id.ibc_client_id().clone(),
                },
            });
        }
        Ok(client)
    }

    /// This method retrieves the client state from storage using the client ID.
    /// 
    /// Arguments:
    /// 
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be stored and retrieved. The specific
    /// implementation of the `Storage` trait is not specified in this function, allowing for
    /// flexibility in choosing the type of storage
    /// * `client_id`: The `client_id` parameter is of type `ClientId` and represents the identifier of
    /// an IBC client. It is used to retrieve the client state from the storage.
    /// 
    /// Returns:
    /// 
    /// a `Result` containing either a `Vec<u8>` representing the client state or a `ContractError` if
    /// there was an error loading the client state from the storage.
    pub fn get_client_state(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<Vec<u8>, ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id.ibc_client_id());

        let client_state = self
            .ibc_store()
            .commitments()
            .load(store, client_key)
            .map_err(|_| ContractError::IbcDecodeError {
                error: format!("NotFound ClientId({})", client_id.ibc_client_id().as_str()),
            })?;

        Ok(client_state)
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn client_state(
        &self,
        store: &dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Box<dyn ibc::core::ics02_client::client_state::ClientState>, ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);

        let client_state_data = self.ibc_store().commitments().load(store, client_key)?;

        let client_state: ClientState = client_state_data.as_slice().try_into().unwrap();

        Ok(Box::new(client_state))
    }

    pub fn decode_client_state(
        &self,
        client_state: ibc_proto::google::protobuf::Any,
    ) -> Result<Box<dyn IbcClientState>, ContractError> {
        let client_state: ClientState = ClientState::try_from(client_state).unwrap();

        Ok(Box::new(client_state))
    }

    pub fn consensus_state(
        &self,
        store: &dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<Box<dyn IbcConsensusState>, ContractError> {
        let consensus_state_key = commitment::consensus_state_commitment_key(
            client_id,
            height.revision_number(),
            height.revision_height(),
        );

        let consensus_state_data = self
            .ibc_store()
            .commitments()
            .load(store, consensus_state_key)?;

        let consensus_state: ConsensusState =
            ConsensusState::try_from(consensus_state_data).unwrap();

        Ok(Box::new(consensus_state))
    }

    fn next_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ContractError,
    > {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ContractError,
    > {
        todo!()
    }

    pub fn host_height(&self) -> Result<ibc::Height, ContractError> {
        Ok(ibc::Height::new(10, 10).unwrap())
    }

    pub fn host_timestamp(
        &self,
        store: &dyn Storage,
    ) -> Result<ibc::timestamp::Timestamp, ContractError> {
        //TODO Update timestamp logic
        let duration = self.ibc_store().expected_time_per_block().load(store)?;
        let block_time = Duration::from_secs(duration);
        Ok(Timestamp::from_nanoseconds(block_time.as_nanos() as u64).unwrap())
    }

    pub fn host_consensus_state(
        &self,
        height: &ibc::Height,
    ) -> Result<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>, ContractError>
    {
        todo!()
    }

    pub fn validate_self_client(
        &self,
        client_state_of_host_on_counterparty: ibc_proto::google::protobuf::Any,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    pub fn client_update_time(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::timestamp::Timestamp, ContractError> {
        Ok(Timestamp::none())
    }

    pub fn client_update_height(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::Height, ContractError> {
        Ok(ibc::Height::new(10, 10).unwrap())
    }

    pub fn max_expected_time_per_block(&self) -> std::time::Duration {
        Duration::from_secs(60)
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_client_state(
        &self,
        store: &mut dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        client_state: Vec<u8>,
    ) -> Result<(), ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);

        self.ibc_store()
            .commitments()
            .save(store, client_key, &client_state)?;

        Ok(())
    }

    pub fn store_consensus_state(
        &self,
        store: &mut dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        consensus_state: Vec<u8>,
    ) -> Result<(), ContractError> {
        let consensus_key = commitment::consensus_state_commitment_key(
            client_id,
            height.revision_number(),
            height.revision_height(),
        );

        self.ibc_store()
            .commitments()
            .save(store, consensus_key, &consensus_state)?;

        Ok(())
    }

    //TODO : Implement Methods
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn store_update_time(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        timestamp: ibc::timestamp::Timestamp,
    ) -> Result<(), ContractError> {
        todo!()
    }

    //TODO : Implement Methods
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn store_update_height(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        host_height: ibc::Height,
    ) -> Result<(), ContractError> {
        todo!()
    }
}
