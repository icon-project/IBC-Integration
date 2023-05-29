use super::*;
/// This is an implementation of two methods for the `CwCallService` struct.

impl<'a> CwIbcConnection<'a> {
    /// The `set_protocol_fee` function sets the protocol fee and the `get_protocol_fee` function
    /// retrieves the current protocol fee.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` or `Deps` object that provides access to the contract's
    /// dependencies such as storage, API, and other modules. `DepsMut` is used when the function needs
    /// to modify the state of the contract, while `Deps` is used
    /// * `info`: MessageInfo is a struct that contains information about the message being executed,
    /// such as the sender's address, the amount of coins being sent, and the gas limit. It is provided
    /// by the Cosmos SDK to the contract's entry points.
    /// * `value`: The `value` parameter in both functions represents the amount of protocol fee to be
    /// set or retrieved. It is of type `u128`, which means it can hold a large unsigned integer value.
    /// The protocol fee is a fee charged by the contract for executing certain operations or
    /// transactions on the blockchain.
    ///
    /// Returns:
    ///
    /// The `set_protocol_fee` function returns a `Result<Response, ContractError>` which contains a
    /// `Response` object with an added attribute "method" set to "set_protocolfee". The
    /// `get_protocol_fee` function returns a `u128` value which represents the current protocol fee.
    pub fn set_protocol_fee(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        value: u128,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        self.add_fee(deps.storage, value)?;

        Ok(Response::new().add_attribute("method", "set_protocolfee"))
    }

    pub fn get_protocol_fee(&self, deps: Deps) -> Result<u128, ContractError> {
        self.query_fee(deps.storage)
    }
}

impl<'a> CwIbcConnection<'a> {
    /// This function adds a fee value to the contract's storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and save the updated fee value. The `dyn` keyword
    /// indicates that the type of `store` is not known at compile time and will be determined at
    /// runtime.
    /// * `value`: `value` is a variable of type `u128`, which represents an unsigned 128-bit integer.
    /// It is the amount of fee that is being added to the contract's state.
    ///
    /// Returns:
    ///
    /// The `add_fee` function returns a `Result` type with either an `Ok(())` value indicating that the
    /// fee was successfully added to the storage, or an `Err` value with a `ContractError::Std` variant
    /// indicating that an error occurred while trying to save the fee value to the storage.
    pub fn add_fee(&self, store: &mut dyn Storage, value: u128) -> Result<(), ContractError> {
        match self.fee().save(store, &value) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This function queries the fee value stored in the contract's storage and returns it as a result.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage, which is a key-value store that persists data on the
    /// blockchain. The `query_fee` function reads the value of the `fee` key from the storage and
    /// returns
    ///
    /// Returns:
    ///
    /// The `query_fee` function returns a `Result` that contains either a `u128` value representing the
    /// fee or a `ContractError` if there was an error while loading the fee from the storage.
    fn query_fee(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        match self.fee().load(store) {
            Ok(value) => Ok(value),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
