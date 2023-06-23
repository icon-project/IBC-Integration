use std::str::FromStr;

use cosmwasm_std::{
    to_binary, to_vec, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, Response, SubMsg,
    WasmMsg,
};
use cw_common::{
    hex_string::HexString, raw_types::channel::RawPacket,
    xcall_types::network_address::NetworkAddress,
};
use debug_print::debug_println;

use crate::{
    error::ContractError,
    events::event_message_forwarded,
    state::{CwIbcConnection, HOST_FORWARD_REPLY_ID},
    types::LOG_PREFIX,
};
use cw_common::ibc_types::IbcHeight as Height;
use cw_common::ProstMessage;

impl<'a> CwIbcConnection<'a> {
    pub fn forward_to_host(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        to: String,
        message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        self.ensure_xcall_handler(deps.as_ref().storage, info.sender.clone())?;

        self.ensure_data_length(message.len())?;
        println!("{LOG_PREFIX} Packet Validated");

        // TODO : ADD fee logic

        //  let sequence_no = self.increment_last_sequence_no(deps.storage)?;
        let ibc_host = self.get_ibc_host(deps.as_ref().storage)?;

        println!("{} Forwarding to {}", LOG_PREFIX, &ibc_host);
        let na = NetworkAddress::from_str(&to).unwrap();

        let ibc_config = self.get_ibc_config(deps.as_ref().storage, na.get_nid())?;

        println!("{LOG_PREFIX} Loaded IbcConfig");
        let sequence_number_host = self.query_host_sequence_no(deps.as_ref(), &ibc_config)?;

        println!("{LOG_PREFIX} Received host sequence no {sequence_number_host}");

        let timeout_height =
            self.query_timeout_height(deps.as_ref(), &ibc_config.src_endpoint().channel_id)?;

        let event =
            event_message_forwarded(sequence_number_host, info.sender.to_string(), &message);
        println!("{} Message Forward Event {:?}", LOG_PREFIX, &event);

        #[cfg(feature = "native_ibc")]
        {
            let packet = self.create_request_packet(deps, env, timeout_height, message.clone())?;

            let submessage: SubMsg<Empty> =
                SubMsg::reply_always(CosmosMsg::Ibc(packet), HOST_FORWARD_REPLY_ID);

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("method", "forward_packet")
                .add_event(event))
        }

        #[cfg(not(feature = "native_ibc"))]
        {
            let packet_data =
                self.create_packet(ibc_config, timeout_height, sequence_number_host, message);

            println!("{} Raw Packet Created {:?}", LOG_PREFIX, &packet_data);
            let submessage = self.create_send_packet_submessage(deps, info, packet_data)?;
            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("action", "xcall-service")
                .add_attribute("method", "forward_packet")
                .add_event(event))
        }
    }

    pub fn create_send_packet_submessage(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        packet: RawPacket,
    ) -> Result<SubMsg, ContractError> {
        let message = cw_common::core_msg::ExecuteMsg::SendPacket {
            packet: HexString::from_bytes(&packet.encode_to_vec()),
        };
        let ibc_host = self.get_ibc_host(deps.as_ref().storage)?;
        let submessage = SubMsg {
            id: HOST_FORWARD_REPLY_ID,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: ibc_host.to_string(),
                msg: to_binary(&message).map_err(ContractError::Std)?,
                funds: info.funds,
            }),
            gas_limit: None,
            reply_on: cosmwasm_std::ReplyOn::Always,
        };
        debug_println!("{LOG_PREFIX} Packet Forwarded To IBCHost {ibc_host} ");
        Ok(submessage)
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
        message: CallServiceMessage,
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
