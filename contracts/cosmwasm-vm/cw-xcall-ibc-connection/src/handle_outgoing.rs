use cw_common::hex_string::HexString;

use crate::{state::XCALL_FORWARD_REPLY_ID, types::LOG_PREFIX};

use super::*;

impl<'a> CwIbcConnection<'a> {
    pub fn forward_to_host(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        message: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let from_address = info.sender.to_string();
        self.ensure_xcall_handler(deps.as_ref().storage, info.sender.clone())?;

        self.ensure_data_length(message.len())?;
        println!("{} Packet Validated", LOG_PREFIX);

        // TODO : ADD fee logic

        let sequence_no = self.increment_last_sequence_no(deps.storage)?;
        let ibc_host = self.get_ibc_host(deps.as_ref().storage)?;

        println!(
            "{} Forwarding to {} with sequence {}",
            LOG_PREFIX, &ibc_host, sequence_no
        );

        let ibc_config = self.ibc_config().load(deps.as_ref().storage).map_err(|e| {
            println!("{} Failed Loading IbcConfig {:?}", LOG_PREFIX, e);
            ContractError::Std(e)
        })?;

        println!("{} Loaded IbcConfig", LOG_PREFIX);
        let query_message = cw_common::core_msg::QueryMsg::SequenceSend {
            port_id: ibc_config.src_endpoint().clone().port_id,
            channel_id: ibc_config.src_endpoint().clone().channel_id,
        };

        let query_request = QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
            contract_addr: ibc_host.to_string(),
            msg: to_binary(&query_message).map_err(ContractError::Std)?,
        });
        println!("{} Created Query Request", LOG_PREFIX);

        let sequence_number_host: u64 = deps
            .querier
            .query(&query_request)
            .map_err(ContractError::Std)?;

        println!(
            "{} Received host sequence no {}",
            LOG_PREFIX, sequence_number_host
        );

        let timeout_height = self.get_timeout_height(deps.as_ref().storage);

        let event = event_message_forwarded(
            sequence_number_host,
            info.sender.to_string(),
            sequence_no,
            &message,
        );
        println!("{} Message Forward Event {:?}", LOG_PREFIX, &event);

        #[cfg(feature = "native_ibc")]
        {
            let packet = self.create_request_packet(deps, env, timeout_height, message.clone())?;

            let submessage: SubMsg<Empty> =
                SubMsg::reply_always(CosmosMsg::Ibc(packet), SEND_CALL_MESSAGE_REPLY_ID);

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("method", "forward_packet")
                .add_event(event))
        }

        #[cfg(not(feature = "native_ibc"))]
        {
            let height =
                Height::new(0, timeout_height).map_err(|error| ContractError::DecodeFailed {
                    error: error.to_string(),
                })?;

            let packet_data = cw_common::raw_types::channel::RawPacket {
                sequence: sequence_number_host,
                source_port: ibc_config.src_endpoint().clone().port_id,
                source_channel: ibc_config.src_endpoint().clone().channel_id,
                destination_port: ibc_config.dst_endpoint().clone().port_id,
                destination_channel: ibc_config.dst_endpoint().clone().channel_id,
                data: to_vec(&message).map_err(|error| ContractError::DecodeFailed {
                    error: error.to_string(),
                })?,
                timeout_height: Some(height.into()),
                timeout_timestamp: 0,
            };

            println!("{} Raw Packet Created {:?}", LOG_PREFIX, &packet_data);

            let message = cw_common::core_msg::ExecuteMsg::SendPacket {
                packet: HexString::from_bytes(&packet_data.encode_to_vec()),
            };

            let submessage = SubMsg {
                id: XCALL_FORWARD_REPLY_ID,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: ibc_host.to_string(),
                    msg: to_binary(&message).map_err(ContractError::Std)?,
                    funds: info.funds,
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Always,
            };
            println!("{} Packet Forwarded To IBCHost {} ", LOG_PREFIX, ibc_host);
            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("action", "xcall-service")
                .add_attribute("method", "forward_packet")
                .add_attribute("sequence_no", sequence_no.to_string())
                .add_event(event))
        }
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
