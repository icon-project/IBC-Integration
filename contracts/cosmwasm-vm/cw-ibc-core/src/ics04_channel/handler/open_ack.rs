use super::*;

pub fn channel_open_ack_validate(
    message: &MsgChannelOpenAck,
    chan_end_on_a: &ChannelEnd,
) -> Result<(), ContractError> {
    if !chan_end_on_a.state_matches(&State::Init) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidChannelState {
                channel_id: message.chan_id_on_a.clone(),
                state: chan_end_on_a.state,
            },
        });
    }

    if chan_end_on_a.connection_hops().len() != 1 {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::InvalidConnectionHopsLength {
                expected: 1,
                actual: chan_end_on_a.connection_hops().len(),
            },
        });
    }

    Ok(())
}

pub fn on_chan_open_ack_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> Result<cosmwasm_std::IbcChannelConnectMsg, ContractError> {
    let port_id = port_id.clone();
    let channel_id = channel_id.ibc_channel_id();
    let counter_party_port_id = channel_end.counterparty().port_id.clone();
    let counter_party_channel = channel_end.counterparty().channel_id().unwrap().clone();
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: counter_party_port_id.to_string(),
        channel_id: counter_party_channel.to_string(),
    };
    let ibc_order = match channel_end.ordering {
        Order::Unordered => cosmwasm_std::IbcOrder::Unordered,
        Order::Ordered => cosmwasm_std::IbcOrder::Ordered,
        Order::None => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::UnknownOrderType {
                    type_id: "None".to_string(),
                },
            })
        }
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        ibc_order,
        channel_end.version.to_string(),
        connection_id.connection_id().to_string(),
    );
    let data = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: ibc_channel,
        counterparty_version: channel_end.version.to_string(),
    };
    Ok(data)
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn execute_open_ack_from_light_client_reply(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = PortId::from(IbcPortId::from_str(&data.port_id).unwrap());
                    let channel_id =
                        ChannelId::from(IbcChannelId::from_str(&data.channel_id).unwrap());
                    let channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    // Getting the module address for on channel open try call
                    let module_id = match self.lookup_module_by_port(deps.storage, port_id.clone())
                    {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };
                    let module_id = types::ModuleId::from(module_id);
                    let contract_address = match self.get_route(deps.storage, module_id) {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };

                    // Generate event for calling on channel open try in x-call
                    let sub_message = on_chan_open_ack_submessage(
                        &channel_end,
                        &port_id,
                        &channel_id,
                        &channel_end.connection_hops[0].clone().into(),
                    )?;
                    let data = cw_xcall::msg::ExecuteMsg::IbcChannelConnect { msg: sub_message };
                    let data = to_binary(&data).unwrap();
                    let on_chan_open_try = create_channel_submesssage(
                        contract_address.to_string(),
                        data,
                        &info,
                        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "channel_open_init_module_validation")
                        .add_submessage(on_chan_open_try))
                }
                None => Err(ContractError::IbcChannelError {
                    error: ChannelError::Other {
                        description: "Data from module is Missing".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(error) => Err(ContractError::IbcChannelError {
                error: ChannelError::VerifyChannelFailed(ClientError::Other { description: error }),
            }),
        }
    }
}
