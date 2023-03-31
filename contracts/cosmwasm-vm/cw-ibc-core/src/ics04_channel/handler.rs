use std::str::FromStr;

use cosmwasm_std::{
    from_binary, to_binary, Binary, CosmosMsg, Empty, MessageInfo, Response, SubMsg, WasmMsg,
};
pub mod open_init;
pub mod open_try;
use super::*;
use cosmwasm_std::Reply;
use open_init::*;
use open_try::*;

impl<'a> ValidateChannel for CwIbcCoreContext<'a> {
    fn validate_channel_open_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenInit,
    ) -> Result<cosmwasm_std::Response, ContractError> {
        // connection hops should be 1
        if message.connection_hops_on_a.len() != 1 {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidConnectionHopsLength {
                    expected: 1,
                    actual: message.connection_hops_on_a.len(),
                },
            });
        }
        let connection_id = ConnectionId::from(message.connection_hops_on_a[0].clone());
        // An IBC connection running on the local (host) chain should exist.
        let conn_end_on_a = self.connection_end(deps.storage, connection_id.clone())?;
        channel_open_init_msg_validate(&message, conn_end_on_a)?;
        let counter = match self.channel_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        let channel_id_on_a = ChannelId::new(counter); // creating new channel_id
        let module_id = match self
            .lookup_module_by_port(deps.storage, PortId::from(message.port_id_on_a.clone()))
        {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };
        let module_id = types::ModuleId::from(module_id);
        let contract_address = match self.get_route(deps.storage, module_id) {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };
        // Store the channel details
        let counter_party = Counterparty::new(message.port_id_on_b.clone(), None);
        let channel_end = ChannelEnd::new(
            State::Uninitialized,
            message.ordering,
            counter_party,
            message.connection_hops_on_a.clone(),
            message.version_proposal.clone(),
        );
        self.store_channel_end(
            deps.storage,
            PortId::from(message.port_id_on_a.clone()),
            channel_id_on_a.clone(),
            channel_end,
        )?;

        // Generate event for calling on channel open init in x-call
        let sub_message = on_chan_open_init_submessage(&message, &channel_id_on_a, &connection_id);
        let data = cw_xcall::msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_open_init = create_channel_submesssage(
            contract_address.to_string(),
            data,
            &info,
            EXECUTE_ON_CHANNEL_OPEN_INIT,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_opne_init_validation")
            .add_submessage(on_chan_open_init))
    }

    fn validate_channel_open_try(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenTry,
    ) -> Result<Response, ContractError> {
        if message.connection_hops_on_b.len() != 1 {
            return Err(ContractError::IbcChannelError {
                error: ChannelError::InvalidConnectionHopsLength {
                    expected: 1,
                    actual: message.connection_hops_on_b.len(),
                },
            });
        }
        let connection_id = ConnectionId::from(message.connection_hops_on_b[0].clone());
        let conn_end_on_b = self.connection_end(deps.storage, connection_id.clone())?;

        // TODO
        // Verification //
        channel_open_try_msg_validate(&message, &conn_end_on_b)?;

        // TODO
        // Verification

        let counter = match self.channel_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        let channel_id_on_b = ChannelId::new(counter); // creating new channel_id
                                                       // Getting the module address for on channel open try call
        let module_id = match self
            .lookup_module_by_port(deps.storage, PortId::from(message.port_id_on_b.clone()))
        {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };
        let module_id = types::ModuleId::from(module_id);
        let contract_address = match self.get_route(deps.storage, module_id) {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };

        let counter_party = Counterparty::new(
            message.port_id_on_a.clone(),
            Some(message.chan_id_on_a.clone()),
        );
        let channel_end = ChannelEnd::new(
            State::Uninitialized,
            message.ordering,
            counter_party,
            message.connection_hops_on_b.clone(),
            message.version_supported_on_a.clone(),
        );
        self.store_channel_end(
            deps.storage,
            PortId::from(message.port_id_on_b.clone()),
            channel_id_on_b.clone(),
            channel_end,
        )?;

        // Generate event for calling on channel open init in x-call
        let sub_message = on_chan_open_try_submessage(&message, &channel_id_on_b, &connection_id);
        let data = cw_xcall::msg::ExecuteMsg::IbcChannelOpen { msg: sub_message };
        let data = to_binary(&data).unwrap();
        let on_chan_open_try = create_channel_submesssage(
            contract_address.to_string(),
            data,
            &info,
            EXECUTE_ON_CHANNEL_OPEN_TRY,
        );

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_opne_init_validation")
            .add_submessage(on_chan_open_try))
    }

    fn validate_channel_open_ack(
        &self,
        deps: DepsMut,
        message: &MsgChannelOpenAck,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    fn validate_channel_open_confirm(
        &self,
        deps: DepsMut,
        message: &MsgChannelOpenConfirm,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    fn validate_channel_close_init(
        &self,
        deps: DepsMut,
        message: &MsgChannelCloseInit,
    ) -> Result<Response, ContractError> {
        todo!()
    }

    fn validate_channel_close_confirm(
        &self,
        deps: DepsMut,
        message: &MsgChannelCloseConfirm,
    ) -> Result<Response, ContractError> {
        todo!()
    }
}

impl<'a> ExecuteChannel for CwIbcCoreContext<'a> {
    fn execute_channel_open_init(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = PortId::from(IbcPortId::from_str(&data.port_id).unwrap());
                    let channel_id =
                        ChannelId::from(IbcChannelId::from_str(&data.channel_id).unwrap());
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    if channel_end.state != State::Uninitialized {
                        return Err(ContractError::IbcChannelError {
                            error: ChannelError::UnknownState { state: 5 },
                        });
                    }
                    channel_end.state = State::Init;
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end,
                    )?;
                    let _sequence = self.increase_channel_sequence(deps.storage)?;
                    self.store_next_sequence_send(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_recv(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_ack(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    let channel_id_event = create_channel_id_generated_event(channel_id.clone());
                    return Ok(Response::new().add_event(channel_id_event));
                }
                None => {
                    return Err(ContractError::IbcChannelError {
                        error: ChannelError::Other {
                            description: "Data from module is Missing".to_string(),
                        },
                    })
                }
            },
            cosmwasm_std::SubMsgResult::Err(_) => {
                return Err(ContractError::IbcChannelError {
                    error: ChannelError::NoCommonVersion,
                })
            }
        }
    }

    fn execute_channel_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary::<cosmwasm_std::IbcEndpoint>(&res).unwrap();
                    let port_id = PortId::from(IbcPortId::from_str(&data.port_id).unwrap());
                    let channel_id =
                        ChannelId::from(IbcChannelId::from_str(&data.channel_id).unwrap());
                    let mut channel_end =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    if channel_end.state != State::Uninitialized {
                        return Err(ContractError::IbcChannelError {
                            error: ChannelError::UnknownState { state: 5 },
                        });
                    }
                    channel_end.state = State::TryOpen;
                    self.store_channel_end(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        channel_end.clone(),
                    )?;
                    let _sequence = self.increase_channel_sequence(deps.storage)?;
                    self.store_next_sequence_send(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_recv(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    self.store_next_sequence_ack(
                        deps.storage,
                        port_id.clone(),
                        channel_id.clone(),
                        1.into(),
                    )?;
                    let channel_id_event = create_channel_id_generated_event(channel_id.clone());
                    let main_event = create_open_try_channel_event(
                        &channel_id,
                        &port_id.clone().ibc_port_id(),
                        channel_end.counterparty().port_id(),
                        &channel_end.counterparty().channel_id.clone().unwrap(),
                        &channel_end.connection_hops()[0],
                        channel_end.version(),
                    );
                    return Ok(Response::new()
                        .add_event(channel_id_event)
                        .add_event(main_event));
                }
                None => {
                    return Err(ContractError::IbcChannelError {
                        error: ChannelError::Other {
                            description: "Data from module is Missing".to_string(),
                        },
                    })
                }
            },
            cosmwasm_std::SubMsgResult::Err(_) => {
                return Err(ContractError::IbcChannelError {
                    error: ChannelError::NoCommonVersion,
                })
            }
        }
    }
}
