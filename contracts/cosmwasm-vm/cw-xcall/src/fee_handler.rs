use super::*;

impl<'a> CwCallService<'a> {
    pub fn set_protocol_feehandler(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        address: String,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
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

    pub fn get_protocol_feehandler(&self, deps: Deps) -> String {
        self.query_feehandler(deps.storage).unwrap()
    }
}

impl<'a> CwCallService<'a> {
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

    fn query_feehandler(&self, store: &dyn Storage) -> Result<String, ContractError> {
        match self.fee_handler().load(store) {
            Ok(address) => Ok(address),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    fn get_balance(&self, querier: QuerierWrapper, address: String) -> StdResult<Coin> {
        querier.query_balance(address, "uconst")
    }
}
