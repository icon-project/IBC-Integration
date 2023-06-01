use super::*;

/// The function validates the channel open acknowledgement message and returns an error if the channel
/// state or connection hops are invalid.
///
/// Arguments:
///
/// * `message`: A reference to a `MsgChannelOpenAck` struct, which contains information about the
/// acknowledgement of a channel opening message.
/// * `chan_end_on_a`: `chan_end_on_a` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the "A" side (i.e. the side that initiated the channel opening handshake). This
/// object contains information such as the channel ID, the counterparty channel ID, the connection hops
///
/// Returns:
///
/// a `Result` type with either an `Ok(())` value indicating that the validation was successful, or an
/// `Err(ContractError)` value indicating that the validation failed with a specific `ContractError`
/// type.
pub fn channel_open_ack_validate(
    message: &MsgChannelOpenAck,
    chan_end_on_a: &ChannelEnd,
) -> Result<(), ContractError> {
    if !chan_end_on_a.state_matches(&State::Init) {
        return Err(ChannelError::InvalidChannelState {
            channel_id: message.chan_id_on_a.clone(),
            state: chan_end_on_a.state,
        })
        .map_err(Into::<ContractError>::into);
    }

    if chan_end_on_a.connection_hops().len() != 1 {
        return Err(ChannelError::InvalidConnectionHopsLength {
            expected: 1,
            actual: chan_end_on_a.connection_hops().len(),
        })
        .map_err(Into::<ContractError>::into);
    }

    Ok(())
}

/// This function creates an IBC channel connect message for an open acknowledgement submessage.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and version.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The ID of the channel being opened and acknowledged.
/// * `connection_id`: The ID of the connection associated with the channel being opened.
///
/// Returns:
///
/// a `Result` with `cosmwasm_std::IbcChannelConnectMsg` as the success type and `ContractError` as the
/// error type.
pub fn on_chan_open_ack_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> Result<cosmwasm_std::IbcChannelConnectMsg, ContractError> {
    let port_id = port_id.clone();
    let channel_id = channel_id;
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
            return Err(ChannelError::UnknownOrderType {
                type_id: "None".to_string(),
            })
            .map_err(Into::<ContractError>::into)
        }
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        ibc_order,
        channel_end.version.to_string(),
        connection_id.to_string(),
    );
    let data = cosmwasm_std::IbcChannelConnectMsg::OpenAck {
        channel: ibc_channel,
        counterparty_version: channel_end.version.to_string(),
    };
    Ok(data)
}

impl<'a> CwIbcCoreContext<'a> {
    /// This function executes an "on channel open try" submessage in response to a successful "open
    /// acknowledgement" message from a light client.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. It is used to interact with the storage, API, and other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by a
    /// light client. It is used to extract the data returned by the sub-message and generate a new
    /// sub-message to be sent to another contract.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn execute_open_ack_from_light_client_reply(
        &self,
        deps: DepsMut,

        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let response = from_binary::<LightClientResponse>(&res).unwrap();
                    let info = response.message_info;
                    let data = response.ibc_endpoint;
                    let port_id = IbcPortId::from_str(&data.port_id).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.channel_id).unwrap();
                    let channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    // Getting the module address for on channel open try call
                    let module_id = match self.lookup_module_by_port(deps.storage, port_id.clone())
                    {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };
                    let module_id = module_id;
                    let contract_address = match self.get_route(deps.storage, module_id) {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };

                    // Generate event for calling on channel open try in x-call
                    let sub_message = on_chan_open_ack_submessage(
                        &channel_end,
                        &port_id,
                        &channel_id,
                        &channel_end.connection_hops[0].clone(),
                    )?;
                    let data =
                        cw_common::xcall_msg::ExecuteMsg::IbcChannelConnect { msg: sub_message };
                    let data = to_binary(&data).unwrap();
                    let on_chan_open_try = create_channel_submesssage(
                        contract_address.to_string(),
                        data,
                        info.funds,
                        EXECUTE_ON_CHANNEL_OPEN_ACK_ON_MODULE,
                    );

                    Ok(
                        Response::new()
                            .add_attribute("action", "channel")
                            .add_attribute("method", "channel_open_init_module_validation"),
                        // .add_submessage(on_chan_open_try)
                    )
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(error) => {
                Err(ChannelError::VerifyChannelFailed(ClientError::Other {
                    description: error,
                }))
                .map_err(Into::<ContractError>::into)
            }
        }
    }
}
