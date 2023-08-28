use common::ibc::core::ics24_host::identifier::ConnectionId;
use common::icon::icon::lightclient::v1::ClientState;
use common::icon::icon::lightclient::v1::ConsensusState;
use common::traits::AnyTypes;
use common::{client_state::IClientState, consensus_state::IConsensusState};
use cosmwasm_std::Deps;
use cosmwasm_std::Env;
use prost::DecodeError;
use prost::Message;

use crate::ics24_host::LastProcessedOn;
use crate::light_client::light_client::LightClient;

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
        client_id: &ClientId,
    ) -> Result<IbcClientType, ContractError> {
        self.ibc_store()
            .client_types()
            .load(store, client_id)
            .map_err(|_e| ContractError::InvalidClientId {
                client_id: client_id.as_str().to_string(),
            })
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
        client_type: IbcClientType,
    ) -> Result<String, ContractError> {
        self.ibc_store()
            .client_registry()
            .load(store, client_type.clone())
            .map_err(|_e| ContractError::InvalidClientType {
                client_type: client_type.as_str().to_string(),
            })
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
        client_id: &ClientId,
    ) -> Result<LightClient, ContractError> {
        self.ibc_store()
            .client_implementations()
            .load(store, client_id)
            .map_err(|_e| ContractError::InvalidClientId {
                client_id: client_id.as_str().to_string(),
            })
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
        client_id: &ClientId,
        client_type: IbcClientType,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .client_types()
            .save(store, client_id, &client_type)
            .map_err(ContractError::Std)
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
        client_type: IbcClientType,
        client: String,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .client_registry()
            .save(store, client_type, &client)
            .map_err(ContractError::Std)
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
        client_id: &ClientId,
        client: LightClient,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .client_implementations()
            .save(store, client_id, &client)
            .map_err(ContractError::Std)
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
        client_type: IbcClientType,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .may_load(store, client_type)
        {
            Ok(result) => match result {
                Some(_) => Err(ClientError::Other {
                    description: "Client Implementation Already Exist".to_string(),
                })
                .map_err(Into::<ContractError>::into),

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
        client_id: &ClientId,
    ) -> Result<LightClient, ContractError> {
        let client = self.get_client_implementations(store, client_id).ok();

        if client.is_none() {
            return Err(ClientError::ClientNotFound {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }
        Ok(client.unwrap())
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
        client_id: &ClientId,
    ) -> Result<Vec<u8>, ContractError> {
        let client_state = self
            .ibc_store()
            .client_states()
            .load(store, client_id)
            .map_err(|_| ContractError::IbcDecodeError {
                error: DecodeError::new("NotFound ClientId(".to_owned() + client_id.as_str() + ")"),
            })?;

        Ok(client_state)
    }

    /// This method retrieves the commitment from storage using the commitment key (PacketCommitment,AckCommitment,Connection,Channel,Client).
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be stored and retrieved. The specific
    /// implementation of the `Storage` trait is not specified in this function, allowing for
    /// flexibility in choosing the type of storage
    /// * `key`: The `key` parameter is of type `Vec<u8>` and represents commitment key of
    /// an IBC client. It is used to retrieve the client state from the storage.
    ///
    /// Returns:
    ///
    /// a `Result` containing either a `Vec<u8>` representing the client state or a `ContractError` if
    /// there was an error loading the client state from the storage.
    pub fn get_commitment(
        &self,
        store: &dyn Storage,
        key: Vec<u8>,
    ) -> Result<Vec<u8>, ContractError> {
        let commitment = self
            .ibc_store()
            .load_commitment(store, key)
            .ok_or(ContractError::InvalidCommitmentKey)?;
        Ok(commitment)
    }

    /// This method retrieves the connection from storage using the connection id.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be stored and retrieved. The specific
    /// implementation of the `Storage` trait is not specified in this function, allowing for
    /// flexibility in choosing the type of storage
    /// * `connection_id`: The `connection_id` parameter is of type `String` and represents connection key of
    /// an IBC client. It is used to retrieve the client state from the storage.
    ///
    /// Returns:
    ///
    /// a `Result` containing either a `Vec<u8>` representing the client state or a `ContractError` if
    /// there was an error loading the client state from the storage.
    pub fn get_connection(
        &self,
        store: &dyn Storage,
        connection_id: &ConnectionId,
    ) -> Result<Vec<u8>, ContractError> {
        let _connection_id = connection_id.clone();
        let connection = self
            .ibc_store()
            .connections()
            .load(store, connection_id)
            .map_err(|_| ContractError::InvalidConnectiontId {
                connection_id: _connection_id.as_str().to_string(),
            })?;
        Ok(connection)
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn client_state(
        &self,
        store: &dyn Storage,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Box<dyn IClientState>, ContractError> {
        let client_state_any = self.client_state_any(store, client_id)?;

        let client_state =
            ClientState::from_any(client_state_any).map_err(Into::<ContractError>::into)?;

        Ok(Box::new(client_state))
    }

    pub fn client_state_any(
        &self,
        store: &dyn Storage,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Any, ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);

        let client_state_any_data = self.ibc_store().client_states().load(store, client_id)?;
        let client_state_any =
            Any::decode(client_state_any_data.as_slice()).map_err(Into::<ContractError>::into)?;
        Ok(client_state_any)
    }

    pub fn consensus_state(
        &self,
        deps: Deps,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
        height: &common::ibc::Height,
    ) -> Result<Box<dyn IConsensusState>, ContractError> {
        let client_impl = self.get_client(deps.storage, &client_id)?;
        let height = height.revision_height();
        return client_impl.get_consensus_state(deps, client_id, height);
    }

    pub fn consensus_state_any(
        &self,
        store: &dyn Storage,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Any, ContractError> {
        let consensus_state_data = self.ibc_store().consensus_states().load(store, client_id)?;
        let consensus_state_any =
            Any::decode(consensus_state_data.as_slice()).map_err(Into::<ContractError>::into)?;
        Ok(consensus_state_any)
    }

    pub fn host_height(&self, env: &Env) -> Result<common::ibc::Height, ContractError> {
        let height = env.block.height;
        let height = common::ibc::Height::new(0, height).map_err(Into::<ContractError>::into)?;
        Ok(height)
    }

    pub fn host_timestamp(
        &self,
        env: &Env,
    ) -> Result<common::ibc::timestamp::Timestamp, ContractError> {
        let current_timestamp = env.block.time;
        IbcTimestamp::from_nanoseconds(current_timestamp.nanos())
            .map_err(|_e| ContractError::FailedConversion)
    }

    pub fn validate_self_client(
        &self,
        client_state_of_host_on_counterparty: Any,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    pub fn client_update_time(
        &self,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
        time_nanos: u64,
    ) -> Result<common::ibc::timestamp::Timestamp, ContractError> {
        Ok(IbcTimestamp::from_nanoseconds(time_nanos).unwrap())
    }

    pub fn client_update_height(
        &self,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
        height: u64,
    ) -> Result<common::ibc::Height, ContractError> {
        let height = common::ibc::Height::new(0, height).map_err(Into::<ContractError>::into)?;
        Ok(height)
    }

    pub fn last_processed_on(
        &self,
        store: &dyn Storage,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<LastProcessedOn, ContractError> {
        return self
            .ibc_store()
            .last_processed_on()
            .load(store, client_id)
            .map_err(Into::<ContractError>::into);
    }

    pub fn max_expected_time_per_block(&self) -> std::time::Duration {
        Duration::from_secs(60)
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_client_state(
        &self,
        store: &mut dyn Storage,
        env: &Env,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
        client_state_any: Vec<u8>,
        client_state_hash: Vec<u8>,
    ) -> Result<(), ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);
        self.ibc_store()
            .client_states()
            .save(store, client_id, &client_state_any)?;

        self.ibc_store()
            .save_commitment(store, client_key, &client_state_hash)?;
        self.store_last_processed_on(store, env, client_id)?;

        Ok(())
    }

    pub fn store_consensus_state(
        &self,
        store: &mut dyn Storage,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
        height: common::ibc::Height,
        consensus_state_any: Vec<u8>,
        consensus_state_hash: Vec<u8>,
    ) -> Result<(), ContractError> {
        let consensus_key = commitment::consensus_state_commitment_key(
            client_id,
            height.revision_number(),
            height.revision_height(),
        );

        self.ibc_store()
            .consensus_states()
            .save(store, client_id, &consensus_state_any)?;

        self.ibc_store()
            .save_commitment(store, consensus_key, &consensus_state_hash)?;

        Ok(())
    }

    pub fn store_last_processed_on(
        &self,
        store: &mut dyn Storage,
        env: &Env,
        client_id: &common::ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<(), ContractError> {
        let last_processed = LastProcessedOn {
            height: env.block.height,
            timestamp: env.block.time.nanos(),
        };
        self.ibc_store()
            .last_processed_on()
            .save(store, client_id, &last_processed)?;
        Ok(())
    }
}
