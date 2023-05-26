use cosmwasm_std::{ensure, ensure_eq, Addr, MessageInfo, Storage};

use crate::{
    error::ContractError,
    state::{CwIbcConnection, MAX_DATA_SIZE, MAX_ROLLBACK_SIZE},
    types::LOG_PREFIX,
};

impl<'a> CwIbcConnection<'a> {
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
    pub fn ensure_ibc_handler(
        &self,
        store: &dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        let ibc_host = self.get_ibc_host(store)?;

        if ibc_host != address {
            println!("{LOG_PREFIX} Invalid IBC Handler ");
            return Err(ContractError::OnlyIbcHandler {});
        }
        Ok(())
    }

    pub fn ensure_xcall_handler(
        &self,
        store: &dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        let ibc_host = self.get_xcall_host(store)?;

        if ibc_host != address {
            println!("{LOG_PREFIX} Invalid Xcall Handler ");
            return Err(ContractError::OnlyIbcHandler {});
        }
        Ok(())
    }
}
