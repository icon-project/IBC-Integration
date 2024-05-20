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
    pub fn query_owner(&self, store: &dyn Storage) -> Result<Addr, StdError> {
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
        owner: Addr,
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
}
