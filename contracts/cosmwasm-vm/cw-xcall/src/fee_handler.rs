use super::*;

impl<'a> CwCallService<'a> {
    /// This function sets a fee handler address and sends any accrued fees to the new fee handler
    /// address.
    ///
    /// Arguments:
    ///
    /// * `deps`: A mutable reference to the dependencies of the contract, which includes the storage
    /// and the querier.
    /// * `env`: The `env` parameter is of type `Env` and contains information about the current
    /// blockchain environment, such as the block height and time. It is used in this function to get
    /// the contract address and pass it to the `get_balance` function.
    /// * `info`: `info` is a `MessageInfo` struct that contains information about the message being
    /// executed, such as the sender's address, the amount of tokens sent with the message, and the gas
    /// limit.
    /// * `address`: The address of the fee handler that will receive protocol fees.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    pub fn set_protocol_feehandler(
        &self,
        deps: DepsMut,
        env: &Env,
        _info: &MessageInfo,
        address: String,
    ) -> Result<Response, ContractError> {
        self.add_feehandler(deps.storage, &address)?;

        if address.len().ne(&0) {
            let accrued_fees = self.get_balance(deps.querier, env.contract.address.to_string())?;

            if accrued_fees.amount.u128() > 0 {
                let message: CosmosMsg<Empty> = CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                    to_address: address,
                    amount: vec![accrued_fees],
                });

                return Ok(Response::new()
                    .add_message(message)
                    .add_attribute("action", "accrued_fees")
                    .add_attribute("method", "setprotocol_feehandler"));
            }
        };

        Ok(Response::new()
            .add_attribute("action", "accrued_fees")
            .add_attribute("method", "setprotocol_feehandler"))
    }

    /// This function retrieves the protocol fee handler address from storage.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is an object of type `Deps` which is a struct that contains various dependencies
    /// required by the contract to interact with the blockchain. It includes the storage, API, and
    /// querier objects. In this function, `deps` is used to access the storage object to query the fee
    ///
    /// Returns:
    ///
    /// A string representing the protocol fee handler.
    pub fn get_protocol_feehandler(&self, deps: Deps) -> String {
        self.query_feehandler(deps.storage).unwrap()
    }
}

impl<'a> CwCallService<'a> {
    /// This function adds a fee handler address to the contract's storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data between contract executions. The `dyn`
    /// keyword indicates that `Storage` is a dynamic trait object, meaning that it can be used to
    /// interact with any
    /// * `address`: The `address` parameter is a reference to a `String` that represents the Ethereum
    /// address of the fee handler contract that needs to be added to the current contract.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` object with either an `Ok(())` value indicating that the fee
    /// handler was successfully added, or an `Err` value containing a `ContractError::Std` object if
    /// there was an error while saving the fee handler to the storage.
    fn add_feehandler(
        &self,
        store: &mut dyn Storage,
        address: &String,
    ) -> Result<(), ContractError> {
        match self.fee_handler().save(store, address) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This function queries the fee handler address from the storage and returns it as a string or an
    /// error.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage, which is a key-value store that persists data on the
    /// blockchain. The `query_feehandler` function takes a reference to this object as an argument so
    /// that it can
    ///
    /// Returns:
    ///
    /// A `Result` containing either the `String` address of the fee handler or a `ContractError` if
    /// there was an error loading the address from the storage.
    fn query_feehandler(&self, store: &dyn Storage) -> Result<String, ContractError> {
        match self.fee_handler().load(store) {
            Ok(address) => Ok(address),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    /// This Rust function queries the balance of a given address using a QuerierWrapper.
    ///
    /// Arguments:
    ///
    /// * `querier`: The `querier` parameter is an instance of the `QuerierWrapper` struct, which is
    /// used to query the blockchain for information such as account balances, contract state, and
    /// transaction history. It provides a set of methods for querying the blockchain, such as
    /// `query_balance` which is used
    /// * `address`: The `address` parameter is a `String` representing the address of the account for
    /// which we want to get the balance.
    ///
    /// Returns:
    ///
    /// The `get_balance` function is returning a `Coin` object, which represents the balance of a given
    /// address in a specific denomination (in this case, "uconst").
    fn get_balance(&self, querier: QuerierWrapper, address: String) -> StdResult<Coin> {
        querier.query_balance(address, "uconst")
    }
}
