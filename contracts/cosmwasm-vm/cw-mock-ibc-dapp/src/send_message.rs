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
        let ibc_config = self.get_ibc_config(deps.as_ref().storage)?;

        let next_sequence_send = self.query_host_sequence_no(deps.as_ref(), &ibc_config)?;
        let msg = Message {
            sn: Nullable::new(Some(next_sequence_send)),
            data: message,
        };

        #[cfg(feature = "native_ibc")]
        {
            let packet = self.create_request_packet(deps, env, timeout_height, msg.clone())?;

            let submessage: SubMsg<Empty> =
                SubMsg::reply_always(CosmosMsg::Ibc(packet), HOST_FORWARD_REPLY_ID);

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("method", "send_message"))
        }

        #[cfg(not(feature = "native_ibc"))]
        {
            let ibc_height = Height::new(0, timeout_height).unwrap();
            let packet_data = self.create_packet(ibc_config, ibc_height, next_sequence_send, msg);

            println!("{} Raw Packet Created {:?}", LOG_PREFIX, &packet_data);

            let submessage: cosmwasm_std::SubMsg =
                self.call_host_send_message(deps, info, packet_data)?;
            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("method", "send_message"))
        }
    }

    fn write_acknowledgement(
        &self,
        _store: &mut dyn Storage,
        config: &IbcConfig,
        _msg: Vec<u8>,
        _sn: i64,
    ) -> Result<Response, ContractError> {
        let _channel_id = config.src_endpoint().channel_id.clone();
        Ok(Response::new())
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
