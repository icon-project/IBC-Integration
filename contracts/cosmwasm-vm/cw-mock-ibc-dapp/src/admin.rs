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
    pub fn query_admin(&self, store: &dyn Storage) -> Result<String, ContractError> {
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
    /// returns an `Err` with
    pub fn add_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        admin: String,
    ) -> Result<Response, ContractError> {
        if admin.is_empty() {
            return Err(ContractError::AdminAddressCannotBeNull {});
        }

        let owner = self
            .owner()
            .load(store)
            .map_err(|_| ContractError::Unauthorized {})?;

        if info.sender != owner {
            return Err(ContractError::Unauthorized {});
        }
        self.admin().save(store, &admin)?;
        Ok(Response::new()
            .add_attribute("method", "add_admin")
            .add_attribute("admin", admin.to_string()))
    }

    /// This function updates the admin address of a contract if the caller is the owner and the new
    /// address is valid.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a trait object of type `dyn Storage`. This is used to interact
    /// with the contract's storage.
    /// * `info`: MessageInfo is a struct that contains information about the message being executed, such
    /// as the sender's address, the amount of tokens being sent, and the gas limit. It is used to ensure
    /// that only authorized parties can execute certain functions and to handle payment transactions.
    /// * `new_admin`: A string representing the new address of the admin that will replace the current
    /// admin.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>`. If the function executes successfully, it returns a `Response`
    /// object with attributes "action" and "admin". If there is an error, it returns a `ContractError`
    /// object with a specific error message.
    pub fn update_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        new_admin: String,
    ) -> Result<Response, ContractError> {
        if new_admin.is_empty() {
            return Err(ContractError::AdminAddressCannotBeNull {});
        }

        if !new_admin.to_string().chars().all(|x| x.is_alphanumeric()) {
            return Err(ContractError::InvalidAddress {
                address: new_admin.to_string(),
            });
        }

        self.ensure_owner(store, &info)?;

        self.admin()
            .update(store, |mut current_admin| -> Result<_, ContractError> {
                if current_admin == new_admin {
                    Err(ContractError::AdminAlreadyExist)
                } else {
                    current_admin = new_admin.clone();
                    Ok(current_admin)
                }
            })?;

        Ok(Response::new()
            .add_attribute("action", "update admin")
            .add_attribute("admin", new_admin.to_string()))
    }

    /// The code defines a function to remove an admin and another function to validate an address.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// interact with the contract's storage and modify its state.
    /// * `info`: `info` is a parameter of type `MessageInfo` which contains information about the message
    /// being executed, such as the sender's address, the amount of coins being sent, and the gas limit. It
    /// is used in the `remove_admin` function to ensure that the sender is the owner of the
    ///
    /// Returns:
    ///
    /// The `remove_admin` function returns a `Result<Response, ContractError>` and the `validate_address`
    /// function returns a `Result<String, ContractError>`.
    pub fn remove_admin(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        self.ensure_owner(store, &info)?;

        self.admin().remove(store);
        Ok(Response::new().add_attribute("method", "remove_admin"))
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
