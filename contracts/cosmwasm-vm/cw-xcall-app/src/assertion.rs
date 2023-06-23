use super::*;

use crate::{
    error::ContractError,
    state::{CwCallService, MAX_DATA_SIZE, MAX_ROLLBACK_SIZE},
    types::{call_request::CallRequest, request::CallServiceMessageRequest},
};

impl<'a> CwCallService<'a> {
    /// This function checks if the caller is a contract and if the rollback option is null, and returns
    /// an error if the rollback is not possible.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an object that contains dependencies required by the contract to interact
    /// with the blockchain. It is of type `Deps`, which is a struct that contains various modules such
    /// as `storage`, `querier`, `api`, etc.
    /// * `address`: The address of the caller that needs to be checked if it is a contract or not.
    /// * `rollback`: `rollback` is an optional `Vec<u8>` parameter that represents the rollback data. If
    /// it is `Some`, it means that a rollback is possible and the caller must be a contract. If it is
    /// `None`, it means that a rollback is not possible and the caller can be any type
    ///
    /// Returns:
    ///
    /// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
    /// containing a `ContractError` if the condition in the `ensure!` macro is not met.
    pub fn ensure_caller_is_contract_and_rollback_is_null(
        &self,
        deps: Deps,
        address: Addr,
        rollback: Option<Vec<u8>>,
    ) -> Result<(), ContractError> {
        if rollback.is_some() {
            ensure!(
                is_contract(deps.querier, address),
                ContractError::RollbackNotPossible
            );
        }

        Ok(())
    }

    /// This function ensures that the length of the data is not greater than the maximum allowed size and
    /// returns an error if it is.
    ///
    /// Arguments:
    ///
    /// * `data_len`: `data_len` is a variable of type `usize` that represents the length of some data. It
    /// is used as a parameter in the `ensure_data_length` function to check if the length of the data is
    /// within the maximum allowed size. If the length of the data exceeds the maximum size,
    ///
    /// Returns:
    ///
    /// The `ensure_data_length` function returns a `Result` type with the success case containing an empty
    /// tuple `()` and the error case containing a `ContractError`.
    pub fn ensure_data_length(&self, data_len: usize) -> Result<(), ContractError> {
        ensure!(
            data_len <= MAX_DATA_SIZE as usize,
            ContractError::MaxDataSizeExceeded
        );

        Ok(())
    }
    /// This function ensures that the length of a given byte array (rollback) is not greater than a
    /// specified maximum size.
    ///
    /// Arguments:
    ///
    /// * `rollback`: `rollback` is a slice of bytes (`&[u8]`) that represents the data to be rolled back in
    /// a smart contract. The function `ensure_rollback_length` checks if the length of the `rollback` slice
    /// is within the maximum allowed size (`MAX_ROLLBACK_SIZE`) and
    ///
    /// Returns:
    ///
    /// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
    /// containing a `ContractError` if the condition in the `ensure!` macro is not met.

    pub fn ensure_rollback_length(&self, rollback: &[u8]) -> Result<(), ContractError> {
        ensure!(
            rollback.is_empty() || rollback.len() <= MAX_ROLLBACK_SIZE as usize,
            ContractError::MaxRollbackSizeExceeded
        );

        Ok(())
    }

    /// The function ensures that the request message is not null and returns an error if it is.
    ///
    /// Arguments:
    ///
    /// * `req_id`: A unique identifier for the request being made.
    /// * `message`: `message` is a reference to a `CallServiceMessageRequest` struct. This struct likely
    /// contains information about a request to call a service, such as the name of the service, the input
    /// parameters, and any other relevant data. The function is checking to make sure that this message is
    /// not null
    ///
    /// Returns:
    ///
    /// a `Result` with either an `Ok(())` if the `data` is not empty, or a
    /// `ContractError::InvalidRequestId` with the given `req_id` if the `data` is empty.
    pub fn ensure_request_not_null(
        &self,
        req_id: u128,
        message: &CallServiceMessageRequest,
    ) -> Result<(), ContractError> {
        let data = to_binary(message).unwrap();
        ensure!(
            !(data.is_empty()),
            ContractError::InvalidRequestId { id: req_id }
        );

        Ok(())
    }

    /// This function ensures that a call request message is not null and returns an error if it is.
    ///
    /// Arguments:
    ///
    /// * `sequence_no`: an unsigned 128-bit integer representing the sequence number of a call request.
    /// * `message`: The `message` parameter is a reference to a `CallRequest` struct. It is used to ensure
    /// that the `data` field of the `CallRequest` is not empty. If it is empty, it will return an error
    /// indicating an invalid sequence ID.
    ///
    /// Returns:
    ///
    /// a `Result` enum with either an `Ok(())` value indicating that the call request is not null, or an
    /// `Err(ContractError)` value indicating that the call request is null and providing an error message.
    pub fn ensure_call_request_not_null(
        &self,
        sequence_no: u128,
        message: &CallRequest,
    ) -> Result<(), ContractError> {
        let data = to_binary(message).unwrap();
        ensure!(
            !(data.is_empty()),
            ContractError::InvalidSequenceId { id: sequence_no }
        );

        Ok(())
    }
    /// This function checks if rollback is enabled and returns an error if it is not.
    ///
    /// Arguments:
    ///
    /// * `enabled`: A boolean value indicating whether rollback is enabled or not. If it is not enabled,
    /// the function will return a `ContractError` with the message "RollbackNotEnabled".
    ///
    /// Returns:
    ///
    /// The function `ensure_rollback_enabled` is returning a `Result` type with the `Ok` variant containing
    /// an empty tuple `()` if the `enabled` parameter is `true`, and a `ContractError` with the
    /// `RollbackNotEnabled` variant if the `enabled` parameter is `false`.

