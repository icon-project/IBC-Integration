use super::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallService<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        self.init(deps.storage, info, msg)
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetAdmin { address } => {
                let validated_address =
                    CwCallService::validate_address(deps.api, address.as_str())?;
                self.add_admin(deps.storage, info, validated_address)
            }
            ExecuteMsg::SetProtocol { value } => self.set_protocol_fee(deps, info, value),
            ExecuteMsg::SetProtocolFeeHandler { address } => {
                self.set_protocol_feehandler(deps, env, info, address)
            }
            ExecuteMsg::SendCallMessage { to, data, rollback } => {
                self.send_packet(deps, info, to, data, rollback)
            }
            ExecuteMsg::ExecuteCall { request_id } => self.execute_call(deps, info, request_id),
            ExecuteMsg::ExecuteRollback { sequence_no } => {
                self.execute_rollback(deps, info, sequence_no)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelOpen { msg } => Ok(self.on_channel_open(msg)?),
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelConnect { msg } => {
                Ok(self.on_channel_connect(deps.storage, msg)?)
            }
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcChannelClose { msg } => Ok(self.on_channel_close(msg)?),
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketReceive { msg } => Ok(self.on_packet_receive(deps, msg)?),
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketAck { msg } => Ok(self.on_packet_ack(msg)?),
            ExecuteMsg::UpdateAdmin { address } => {
                let validated_address =
                    CwCallService::validate_address(deps.api, address.as_str())?;
                self.update_admin(deps.storage, info, validated_address)
            }
            ExecuteMsg::RemoveAdmin {} => self.remove_admin(deps.storage, info),
            #[cfg(not(feature = "native_ibc"))]
            ExecuteMsg::IbcPacketTimeout { msg } => Ok(self.on_packet_timeout(msg)?),
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetAdmin {} => match self.query_admin(deps.storage) {
                Ok(admin) => Ok(to_binary(&admin)?),
                Err(error) => Err(StdError::NotFound {
                    kind: error.to_string(),
                }),
            },

            QueryMsg::GetProtocolFee {} => to_binary(&self.get_protocol_fee(deps)),
            QueryMsg::GetProtocolFeeHandler {} => to_binary(&self.get_protocol_feehandler(deps)),
        }
    }

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            EXECUTE_CALL_ID => self.reply_execute_call_message(deps.as_ref(), env, msg),
            EXECUTE_ROLLBACK_ID => self.reply_execute_rollback(deps.as_ref(), msg),
            SEND_CALL_MESSAGE_REPLY_ID => self.reply_sendcall_message(msg),
            ACK_FAILURE_ID => self.reply_ack_on_error(msg),
            _ => Err(ContractError::ReplyError {
                code: msg.id,
                msg: "Unkown".to_string(),
            }),
        }
    }
}

impl<'a> CwCallService<'a> {
    fn init(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let last_sequence_no = u128::default();
        let last_request_id = u128::default();
        let owner = Address::from(info.sender.as_str());

        self.add_owner(store, owner.clone())?;
        self.add_admin(store, info, owner)?;
        self.init_last_sequence_no(store, last_sequence_no)?;
        self.init_last_request_id(store, last_request_id)?;
        self.set_timeout_height(store, msg.timeout_height)?;
        self.set_ibc_host(store, msg.ibc_host.clone())?;

        Ok(Response::new()
            .add_attribute("action", "instantiate")
            .add_attribute("method", "init")
            .add_attribute("ibc_host", msg.ibc_host))
    }

    fn reply_execute_rollback(&self, deps: Deps, msg: Reply) -> Result<Response, ContractError> {
        let sequence_no = self.last_sequence_no().load(deps.storage)?;

        let response = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => CallServiceMessageResponse::new(
                sequence_no,
                CallServiceResponseType::CallServiceResponseSuccess,
                "",
            ),
            cosmwasm_std::SubMsgResult::Err(err) => {
                let error_message = format!("CallService Reverted : {err}");
                CallServiceMessageResponse::new(
                    sequence_no,
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                )
            }
        };

