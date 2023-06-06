use super::*;

/// The function validates a channel close confirmation message.
///
/// Arguments:
///
/// * `message`: A message of type `MsgChannelCloseConfirm` which contains information about the channel
/// close confirmation being validated.
/// * `chan_end_on_b`: `chan_end_on_b` is a reference to a `ChannelEnd` object representing the state of
/// the channel on the counterparty chain.
///
/// Returns:
///
/// a `Result` type with the `Ok` variant containing an empty tuple `()` and the `Err` variant
/// containing a `ContractError` type.
pub fn channel_close_confirm_validate(
    message: &MsgChannelCloseConfirm,
    chan_end_on_b: &ChannelEnd,
) -> Result<(), ContractError> {
    if chan_end_on_b.state_matches(&State::Closed) {
        return Err(ChannelError::ChannelClosed {
            channel_id: message.chan_id_on_b.clone(),
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
    /// This function executes a close confirmation after the validation from the light client and call the
    /// xcall.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. It is used to interact with the storage, API, and other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to a light client. It is used to extract the data returned by the sub-message and
    /// perform further actions based on it.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn execute_close_confirm_from_light_client_reply(
        &self,
        deps: DepsMut,

        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let response = from_binary_response::<LightClientResponse>(&res).unwrap();
                    let info = response.message_info;
                    let data = response.ibc_endpoint;
                    let port_id =
                        IbcPortId::from_str(&data.port_id).map_err(Into::<ContractError>::into)?;
                    let channel_id = IbcChannelId::from_str(&data.channel_id)
                        .map_err(Into::<ContractError>::into)?;
                    let channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    // Getting the module address for on channel open try call
                    let contract_address =
                        match self.lookup_modules(deps.storage, port_id.as_bytes().to_vec()) {
                            Ok(addr) => addr,
                            Err(error) => return Err(error),
                        };

                    // Generate event for calling on channel open try in x-call
                    let sub_message =
                        on_chan_close_confirm_submessage(&channel_end, &port_id, &channel_id)?;
                    let data =
                        cw_common::xcall_msg::ExecuteMsg::IbcChannelClose { msg: sub_message };
                    let data = to_binary(&data).map_err(Into::<ContractError>::into)?;
                    let on_chan_close_confirm = create_channel_submesssage(
                        contract_address,
                        data,
                        info.funds,
                        EXECUTE_ON_CHANNEL_CLOSE_CONFIRM_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "channel_close_confirm_module_validation")
                        .add_submessage(on_chan_close_confirm))
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

/// This function creates an IBC channel close confirmation sub message for calling xcall.
///
/// Arguments:
///
/// * `channel_end`: A reference to a `ChannelEnd` struct, which contains information about the channel,
/// such as its state, ordering, and connection hops.
/// * `port_id`: The identifier of the port associated with the channel being closed.
/// * `channel_id`: The unique identifier of the channel within the given port.
///
/// Returns:
///
/// a `Result` with an `IbcChannelCloseMsg` as the Ok variant and a `ContractError` as the Err variant.
pub fn on_chan_close_confirm_submessage(
    channel_end: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Result<cosmwasm_std::IbcChannelCloseMsg, ContractError> {
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
    let data = cosmwasm_std::IbcChannelCloseMsg::CloseConfirm {
        channel: ibc_channel,
    };
    Ok(data)
}