    pub fn ensure_rollback_enabled(&self, enabled: bool) -> Result<(), ContractError> {
        ensure!(enabled, ContractError::RollbackNotEnabled);

        Ok(())
    }

    /// This function checks if the sender of a message is the owner of a contract.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to the storage implementation that the contract is using. It is
    /// used to load and store data on the blockchain. In this function, it is used to load the current
    /// owner of the contract from storage.
    /// * `info`: `info` is a reference to a `MessageInfo` struct which contains information about the
    /// current message being processed by the contract. This includes the sender of the message, the amount
    /// of funds attached to the message, and other metadata. The `ensure_owner` function uses the `sender`
    /// field of
    ///
    /// Returns:
    ///
    /// The `ensure_owner` function is returning a `Result` type with the `Ok` variant containing an empty
    /// tuple `()` if the `info.sender` matches the owner loaded from storage, and a `ContractError` if the
    /// sender is not authorized.
    pub fn ensure_owner(
        &self,
        store: &dyn Storage,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        let owner = self.owner().load(store)?;

        ensure_eq!(info.sender, owner, ContractError::Unauthorized {});

        Ok(())
    }
    /// The function ensures that the given address is the admin of the contract.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is used to interact
    /// with the contract's storage and retrieve data from it. The `ensure_admin` function uses `store` to
    /// query the current admin address stored in the contract's storage.
    /// * `address`: `address` is a variable of type `Addr` that represents the address of a user or
    /// contract. It is used as an argument in the `ensure_admin` function to check if the address matches
    /// the admin address stored in the contract's storage. If the addresses do not match, the function
    /// returns
    ///
    /// Returns:
    ///
    /// a `Result` with either an `Ok(())` value indicating that the `address` parameter matches the stored
    /// `admin` value, or a `ContractError` value with the message "OnlyAdmin" if the `address` parameter
    /// does not match the stored `admin` value.
    pub fn ensure_admin(&self, store: &dyn Storage, address: Addr) -> Result<(), ContractError> {
        let admin = self.query_admin(store)?;
        ensure_eq!(admin, address, ContractError::OnlyAdmin);

        Ok(())
    }
    /// The function ensures that the given address is the IBC handler.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to interact
    /// with the contract's storage and retrieve data from it. The `Storage` trait defines methods for
    /// getting and setting key-value pairs in the contract's storage.
    /// * `address`: The `address` parameter is of type `Addr` and represents the address of the IBC handler
    /// that needs to be checked against the stored IBC host address.
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` which means it can either return an `Ok(())` indicating that the
    /// function executed successfully or an `Err(ContractError)` indicating that an error occurred during
    /// execution.
    pub fn ensure_connection_handler(
        &self,
        store: &dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        let connections = self.get_all_connections(store)?;

        if !connections.contains(&address.to_string()) {
            return Err(ContractError::OnlyIbcHandler {});
        }
        Ok(())
    }

    pub fn ensure_enough_funds(
        &self,
        required_fee: u128,
        info: &MessageInfo,
    ) -> Result<(), ContractError> {
        let total_funds: u128 = info
            .funds
            .iter()
            .map(|c| c.amount.into())
            .collect::<Vec<u128>>()
            .iter()
            .sum();
        ensure!(total_funds >= required_fee, ContractError::InsuffcientFunds);
        Ok(())
    }
}

/// The function checks if a given address is a valid smart contract by querying its information using a
/// QuerierWrapper.
///
/// Arguments:
///
/// * `querier`: The `querier` parameter is an instance of the `QuerierWrapper` struct, which is used to
/// query information from the blockchain. It provides methods to query account balances, contract
/// state, and other information related to the blockchain.
/// * `address`: The `address` parameter is a variable of type `Addr` which represents the address of a
/// smart contract on the blockchain.
///
/// Returns:
///
/// The function `is_contract` returns a boolean value indicating whether the given address is a valid
/// smart contract on the blockchain or not. It does this by querying the blockchain through the
/// `querier` object to get information about the contract at the given `address`. If the query is
/// successful, it returns `true`, indicating that the address is a valid contract. If the query fails,
/// it returns `
fn is_contract(querier: QuerierWrapper, address: Addr) -> bool {
    querier.query_wasm_contract_info(address).is_ok()
}
