use super::*;

impl<'a> CwCallService<'a> {
    pub fn send_packet(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        to: String,
        data: Vec<u8>,
        rollback: Option<Vec<u8>>,
    ) -> Result<Response, ContractError> {
        let from_address = info.sender.to_string();
        self.ensure_caller_is_contract_and_rollback_is_null(
            deps.as_ref(),
            info.sender.clone(),
            rollback.clone(),
        )?;
        let need_response = rollback.is_some();

        let rollback_data = match rollback {
            Some(data) => data,
            None => vec![],
        };

        self.ensure_data_length(data.len())?;
        self.ensure_rollback_length(&rollback_data)?;

        // TODO : ADD fee logic

        let sequence_no = self.increment_last_sequence_no(deps.storage)?;
        let ibc_host = self.get_host(deps.as_ref().storage)?;

        let ibc_config = self
            .ibc_config()
            .load(deps.as_ref().storage)
            .map_err(ContractError::Std)?;

        let query_message = cw_common::core_msg::QueryMsg::SequenceSend {
            port_id: ibc_config.src_endpoint().clone().port_id,
            channel_id: ibc_config.src_endpoint().clone().channel_id,
        };

        let query_request = QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart {
            contract_addr: ibc_host.to_string(),
            msg: to_binary(&query_message).map_err(ContractError::Std)?,
        });

        let sequence_number_host: u64 = deps
            .querier
            .query(&query_request)
            .map_err(ContractError::Std)?;

        if need_response {
            let request = CallRequest::new(
                Address::from(&from_address),
                to.clone(),
                rollback_data.clone(),
                need_response,
            );

            self.set_call_request(deps.storage, sequence_no, request)?;
        }

        let call_request = CallServiceMessageRequest::new(
            Address::from(info.sender.as_str()),
            to,
            sequence_no,
            rollback_data.to_vec(),
            data.to_vec(),
        );

        let message: CallServiceMessage = call_request.into();
        let timeout_height = self.get_timeout_height(deps.as_ref().storage)?;

        let event = event_xcall_message_sent(
            sequence_number_host,
            info.sender.to_string(),
            sequence_no,
            &message,
        );

        #[cfg(feature = "native_ibc")]
        {
            let packet = self.create_request_packet(deps, env, timeout_height, message.clone())?;

            let submessage: SubMsg<Empty> =
                SubMsg::reply_always(CosmosMsg::Ibc(packet), SEND_CALL_MESSAGE_REPLY_ID);

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("action", "xcall-service")
                .add_attribute("method", "send_packet")
                .add_event(event))
        }

        #[cfg(not(feature = "native_ibc"))]
        {
            let height =
                Height::new(0, timeout_height).map_err(|error| ContractError::DecodeFailed {
                    error: error.to_string(),
                })?;

            let packet_data = cw_common::RawPacket {
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

            let message = cw_common::core_msg::ExecuteMsg::SendPacket {
                packet: packet_data.encode_to_vec(),
            };

            let submessage = SubMsg {
                id: SEND_CALL_MESSAGE_REPLY_ID,
                msg: CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: ibc_host.to_string(),
                    msg: to_binary(&message).map_err(ContractError::Std)?,
                    funds: info.funds,
                }),
                gas_limit: None,
                reply_on: cosmwasm_std::ReplyOn::Always,
            };

            Ok(Response::new()
                .add_submessage(submessage)
                .add_attribute("action", "xcall-service")
                .add_attribute("method", "send_packet")
                .add_attribute("sequence_no", sequence_no.to_string())
                .add_event(event))
        }
    }
}

#[cfg(feature = "native_ibc")]
impl<'a> CwCallService<'a> {
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
