use super::*;

/// This function validates if a channel open try message is valid based on the connection end on B.
/// 
/// Arguments:
/// 
/// * `message`: A message of type `MsgChannelOpenTry` which contains information about the channel
/// being opened and its associated parameters.
/// * `conn_end_on_b`: `conn_end_on_b` is a reference to a `ConnectionEnd` object representing the
/// connection end on the counterparty chain.
/// 
/// Returns:
/// 
/// a `Result` type with either an `Ok(())` value indicating that the validation was successful, or an
/// `Err(ContractError)` value indicating that the validation failed with a specific `ContractError`
/// type.
pub fn channel_open_try_msg_validate(
    message: &MsgChannelOpenTry,
    conn_end_on_b: &ConnectionEnd,
) -> Result<(), ContractError> {
    if !conn_end_on_b.state_matches(&ConnectionState::Open) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ConnectionNotOpen {
                connection_id: message.connection_hops_on_b[0].clone(),
            },
        });
    };

    let conn_version = match conn_end_on_b.versions() {
        [version] => version,
        _ => {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidVersionLengthConnection,
            })
        }
    };

    let channel_feature = message.ordering.to_string();
    if !conn_version.is_supported_feature(channel_feature) {
        return Err(ContractError::IbcChannelError {
            error: ChannelError::ChannelFeatureNotSupportedByConnection,
        });
    }

    Ok(())
}

/// This function creates an IBC channel open try message based on the provided channel end, port ID,
/// channel ID, and connection ID.
/// 
/// Arguments:
/// 
/// * `msg`: A reference to a ChannelEnd struct, which contains information about the channel.
/// * `port_id`: The identifier of the port associated with the channel being opened.
/// * `channel_id`: The channel identifier, which is a unique identifier for the channel within the
/// given port and connection.
/// * `connection_id`: The ID of the connection associated with the channel being opened.
pub fn on_chan_open_try_submessage(
    msg: &ChannelEnd,
    port_id: &PortId,
    channel_id: &ChannelId,
    connection_id: &ConnectionId,
) -> cosmwasm_std::IbcChannelOpenMsg {
    let port_id = port_id.clone();
    let channel_id = channel_id.ibc_channel_id();
    let counter_party_port_id = msg.counterparty().port_id.clone();
    let counter_party_channel = msg.counterparty().channel_id().unwrap().clone();
    let endpoint = cosmwasm_std::IbcEndpoint {
        port_id: port_id.to_string(),
        channel_id: channel_id.to_string(),
    };
    let counter_party = cosmwasm_std::IbcEndpoint {
        port_id: counter_party_port_id.to_string(),
        channel_id: counter_party_channel.to_string(),
    };
    let ibc_channel = cosmwasm_std::IbcChannel::new(
        endpoint,
        counter_party,
        cosmwasm_std::IbcOrder::Unordered,
        msg.version.to_string(),
        connection_id.connection_id().to_string(),
    );
    cosmwasm_std::IbcChannelOpenMsg::OpenTry {
        channel: ibc_channel,
        counterparty_version: msg.version.to_string(),
    }
}

impl<'a> CwIbcCoreContext<'a> {
    /// This function executes an on-channel open try submessage for a given light client response.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage, API, and other modules. It is of type `DepsMut`.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract. It is used to extract the data returned by the sub-message and use it to perform
    /// further actions.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn execute_open_try_from_light_client(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let response =
                        from_binary::<cw_common::client_response::LightClientResponse>(&res)
                            .unwrap();
                    let info = response.message_info;
                    let data = response.ibc_endpoint;
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
                    let module_id = cw_common::types::ModuleId::from(module_id);
                    let contract_address = match self.get_route(deps.storage, module_id) {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };

                    // Generate event for calling on channel open try in x-call
                    let sub_message = on_chan_open_try_submessage(
                        &channel_end,
                        &port_id,
                        &channel_id,
                        &channel_end.connection_hops[0].clone().into(),
                    );
                    let data =
                        cw_common::xcall_msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };
                    let data = to_binary(&data).unwrap();
                    let on_chan_open_try = create_channel_submesssage(
                        contract_address.to_string(),
                        data,
                        info.funds,
                        EXECUTE_ON_CHANNEL_OPEN_TRY,
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
            cosmwasm_std::SubMsgResult::Err(_) => Err(ContractError::IbcChannelError {
                error: ChannelError::NoCommonVersion,
            }),
        }
    }
}
