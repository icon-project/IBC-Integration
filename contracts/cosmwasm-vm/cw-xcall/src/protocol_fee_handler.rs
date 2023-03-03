use cosmwasm_std::{Coin, QuerierWrapper};
use cosmwasm_std::{
    CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Storage,
};

use crate::{error::ContractError, state::CwCallService, types::address::Address};

impl<'a> CwCallService<'a> {
    pub fn set_protocol_feehandler(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        address: Address,
    ) -> Result<Response, ContractError> {
        self.ensure_admin(deps.storage, info.sender)?;
        self.add_feehandler(deps.storage, &address)?;

        if address.len().ne(&0) {
            let accured_fees = self.get_balance(deps.querier, env.contract.address.to_string())?;

            if accured_fees.amount.u128() > 0 {
                let message: CosmosMsg<Empty> = CosmosMsg::Bank(cosmwasm_std::BankMsg::Send {
                    to_address: address.to_string(),
                    amount: vec![accured_fees],
                });

                return Ok(Response::new()
                    .add_message(message)
                    .add_attribute("action", "accured_fees")
                    .add_attribute("method", "setprotocol_feehandler"));
            }
        };
        
        Ok(Response::new()
            .add_attribute("action", "accured_fees")
            .add_attribute("method", "setprotocol_feehandler"))
    }

    pub fn get_protocol_feehandler(&self, deps: Deps) -> Address {
        self.query_feehandler(deps.storage).unwrap()
    }

    pub fn add_feehandler(
        &self,
        store: &mut dyn Storage,
        address: &Address,
    ) -> Result<(), ContractError> {
        match self.fee_handler().save(store, address) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn query_feehandler(&self, store: &dyn Storage) -> Result<Address, ContractError> {
        match self.fee_handler().load(store) {
            Ok(address) => Ok(address),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    fn get_balance(&self, querier: QuerierWrapper, address: String) -> StdResult<Coin> {
        querier.query_balance(address, "uconst")
    }
}
