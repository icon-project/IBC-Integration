use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function stores a capability in a storage using a given name and address.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// store the capability information in the contract's storage. The `dyn Storage` trait provides a
    /// common interface for different storage implementations, such as in-memory or on-disk storage.
    /// * `name`: A vector of bytes representing the name of the capability to be stored in the storage.
    /// * `address`: The `address` parameter is a vector of strings representing the addresses of the
    /// capabilities being stored. It is used in conjunction with the `name` parameter to save the
    /// capabilities in the storage.
    ///
    /// Returns:
    ///
    /// This function returns a `Result<(), ContractError>` where `()` indicates that the function
    /// returns no meaningful value on success, and `ContractError` is an error type that can be returned
    /// if an error occurs during the execution of the function.
    pub fn store_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: Vec<String>,
    ) -> Result<(), ContractError> {
        match self.ibc_store().capabilities().save(store, name, &address) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: format!("FailedToStore {}", error),
            }),
        }
    }

    /// This function retrieves a capability from the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `dyn` keyword indicates that `Storage` is a
    /// trait, which means that it defines a set of methods that must be implemented by any type
    /// * `name`: `name` is a `Vec<u8>` type parameter representing the name of the capability to be
    /// retrieved from the storage. It is used as a key to load the capability from the storage.
    ///
    /// Returns:
    ///
    /// a `Result` object that contains either a `Vec<String>` or a `ContractError`. The `Vec<String>`
    /// contains the capabilities associated with the given `name` that are loaded from the `store`. If
    /// the capabilities are not found, a `ContractError` with the message "CapabilityNotFound" is
    /// returned.
    pub fn get_capability(
        &self,
        store: &dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        self.ibc_store()
            .capabilities()
            .load(store, name)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "CapabilityNotFound".into(),
            })
    }
    /// This function sets the expected time per block in a storage using the given value.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract and save the `expected_time_per_block` value.
    /// * `expected_time_per_block`: The expected time per block is a parameter that determines the
    /// average time it takes for a new block to be added to the blockchain. It is usually measured in
    /// seconds or minutes. This function sets the expected time per block in the storage of the
    /// contract.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with an empty tuple `()` as the success value and a
    /// `ContractError` as the error value.
    pub fn set_expected_time_per_block(
        &self,
        store: &mut dyn Storage,
        expected_time_per_block: u64,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .expected_time_per_block()
            .save(store, &expected_time_per_block)?;

        Ok(())
    }

    /// This function retrieves the expected time per block from the IBC store and returns it as a result
    /// or throws an error if it is not found.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to access
    /// the key-value store of the contract's state. The `may_load` method is called on the
    /// `expected_time_per_block()` value of the `ibc_store()` method, which returns an
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either the expected time per block as a `u64` or a
    /// `ContractError` if the expected time per block is not found in the provided `store`.
    pub fn get_expected_time_per_block(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().expected_time_per_block().may_load(store)? {
            Some(time) => Ok(time),
            None => Err(ContractError::IbcDecodeError {
                error: "NotFound".to_string(),
            }),
        }
    }

    /// The function updates the capabilities of a store by adding an address to a list of values
    /// associated with a given name.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract.
    /// * `name`: A vector of bytes representing the name of the capability being claimed.
    /// * `address`: The `address` parameter is a `String` representing the address of the capability
    /// being claimed.
    ///
    /// Returns:
    ///
    /// a `Result` with the `Ok` variant containing an empty tuple `()` if the function executes
    /// successfully, and the `Err` variant containing a `ContractError` if an error occurs during
    /// execution.
    pub fn claim_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: String,
    ) -> Result<(), ContractError> {
        self.ibc_store().capabilities().update(
            store,
            name,
            |update| -> Result<_, ContractError> {
                match update {
                    Some(mut value) => {
                        if value.contains(&address) {
                            return Err(ContractError::IbcContextError {
                                error: "Capability already claimed".to_string(),
                            });
                        }
                        value.push(address);
                        Ok(value)
                    }
                    None => Err(ContractError::IbcDecodeError {
                        error: "KeyNotFound".into(),
                    }),
                }
            },
        )?;

        Ok(())
    }

    /// The function checks if the caller has a specific capability stored in the provided storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the persistent storage of the smart contract. The `dyn` keyword indicates that
    /// `Storage` is a dynamic trait object, which means that it can be used to refer to any type that
    /// * `info`: `info` is a parameter of type `MessageInfo` which contains information about the
    /// current message being processed, such as the sender's address and the amount of tokens attached
    /// to the message. It is used to identify the caller of the function.
    /// * `name`: `name` is a `Vec<u8>` parameter representing the name of the capability that needs to
    /// be authenticated.
    ///
    /// Returns:
    ///
    /// A boolean value is being returned. If the `capability` contains the `caller`, then `true` is
    /// returned, otherwise `false` is returned.
    pub fn authenticate_capability(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        name: Vec<u8>,
    ) -> bool {
        let caller = info.sender.to_string();
        let capability = self.get_capability(store, name).unwrap();
        if capability.contains(&caller) {
            return true;
        }
        false
    }

    /// This function looks up modules in a storage and returns their capabilities.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// interact with the contract's storage and retrieve data from it. The `dyn` keyword indicates that
    /// the type of the object implementing the `Storage` trait is not known at compile time and will be
    /// determined
    /// * `name`: `name` is a `Vec<u8>` which represents the name of the module being looked up. It is a
    /// byte vector because module names are typically represented as bytes in Rust.
    ///
    /// Returns:
    ///
    /// If `capabilities.len()` is equal to 0, then an `Err` variant of `ContractError::Unauthorized` is
    /// returned. Otherwise, an `Ok` variant containing the `capabilities` vector is returned.
    pub fn lookup_modules(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        let capabilities = self.get_capability(store, name)?;
        if capabilities.len() == 0 {
            return Err(ContractError::Unauthorized {});
        }
        Ok(capabilities)
    }

    /// This function calculates the delay time for a block based on the maximum expected time per block
    /// and the delay period time.
    ///
    /// Arguments:
    ///
    /// * `delay_period_time`: `delay_period_time` is a `Duration` representing the time period for which
    /// the delay needs to be calculated.
    ///
    /// Returns:
    ///
    /// an unsigned 64-bit integer representing the calculated delay in seconds.
    pub fn calc_block_delay(&self, delay_period_time: &Duration) -> u64 {
        let max_expected_time_per_block = self.max_expected_time_per_block();

        if max_expected_time_per_block.is_zero() {
            return 0;
        }

        let delay = delay_period_time
            .as_secs()
            .checked_div(max_expected_time_per_block.as_secs())
            .unwrap();

        delay_period_time
            .checked_add(Duration::from_secs(delay))
            .unwrap()
            .as_secs()
    }
}
