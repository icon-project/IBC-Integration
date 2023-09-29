use cosmwasm_std::{Addr, Api, MessageInfo, Response, Storage};

use crate::{error::ContractError, state::CwIbcConnection};

impl<'a> CwIbcConnection<'a> {
    /// This function queries the admin of a smart contract from the storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. This is used to
    /// interact with the contract's storage, which is where data is stored permanently on the
    /// blockchain. The `query_admin` function uses the `store` parameter to load the current admin
    /// address from storage and return
    ///
    /// Returns:
    ///
    /// The function `query_admin` returns a `Result` containing either a `String` representing the admin
    /// address if it exists in the storage or a `ContractError` if it does not exist.
    pub fn query_admin(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        let admin = self
            .admin()
            .load(store)
            .map_err(|_| ContractError::AdminNotExist)?;

        Ok(admin)
    }

    /// This function adds an admin to the contract if the sender is the owner and the admin does not
    /// already exist.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data on the blockchain.
    /// * `info`: `info` is a parameter of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender's address, the amount of coins being sent, and the gas limit.
    /// This parameter is used to check if the sender is authorized to add an admin.
    /// * `admin`: A string representing the address of the new admin to be added to the contract.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>`. If the `admin` parameter is empty, it returns an `Err` with
    /// the `ContractError::AdminAddressCannotBeNull` variant. If the `info.sender` is not the owner, it
    /// returns an `Err` with the `ContractError::Unauthorized` variant. If an admin already exists, it
    /// returns an `Err`
    pub fn add_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        admin: Addr,
    ) -> Result<Response, ContractError> {
        self.admin().save(store, &admin)?;
        Ok(Response::new()
            .add_attribute("method", "add_admin")
            .add_attribute("admin", admin.to_string()))
    }

    pub fn validate_address(api: &dyn Api, address: &str) -> Result<Addr, ContractError> {
        if !address.chars().all(|x| x.is_alphanumeric()) {
            return Err(ContractError::InvalidAddress {
                address: address.to_string(),
            });
        }

        let validated_address = api.addr_validate(address).map_err(ContractError::Std)?;

        Ok(validated_address)
    }
}
