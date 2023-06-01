use super::*;

/// This function validates the channel open confirmation message and returns an error if the channel
/// state or connection hops are invalid.
///
/// Arguments:
///
/// * `message`: A reference to a `MsgChannelOpenConfirm` struct, which contains information about the
/// channel open confirmation message being validated.
/// * `chan_end_on_b`: `chan_end_on_b` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the counterparty chain.
///
/// Returns:
///
/// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
/// containing a `ContractError` type.
pub fn channel_open_confirm_validate(
    message: &MsgChannelOpenConfirm,
    chan_end_on_b: &ChannelEnd,
) -> Result<(), ContractError> {
    if !chan_end_on_b.state_matches(&State::TryOpen) {
        return Err(ChannelError::InvalidChannelState {
            channel_id: message.chan_id_on_b.clone(),
            state: chan_end_on_b.state,
        })
        .map_err(Into::<ContractError>::into);
    }
    if chan_end_on_b.connection_hops().len() != 1 {
        return Err(ChannelError::InvalidConnectionHopsLength {
            expected: 1,
            actual: chan_end_on_b.connection_hops().len(),
        })
        .map_err(Into::<ContractError>::into);
    }

    Ok(())
}

impl<'a> CwIbcCoreContext<'a> {
    /// This function executes a confirmation message from a light client and generates an event for
    /// calling on channel open try in x-call.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules. It is of type `DepsMut`.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message execution.
    /// It is used in the `execute_open_confirm_from_light_client_reply` function to extract the data
    /// returned by the sub-message and use it to generate a new sub-message to be executed.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can occur
    /// during contract execution.
    pub fn execute_open_confirm_from_light_client_reply(
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
                    let sub_message =
                        on_chan_open_confirm_submessage(&channel_end, &port_id, &channel_id)?;
                    let data =
                        cw_common::xcall_msg::ExecuteMsg::IbcChannelConnect { msg: sub_message };
                    let data = to_binary(&data).unwrap();
                    let on_chan_open_try = create_channel_submesssage(
                        contract_address.to_string(),
                        data,
                        info.funds,
                        EXECUTE_ON_CHANNEL_OPEN_CONFIRM_ON_MODULE,
                    );

                    Ok(
                        Response::new()
                            .add_attribute("action", "channel")
                            .add_attribute("method", "channel_open_confirm_module_validation"),
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

/// This function creates an IBC channel connect message for an open confirmation submessage for calling in xcall.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and connection hops.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The unique identifier of the channel within the given port.
///
/// Returns:
///
/// a `Result` with a `cosmwasm_std::IbcChannelConnectMsg` as the success type and a `ContractError` as
/// the error type. The success type is the result of creating an `IbcChannelConnectMsg` with the
/// `OpenConfirm` variant, which contains an `IbcChannel` struct with information about the channel
/// endpoint, counterparty,
pub fn on_chan_open_confirm_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Result<cosmwasm_std::IbcChannelConnectMsg, ContractError> {
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
        channel_end.connection_hops[0].clone().as_str(),
    );
    let data = cosmwasm_std::IbcChannelConnectMsg::OpenConfirm {
        channel: ibc_channel,
    };
    Ok(data)
}
