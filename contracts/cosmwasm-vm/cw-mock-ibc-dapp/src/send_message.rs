use common::{ibc::core::ics02_client::height::Height, rlp::Nullable};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Storage};

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
        message: Vec<u8>,
        timeout_height: u64,
    ) -> Result<Response, ContractError> {
        println!("{LOG_PREFIX} Packet Validated");
        // let ibc_config = self.get_ibc_config(deps.as_ref().storage)?;

        // let next_sequence_send = self.query_host_sequence_no(deps.as_ref(), &ibc_config)?;
        // let msg = Message {
        //     sn: Nullable::new(Some(next_sequence_send)),
        //     data: message,
        // };

        // let ibc_height = Height::new(0, timeout_height).unwrap();
        // let packet_data = self.create_packet(ibc_config, ibc_height, next_sequence_send, msg);

        // println!("{} Raw Packet Created {:?}", LOG_PREFIX, &packet_data);

        // let submessage: cosmwasm_std::SubMsg =
        //     self.call_host_send_message(deps, info, packet_data)?;
        Ok(Response::new()
            .add_attribute("method", "send_message"))
    }
}
