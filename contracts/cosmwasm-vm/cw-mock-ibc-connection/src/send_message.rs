use common::rlp::Nullable;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Storage};
use cw_xcall_lib::network_address::NetId;

use crate::{
    error::ContractError,
    state::{CwIbcConnection, IbcConfig},
    types::{message::Message, LOG_PREFIX},
};

impl<'a> CwIbcConnection<'a> {
    pub fn send_message(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        _nid: NetId,
        _sn: i64,
        _message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        self.ensure_xcall_handler(deps.as_ref().storage, info.sender.clone())?;

        println!("{LOG_PREFIX} Packet Validated");
       Ok(Response::new())
    }
}
