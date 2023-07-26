use common::utils::keccak256;
use cw_common::{get_address_storage_prefix, query_helpers::get_contract_info};
use prost::DecodeError;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This method stores a connection in a storage and returns an error if there is any.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a type that implements the `Storage` trait. This is where the
    /// connection data will be stored.
    /// * `conn_id`: The `conn_id` parameter is of type `ConnectionId` and represents the unique
    /// identifier for the connection being stored.
    /// * `conn_end`: `conn_end` is an instance of the `ConnectionEnd` struct, which represents the end
    /// of a connection in the Inter-Blockchain Communication (IBC) protocol. It contains information
    /// such as the connection state, counterparty endpoint, and associated client identifier.
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` where `()` indicates that the function returns nothing on success,
    /// and `ContractError` is an error type that can be returned in case of an error.
    pub fn store_connection(
        &self,
        store: &mut dyn Storage,
        conn_id: &ConnectionId,
        conn_end: &ConnectionEnd,
    ) -> Result<(), ContractError> {
        let data = conn_end
            .encode_vec()
            .map_err(|error| ConnectionError::Other {
                description: error.to_string(),
            })
            .map_err(Into::<ContractError>::into)?;
        match self.ibc_store().connections().save(store, conn_id, &data) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    /// This method retrieves a connection end from the storage based on a given connection ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an interface
    /// that defines methods for reading and writing data to a persistent storage. The actual
    /// implementation of this trait is provided by the underlying storage engine, which could be a
    /// database, a file system, or any other
    /// * `conn_id`: `conn_id` is a unique identifier for a connection in the IBC (Inter-Blockchain
    /// Communication) protocol. It is used to retrieve information about a specific connection from the
    /// storage.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `ConnectionEnd` object or a `ContractError` if
    /// there was an error while loading or decoding the data.
    pub fn connection_end(
        &self,
        store: &dyn Storage,
        conn_id: &ConnectionId,
    ) -> Result<ConnectionEnd, ContractError> {
        let data = self.ibc_store().connections().load(store, conn_id)?;

        let connection_end =
            ConnectionEnd::decode(&*data).map_err(|error| ContractError::IbcDecodeError {
                error: DecodeError::new(error.to_string()),
            })?;

        Ok(connection_end)
    }

    /// This method stores a connection ID for a given client ID in a storage object.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract.
    /// * `client_id`: The `client_id` parameter is a unique identifier for an IBC client. It is used to
    /// associate a connection with a specific client.
    /// * `conn_id`: `conn_id` is a unique identifier for a connection between two IBC-enabled
    /// blockchains. It is used to store the connection information in the IBC store.
    ///
    /// Returns:
    ///
    /// a `Result` type with either an `Ok(())` value indicating success or an `Err` value with a
    /// `ContractError::Std` variant indicating an error occurred while saving the client connection to
    /// the storage.
    pub fn store_connection_to_client(
        &self,
        store: &mut dyn Storage,
        client_id: &ClientId,
        conn_id: &ConnectionId,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_connections()
            .save(store, client_id, conn_id)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method loads a client's connection from the IBC store using the provided client ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is an abstract
    /// type that represents a key-value store where data can be stored and retrieved. The specific
    /// implementation of the `Storage` trait is provided by the underlying blockchain platform on which
    /// the smart contract is deployed. The
    /// * `client_id`: The `client_id` parameter is an identifier for a specific client in the IBC
    /// (Inter-Blockchain Communication) protocol. It is used to retrieve information about the client's
    /// connections to other blockchains.
    ///
    /// Returns:
    ///
    /// a `Result` object that contains either a `ConnectionId` or a `ContractError`.
    pub fn client_connection(
        &self,
        store: &dyn Storage,
        client_id: &ClientId,
    ) -> Result<ConnectionId, ContractError> {
        self.ibc_store()
            .client_connections()
            .load(store, client_id)
            .map_err(ContractError::Std)
    }
    /// This method is used to increase the connection counter in the IBC store. It takes a mutable
    /// reference to a storage object and returns a `Result` containing the new sequence number or a
    /// `ContractError` if there was an error while updating the counter.
    pub fn increase_connection_counter(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u64, ContractError> {
        let sequence_no = self.ibc_store().next_connection_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        )?;

        Ok(sequence_no)
    }

    /// This method returns the next connection sequence number from the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to interact
    /// with the storage of the smart contract. The `connection_counter` function takes a reference to
    /// `store` as an argument so that it can load the next connection sequence from the storage.
    ///
    /// Returns:
    ///
    /// a `Result<u64, ContractError>`. If the `next_connection_sequence()` method call on the
    /// `ibc_store()` returns an `Ok` value, then the function returns an `Ok` result with the value. If
    /// the method call returns an `Err` value, then the function returns an `Err` result with a
    /// `ContractError` containing the error.
    pub fn connection_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_connection_sequence().load(store) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method initializes the next sequence number for a connection in a Rust-based IBC
    /// implementation.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `dyn` keyword indicates that `Storage`
    /// is a dynamic trait object, which means that it can be used to refer to any type that implements
    /// * `sequence`: The `sequence` parameter is an unsigned 64-bit integer that represents the next
    /// sequence number for a connection in the IBC (Inter-Blockchain Communication) protocol. This
    /// function initializes the next connection sequence number in the storage provided by the `store`
    /// parameter.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either `Ok(())` if the `sequence` was successfully saved
    /// in the `store`, or `Err(ContractError::Std(error))` if there was an error while saving the
    /// `sequence`.
    pub fn connection_next_sequence_init(
        &self,
        store: &mut dyn Storage,
        sequence: u64,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_connection_sequence()
            .save(store, &sequence)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method initializes a connection counter and saves it to a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data on the blockchain. The
    /// `init_connection_counter` function takes this as a parameter so that it can save the
    /// `sequence_no` value to the storage
    /// * `sequence_no`: `sequence_no` is an unsigned 64-bit integer that represents the sequence number
    /// of the next connection. It is used to initialize the connection counter in the IBC
    /// (Inter-Blockchain Communication) module.
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` where `()` indicates that the function returns nothing on success
    /// and `ContractError` is an error type that can be returned in case of an error.
    pub fn init_connection_counter(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_connection_sequence()
            .save(store, &sequence_no)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    /// This method checks if a connection already exists for a given client ID in a storage and
    /// returns an error if it does, otherwise it returns Ok.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract.
    /// * `client_id`: The `client_id` parameter is the identifier of the client for which the function
    /// is checking if a connection already exists.
    ///
    /// Returns:
    ///
    /// a `Result` with either `Ok(())` if there is no existing connection for the given `client_id`, or
    /// an `Err` with a `ContractError` if there is an error or if a connection already exists for the
    /// given `client_id`.
    pub fn check_for_connection(
        &self,
        store: &dyn Storage,
        client_id: &ClientId,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_connections()
            .may_load(store, client_id)
        {
            Ok(result) => match result {
                Some(id) => Err(ConnectionError::ConnectionExists(id.to_string()))
                    .map_err(Into::<ContractError>::into),
                None => Ok(()),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This method updates the commitment of a connection in a storage using the provided connection
    /// ID and connection end.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract.
    /// * `connection_id`: The identifier of the connection being updated.
    /// * `connection_end`: `connection_end` is an instance of the `ConnectionEnd` struct, which
    /// represents the end of a connection on a chain. It contains information such as the client ID,
    /// the counterparty connection ID, the connection state, and the version.
    ///
    /// Returns:
    ///
    /// a `Result` with an empty `Ok(())` value if the operation is successful, or a `ContractError` if
    /// there is an error during the operation.
    pub fn update_connection_commitment(
        &self,
        store: &mut dyn Storage,
        connection_id: &ConnectionId,
        connection_end: &ConnectionEnd,
    ) -> Result<(), ContractError> {
        let connection_commit_key = commitment::connection_commitment_key(connection_id);

        let connection_end_bytes =
            connection_end
                .encode_vec()
                .map_err(|error| ContractError::IbcConnectionError {
                    error: ConnectionError::Other {
                        description: error.to_string(),
                    },
                })?;

        let commitment_bytes = keccak256(&connection_end_bytes).to_vec();

        self.ibc_store()
            .save_commitment(store, connection_commit_key, &commitment_bytes)?;

        Ok(())
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn commitment_prefix(&self, deps: Deps, env: &Env) -> CommitmentPrefix {
        let address = self.get_self_address(deps, env);
        let prefix = get_address_storage_prefix(&address, StorageKey::Commitments.as_str());
        CommitmentPrefix::try_from(prefix).unwrap_or_default() //TODO
    }

    fn get_self_address(&self, deps: Deps, env: &Env) -> String {
        let addr = env.contract.address.to_string();

        if addr.contains("contract") {
            let info = get_contract_info(deps, addr).unwrap();
            return info.admin.unwrap();
        }
        addr
    }
}
