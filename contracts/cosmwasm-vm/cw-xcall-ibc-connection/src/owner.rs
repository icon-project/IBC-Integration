use super::*;

impl<'a> CwIbcConnection<'a> {
    /// This function queries the owner of a smart contract stored in a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `load` method is called on the `owner()`
    /// field of the smart contract, which loads the value of the owner from the storage. The `
    ///
    /// Returns:
    ///
    /// A `Result` containing either a `String` representing the owner of the contract or a `StdError`
    /// if an error occurred while loading the owner from the storage.
    pub fn query_owner(&self, store: &dyn Storage) -> Result<String, StdError> {
        let owner = self.owner().load(store)?;

        Ok(owner)
    }

    /// This function adds an owner to a storage and returns a response with the added owner's
    /// information.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object `dyn Storage`. This is used to
    /// interact with the contract's storage and persist data on the blockchain. The `add_owner`
    /// function is expected to modify the storage by adding a new owner to the contract.
    /// * `owner`: A string representing the address of the new owner to be added to the contract.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>`. If the function executes successfully, it will return an
    /// `Ok` variant containing a `Response` object with some attributes added to it. If there is an
    /// error, it will return an `Err` variant containing a `ContractError` object.
    pub fn add_owner(
        &self,
        store: &mut dyn Storage,
        owner: String,
    ) -> Result<Response, ContractError> {
        match self.owner().may_load(store)? {
            Some(address) => {
                if address != owner {
                    self.owner().save(store, &owner)?;
                } else {
                    return Err(ContractError::OwnerAlreadyExist);
                }
            }
            None => {
                self.owner().save(store, &owner)?;
            }
        };

        Ok(Response::new()
            .add_attribute("method", "add_owner")
            .add_attribute("owner", owner.to_string()))
    }

    /// This function updates the owner of a contract if the sender is the current owner and the new
    /// owner does not already exist.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object `dyn Storage`. This is used to
    /// interact with the contract's storage and persist data between contract executions.
    /// * `info`: `info` is a `MessageInfo` struct that contains information about the message that
    /// triggered the contract execution, such as the sender's address, the amount of tokens
    /// transferred, and the message's ID. It is used to verify that the sender is the current owner of
    /// the contract before updating the owner
    /// * `new_owner`: A String representing the new owner that the current owner is trying to update
    /// to.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>`. If the function executes successfully, it returns a
    /// `Response` object with some attributes added to it. If there is an error, it returns a
    /// `ContractError`.
    pub fn update_owner(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        new_owner: String,
    ) -> Result<Response, ContractError> {
        self.owner()
            .update(store, |mut current_owner| -> Result<_, ContractError> {
                if info.sender == current_owner {
                    if current_owner == new_owner {
                        Err(ContractError::OwnerAlreadyExist)
                    } else {
                        current_owner = new_owner.clone();
                        Ok(current_owner)
                    }
                } else {
                    Err(ContractError::Unauthorized {})
                }
            })?;

        Ok(Response::new()
            .add_attribute("action", "update owner")
            .add_attribute("owner", new_owner.to_string()))
    }
}