        let event = event_rollback_executed(
            sequence_no,
            to_int(response.response_code()),
            &to_string(response.message()).unwrap(),
        );

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_rollback")
            .add_event(event))
    }

    fn reply_execute_call_message(
        &self,
        deps: Deps,
        env: Env,
        msg: Reply,
    ) -> Result<Response, ContractError> {
        let req_id = self.last_request_id().load(deps.storage)?;
        let request = self.message_request().load(deps.storage, req_id)?;

        let responses = match msg.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                let code = 0;

                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseSuccess,
                    "",
                );
                let event = event_call_executed(req_id, code, "");
                (message_response, event)
            }
            cosmwasm_std::SubMsgResult::Err(err) => {
                let code = -1;
                let error_message = format!("CallService Reverted : {err}");
                let message_response = CallServiceMessageResponse::new(
                    request.sequence_no(),
                    CallServiceResponseType::CallServiceResponseFailure,
                    &error_message,
                );
                let event = event_call_executed(req_id, code, &error_message);
                (message_response, event)
            }
        };

        if !request.rollback().is_empty() {
            let message: CallServiceMessage = responses.0.into();

            let packet = self.create_packet_response(deps, env, to_binary(&message).unwrap());

            return Ok(Response::new()
                .add_attribute("action", "call_message")
                .add_attribute("method", "execute_callback")
                .add_message(packet));
        }

        Ok(Response::new()
            .add_attribute("action", "call_message")
            .add_attribute("method", "execute_callback")
            .add_event(responses.1))
    }

    fn create_packet_response(&self, deps: Deps, env: Env, data: Binary) -> IbcMsg {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();

        let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(300));

        IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data,
            timeout,
        }
    }
    fn reply_ack_on_error(&self, reply: Reply) -> Result<Response, ContractError> {
        match reply.result {
            SubMsgResult::Ok(_) => Ok(Response::new()),
            SubMsgResult::Err(err) => Ok(Response::new().set_data(make_ack_fail(err))),
        }
    }

    /// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
    fn on_channel_open(&self, msg: IbcChannelOpenMsg) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg.clone() {
            IbcChannelOpenMsg::OpenInit { channel } => channel.endpoint,
            IbcChannelOpenMsg::OpenTry {
                channel,
                counterparty_version: _,
            } => channel.endpoint,
        };
        let channel = msg.channel();

        check_order(&channel.order)?;

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }

        Ok(Response::new()
            .set_data(to_binary(&ibc_endpoint).unwrap())
            .add_attribute("method", "on_channel_open")
            .add_attribute("version", IBC_VERSION))
    }
    fn on_channel_connect(
        &self,
        store: &mut dyn Storage,
        msg: IbcChannelConnectMsg,
    ) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg.clone() {
            IbcChannelConnectMsg::OpenAck {
                channel,
                counterparty_version: _,
            } => channel.endpoint,
            IbcChannelConnectMsg::OpenConfirm { channel } => channel.endpoint,
        };
        let channel = msg.channel();

        check_order(&channel.order)?;

        if let Some(counter_version) = msg.counterparty_version() {
            check_version(counter_version)?;
        }

        let source = msg.channel().endpoint.clone();
        let destination = msg.channel().counterparty_endpoint.clone();

        let ibc_config = IbcConfig::new(source, destination);
        let mut call_service = CwCallService::default();
        call_service.save_config(store, &ibc_config)?;

        Ok(Response::new()
            .set_data(to_binary(&ibc_endpoint).unwrap())
            .add_attribute("method", "on_channel_connect")
            .add_attribute(
                "source_channel_id",
                msg.channel().endpoint.channel_id.as_str(),
            )
            .add_attribute("source_port_id", msg.channel().endpoint.port_id.as_str())
            .add_attribute(
                "destination_channel_id",
                msg.channel().counterparty_endpoint.channel_id.as_str(),
            )
            .add_attribute(
                "destination_port_id",
                msg.channel().counterparty_endpoint.port_id.as_str(),
            ))
    }
    fn on_channel_close(&self, msg: IbcChannelCloseMsg) -> Result<Response, ContractError> {
        let ibc_endpoint = match msg.clone() {
            IbcChannelCloseMsg::CloseInit { channel } => channel.endpoint,
            IbcChannelCloseMsg::CloseConfirm { channel } => channel.endpoint,
        };
        let channel = msg.channel().endpoint.channel_id.clone();

        Ok(Response::new()
            .add_attribute("method", "ibc_channel_close")
            .add_attribute("channel", channel)
            .set_data(to_binary(&ibc_endpoint).unwrap()))
    }
    fn on_packet_receive(
        &self,
        deps: DepsMut,
        msg: IbcPacketReceiveMsg,
    ) -> Result<Response, ContractError> {
        match self.receive_packet_data(deps, msg.packet) {
            Ok(ibc_response) => Ok(Response::new()
                .add_attributes(ibc_response.attributes.clone())
                .set_data(ibc_response.acknowledgement)
                .add_events(ibc_response.events)),
            Err(error) => Ok(Response::new()
                .add_attribute("method", "ibc_packet_receive")
                .add_attribute("error", error.to_string())
                .set_data(make_ack_fail(error.to_string()))),
        }
    }

    fn on_packet_ack(&self, ack: IbcPacketAckMsg) -> Result<Response, ContractError> {
        let ack_response: Ack = from_binary(&ack.acknowledgement.data)?;
        let message: CallServiceMessage = from_binary(&ack.original_packet.data)?;
        let message_type = match message.message_type() {
            CallServiceMessageType::CallServiceRequest => "call_service_request",
            CallServiceMessageType::CallServiceResponse => "call_service_response",
        };

        match ack_response {
            Ack::Result(_) => {
                let attributes = vec![
                    attr("action", "acknowledge"),
                    attr("success", "true"),
                    attr("message_type", message_type),
                ];

                Ok(Response::new().add_attributes(attributes))
            }
            Ack::Error(err) => Ok(Response::new()
                .add_attribute("action", "acknowledge")
                .add_attribute("message_type", message_type)
                .add_attribute("success", "false")
                .add_attribute("error", err)),
        }
    }

    fn on_packet_timeout(&self, _msg: IbcPacketTimeoutMsg) -> Result<Response, ContractError> {
        let submsg = SubMsg::reply_on_error(CosmosMsg::Custom(Empty {}), ACK_FAILURE_ID);
        Ok(Response::new()
            .add_submessage(submsg)
            .add_attribute("method", "ibc_packet_timeout"))
    }
    fn reply_sendcall_message(&self, message: Reply) -> Result<Response, ContractError> {
        match message.result {
            SubMsgResult::Ok(_) => Ok(Response::new()
                .add_attribute("action", "reply")
                .add_attribute("method", "sendcall_message")),
            SubMsgResult::Err(error) => Err(ContractError::ReplyError {
                code: message.id,
                msg: error,
            }),
        }
    }
}
