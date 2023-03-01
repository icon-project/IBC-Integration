use cosmwasm_std::Coin;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, StdResult,
};

use crate::{error::ContractError, state::CwCallservice, types::address::Address};

impl<'a> CwCallservice<'a> {
    pub fn setprotocol_feehandler(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        address: Address,
    ) -> Result<Response, ContractError> {
        self.ensure_admin_call_or_not(deps.storage, info.sender)?;
        self.fee_handler().save(deps.storage, &address)?;

        if address.to_string().len().ne(&0) {
            let accured_fees = self.get_balance(deps.as_ref(), &address)?;

            if !accured_fees.is_empty() {
                self.create_packet(
                    deps.as_ref(),
                    env,
                    to_binary(&address).unwrap(),
                    accured_fees[0].clone(),
                );
            }
        }

        Ok(Response::new()
            .add_attribute("action", "accured_fees")
            .add_attribute("method", "setprotocol_feehandler"))
    }

    pub fn create_packet(&self, deps: Deps, env: Env, data: Binary, accured_fees: Coin) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));
        let to_address = " ";
        let amount = accured_fees;

        IbcMsg::Transfer {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            to_address: " ".to_owned(),
            amount,
            timeout,
        }
    }

    pub fn get_balance(&self, deps: Deps, address: &Address) -> StdResult<Vec<Coin>> {
        let _ = &CwCallservice::new().fee_handler().load(deps.storage)?;
        deps.querier.query_all_balances(address.to_string())
    }

    pub fn get_protocolfeehandler(&self, deps: Deps) -> StdResult<Address> {
        return self.fee_handler().load(deps.storage);
    }
}
