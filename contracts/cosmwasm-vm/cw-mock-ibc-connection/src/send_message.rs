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
        nid: NetId,
        sn: i64,
        message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        // self.ensure_xcall_handler(deps.as_ref().storage, info.sender.clone())?;
        Ok(Response::new())
    }

    fn write_acknowledgement(
        &self,
        store: &mut dyn Storage,
        config: &IbcConfig,
        msg: Vec<u8>,
        sn: i64,
    ) -> Result<Response, ContractError> {
        let channel_id = config.src_endpoint().channel_id.clone();
        let packet = self.get_incoming_packet(store, &channel_id, sn)?;
        self.remove_incoming_packet(store, &channel_id, sn);
        let submsg = self.call_host_write_acknowledgement(store, packet, msg)?;
        Ok(Response::new().add_submessage(submsg))
    }
}

#[cfg(feature = "native_ibc")]
impl<'a> CwIbcConnection<'a> {
    /// This function creates an IBC message to send a packet with a timeout to a destination endpoint.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract. It is used to
    /// interact with the storage and other modules of the contract.
    /// * `env`: `env` is an object that contains information about the current blockchain environment,
    /// such as the current block height, time, and chain ID. It is used to calculate the timeout for the
    /// IBC packet.
    /// * `time_out_height`: The height of the block at which the timeout for the packet will occur.
    /// * `message`: `message` is a `CallServiceMessage` struct that contains the information needed to
    /// create a request packet to be sent over the IBC channel. This includes the method name, input
    /// arguments, and any other relevant data needed for the service call.
    ///
    /// Returns:
    ///
    /// a `Result` with an `IbcMsg` on success or a `ContractError` on failure.
    fn create_request_packet(
        &self,
        deps: DepsMut,
        env: Env,
        time_out_height: u64,
        message: Message,
    ) -> Result<IbcMsg, ContractError> {
        let ibc_config = self
            .ibc_config()
            .load(deps.as_ref().storage)
            .map_err(ContractError::Std)?;

        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));

        Ok(IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data: to_binary(&message).unwrap(),
            timeout,
        })
    }
}
