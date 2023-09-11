use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response, Uint128};
use cw_xcall_lib::network_address::NetId;

use crate::{error::ContractError, state::CwIbcConnection, types::LOG_PREFIX};

impl<'a> CwIbcConnection<'a> {
    pub fn send_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        nid: NetId,
        sn: i64,
        _message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        self.ensure_xcall_handler(deps.as_ref().storage, info.sender.clone())?;
        println!("{LOG_PREFIX} Packet Validated");

        let network_fee = self.get_network_fees(deps.as_ref().storage, nid.clone());
        let mut total_fee = network_fee.send_packet_fee;

        if sn > 0 {
            total_fee = total_fee + network_fee.ack_fee;
        }
        let config = self.get_config(deps.storage)?;

        let fund = get_amount_for_denom(&info.funds, config.denom);

        if fund < total_fee.into() {
            return Err(ContractError::InsufficientFunds {});
        }

        Ok(Response::new())
    }
}


fn get_amount_for_denom(funds: &Vec<Coin>, target_denom: String) -> Uint128 {
    for coin in funds.iter() {
        if coin.denom == target_denom {
            return coin.amount;
        }
    }
    Uint128::zero()
}
