use cosmwasm_std::IbcReceiveResponse;
use ibc::core::ics04_channel::{
    msgs::{acknowledgement::Acknowledgement, recv_packet::MsgRecvPacket},
    packet::Receipt,
};

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function validates a received packet in an IBC channel and creates a submessage to call a
    /// light client for further validation.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender and the amount of funds sent with the message.
    /// * `msg`: `msg` is a reference to a `MsgRecvPacket` struct, which contains information about a
    /// received packet in an IBC channel. It includes the packet data, the sender and receiver channel
    /// IDs, the timeout height and timestamp, and proof information for verifying the packet
    /// commitment.
    /// 
    /// Returns:
    /// 
    /// A `Result<Response, ContractError>` is being returned.
    pub fn validate_receive_packet(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: &MsgRecvPacket,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet.clone();
        let chan_end_on_b = self.get_channel_end(
            deps.storage,
            msg.packet.port_id_on_b.clone().into(),
            msg.packet.chan_id_on_b.clone().into(),
        )?;
        if !chan_end_on_b.state_matches(&State::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidChannelState {
                    channel_id: msg.packet.chan_id_on_a.clone(),
                    state: chan_end_on_b.state,
                },
            });
        }
        let counterparty = Counterparty::new(
            msg.packet.port_id_on_a.clone(),
            Some(msg.packet.chan_id_on_a.clone()),
        );
        if !chan_end_on_b.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: msg.packet.port_id_on_a.clone(),
                    channel_id: msg.packet.chan_id_on_a.clone(),
                },
            });
        }
        let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
        let conn_end_on_b = self.connection_end(deps.storage, conn_id_on_b.clone().into())?;
        if !conn_end_on_b.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ConnectionNotOpen {
                    connection_id: chan_end_on_b.connection_hops()[0].clone(),
                },
            });
        }
        let latest_height = self.host_height()?;
        if msg.packet.timeout_height_on_b.has_expired(latest_height) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::LowPacketHeight {
                    chain_height: latest_height,
                    timeout_height: msg.packet.timeout_height_on_b,
                },
            });
        }
        let latest_timestamp = self.host_timestamp(deps.storage)?;
        if let Expiry::Expired = latest_timestamp.check_expiry(&msg.packet.timeout_timestamp_on_b) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::LowPacketTimestamp,
            });
        }
        let client_id_on_b = conn_end_on_b.client_id();
        let client_state_of_a_on_b = self.client_state(deps.storage, client_id_on_b)?;
        // The client must not be frozen.
        if client_state_of_a_on_b.is_frozen() {
            return Err(ContractError::IbcPacketError {
                error: PacketError::FrozenClient {
                    client_id: client_id_on_b.clone(),
                },
            });
        }
        let consensus_state_of_a_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &msg.proof_height_on_a)?;
        let expected_commitment_on_a = commitment::compute_packet_commitment(
            &msg.packet.data,
            &msg.packet.timeout_height_on_b,
            &msg.packet.timeout_timestamp_on_b,
        );
        let commitment_path_on_a = commitment::packet_commitment_path(
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.seq_on_a,
        );
        self.verify_connection_delay_passed(
            deps.storage,
            msg.proof_height_on_a,
            conn_end_on_b.clone(),
        )?;

        let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);

        let funds = self.update_fee(info.funds.clone(), fee)?;

        let packet_data = PacketData::new(
            packet.clone(),
            msg.signer.clone(),
            None,
            cw_common::types::MessageInfo {
                sender: info.sender,
                funds,
            },
        );
        let packet_data = to_vec(&packet_data).map_err(|e| ContractError::IbcDecodeError {
            error: e.to_string(),
        })?;
        let light_client_message = LightClientMessage::VerifyPacketData {
            client_id: client_id_on_b.to_string(),
            verify_packet_data: VerifyPacketData {
                height: msg.proof_height_on_a.to_string(),
                prefix: conn_end_on_b.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_commitment_on_a.clone().into(),
                root: consensus_state_of_a_on_b.root().clone().into_vec(),
                commitment_path: commitment_path_on_a,
                commitment: expected_commitment_on_a.into_vec(),
            },
            packet_data,
        };
        let light_client_address =
            self.get_client(deps.as_ref().storage, client_id_on_b.clone().into())?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&light_client_message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            VALIDATE_ON_PACKET_RECEIVE_ON_LIGHT_CLIENT,
        );
        Ok(Response::new()
            .add_attribute("action", "Light client packet receive validate call")
            .add_submessage(sub_msg))
    }

    /// This function receives and validates a packet from a light client in an IBC channel.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// depend on.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to validate the response received from the other module
    /// after sending a packet.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn receive_packet_validate_reply_from_light_client(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let packet_data = from_binary::<PacketDataResponse>(&res).map_err(|e| {
                        ContractError::IbcDecodeError {
                            error: e.to_string(),
                        }
                    })?;
                    let info = packet_data.message_info;
                    let packet = Packet::from(packet_data.packet.clone());

                    let chan_end_on_b = self.get_channel_end(
                        deps.storage,
                        packet.port_id_on_b.clone().into(),
                        packet.chan_id_on_b.clone().into(),
                    )?;

                    if chan_end_on_b.order_matches(&Order::Ordered) {
                        let next_seq_recv = self.get_next_sequence_recv(
                            deps.storage,
                            packet.port_id_on_b.clone().into(),
                            packet.chan_id_on_b.clone().into(),
                        )?;
                        if packet.seq_on_a > next_seq_recv {
                            return Err(ContractError::IbcPacketError {
                                error: PacketError::InvalidPacketSequence {
                                    given_sequence: packet.seq_on_a,
                                    next_sequence: next_seq_recv,
                                },
                            });
                        }

                        if packet.seq_on_a == next_seq_recv {
                            // Case where the recvPacket is successful and an
                            // acknowledgement will be written (not a no-op)
                            self.validate_write_acknowledgement(deps.storage, &packet)?;
                        }
                    } else {
                        let packet_rec = self.get_packet_receipt(
                            deps.storage,
                            &packet.port_id_on_a.clone().into(),
                            &packet.chan_id_on_a.clone().into(),
                            packet.seq_on_a,
                        );
                        match packet_rec {
                            Ok(_receipt) => {}
                            Err(ContractError::IbcPacketError {
                                error: PacketError::PacketReceiptNotFound { sequence },
                            }) if sequence == packet.seq_on_a => {}
                            Err(e) => return Err(e),
                        }
                        // Case where the recvPacket is successful and an
                        // acknowledgement will be written (not a no-op)
                        self.validate_write_acknowledgement(deps.storage, &packet)?;
                    };

                    let port_id = PortId::from(packet_data.packet.port_id_on_a.clone());
                    // Getting the module address for on packet timeout call
                    let module_id = match self.lookup_module_by_port(deps.storage, port_id) {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };
                    let contract_address = match self
                        .get_route(deps.storage, cw_common::types::ModuleId::from(module_id))
                    {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };

                    let src = IbcEndpoint {
                        port_id: packet_data.packet.port_id_on_b.to_string(),
                        channel_id: packet_data.packet.chan_id_on_b.to_string(),
                    };
                    let dest = IbcEndpoint {
                        port_id: packet_data.packet.port_id_on_a.to_string(),
                        channel_id: packet_data.packet.chan_id_on_a.to_string(),
                    };
                    let data = Binary::from(packet.data);
                    let timeoutblock = match packet_data.packet.timeout_height_on_b {
                        ibc::core::ics04_channel::timeout::TimeoutHeight::Never => {
                            IbcTimeoutBlock {
                                revision: 1,
                                height: 1,
                            }
                        }
                        ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => {
                            IbcTimeoutBlock {
                                revision: x.revision_number(),
                                height: x.revision_height(),
                            }
                        }
                    };
                    let timeout = IbcTimeout::with_block(timeoutblock);
                    let ibc_packet = IbcPacket::new(
                        data,
                        src,
                        dest,
                        packet_data.packet.seq_on_a.into(),
                        timeout,
                    );
                    let address = Addr::unchecked(packet_data.signer.to_string());
                    let cosm_msg = cw_common::xcall_msg::ExecuteMsg::IbcPacketReceive {
                        msg: cosmwasm_std::IbcPacketReceiveMsg::new(ibc_packet, address),
                    };
                    let create_client_message: CosmosMsg =
                        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                            contract_addr: contract_address.to_string(),
                            msg: to_binary(&cosm_msg).unwrap(),
                            funds: info.funds,
                        });
                    let sub_msg: SubMsg = SubMsg::reply_on_success(
                        create_client_message,
                        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "channel_recieve_packet_validation")
                        .add_submessage(sub_msg))
                }
                None => Err(ContractError::IbcChannelError {
                    error: ChannelError::Other {
                        description: "Data from module is Missing".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(_) => Err(ContractError::IbcPacketError {
                error: PacketError::InvalidProof,
            }),
        }
    }

    /// This function validates if a write acknowledgement exists for a given packet and returns an
    /// error if it already exists.
    /// 
    /// Arguments:
    /// 
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `dyn` keyword indicates that `Storage` is a
    /// trait, and `store` can hold any object that implements this trait.
    /// * `packet`: The `packet` parameter is a reference to a `Packet` struct. It contains information
    /// about an IBC packet, such as the source and destination channels, ports, and sequence numbers.
    /// 
    /// Returns:
    /// 
    /// If the condition in the `if` statement is true, then an `Err` variant of `ContractError` is
    /// returned with a `PacketError::AcknowledgementExists` error. Otherwise, `Ok(())` is returned.
    pub fn validate_write_acknowledgement(
        &self,
        store: &mut dyn Storage,
        packet: &Packet,
    ) -> Result<(), ContractError> {
        if self
            .get_packet_acknowledgement(
                store,
                &packet.port_id_on_b.clone().into(),
                &packet.chan_id_on_b.clone().into(),
                packet.seq_on_a,
            )
            .is_ok()
        {
            return Err(ContractError::IbcPacketError {
                error: PacketError::AcknowledgementExists {
                    sequence: packet.seq_on_a,
                },
            });
        }
        Ok(())
    }

    /// This function handles the receiving and processing of an IBC packet and update the sequence accordingly.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a `DepsMut` object, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module on the IBC channel. It is used to handle the response received from
    /// the other module after sending a packet.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn execute_receive_packet(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let response_data =
                        from_binary::<IbcReceiveResponse>(&res).map_err(ContractError::Std)?;
                    let response_data =
                        from_binary::<XcallPacketResponseData>(&response_data.acknowledgement)
                            .map_err(ContractError::Std)?;
                    let ack = response_data.acknowledgement;
                    let packet = response_data.packet.clone();
                    let port = response_data.packet.src.port_id;
                    let chan = response_data.packet.src.channel_id;
                    let seq = response_data.packet.sequence;
                    let channel_id = ChannelId::from(
                        IbcChannelId::from_str(&chan)
                            .map_err(|error| ContractError::IbcValidationError { error })?,
                    );
                    let port_id = PortId::from(IbcPortId::from_str(&port).unwrap());
                    let chan_end_on_b =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;

                    let packet_already_received = match chan_end_on_b.ordering {
                        // Note: ibc-go doesn't make the check for `Order::None` channels
                        Order::None => false,
                        Order::Unordered => self
                            .get_packet_receipt(deps.storage, &port_id, &channel_id, seq.into())
                            .is_ok(),
                        Order::Ordered => {
                            let next_seq_recv = self.get_next_sequence_recv(
                                deps.storage,
                                port_id.clone(),
                                channel_id.clone(),
                            )?;

                            // the sequence number has already been incremented, so
                            // another relayer already relayed the packet
                            seq < Into::<u64>::into(next_seq_recv)
                        }
                    };

                    if packet_already_received {
                        return Ok(
                            Response::new().add_attribute("message", "Packet already received")
                        );
                    }
                    // state changes
                    {
                        // `recvPacket` core handler state changes
                        match chan_end_on_b.ordering {
                            Order::Unordered => {
                                self.store_packet_receipt(
                                    deps.storage,
                                    &port_id,
                                    &channel_id,
                                    seq.clone().into(),
                                    Receipt::Ok,
                                )?;
                            }
                            Order::Ordered => {
                                self.increase_next_sequence_recv(
                                    deps.storage,
                                    port_id.clone(),
                                    channel_id.clone(),
                                )?;
                            }
                            _ => {}
                        }
                        let acknowledgement: cw_common::types::Ack = from_binary(&ack.into())
                            .map_err(|e| ContractError::IbcDecodeError {
                                error: e.to_string(),
                            })?;
                        let acknowledgement = match acknowledgement {
                            cw_common::types::Ack::Result(binary) => binary,
                            cw_common::types::Ack::Error(e) => {
                                return Err(ContractError::IbcPacketError {
                                    error: PacketError::AppModule { description: e },
                                })
                            }
                        };
                        let acknowledgement = Acknowledgement::try_from(acknowledgement.0)
                            .map_err(|e| ContractError::IbcPacketError { error: e })?;
                        self.store_packet_acknowledgement(
                            deps.storage,
                            &port_id,
                            &channel_id,
                            seq.into(),
                            commitment::compute_ack_commitment(&acknowledgement),
                        )?;
                    }

                    let event_recieve_packet = create_recieve_packet_event(
                        &packet.src.port_id,
                        &packet.src.channel_id,
                        &packet.sequence.to_string(),
                        &packet.dest.port_id,
                        &packet.dest.channel_id,
                        &packet.timeout.block().unwrap().height.to_string(),
                        &packet.timeout.timestamp().unwrap().to_string(),
                        chan_end_on_b.ordering.as_str(),
                        chan_end_on_b.connection_hops[0].as_str(),
                    );
                    let write_ack_event = create_write_ack_event(
                        packet,
                        chan_end_on_b.ordering.as_str(),
                        chan_end_on_b.connection_hops[0].as_str(),
                    )?;

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "execute_receive_packet")
                        .add_attribute("message", "success: packet receive")
                        .add_attribute("message", "success: packet write acknowledgement")
                        .add_event(event_recieve_packet)
                        .add_event(write_ack_event))
                }
                None => Err(ContractError::IbcChannelError {
                    error: ChannelError::Other {
                        description: "Data from module is Missing".to_string(),
                    },
                }),
            },
            cosmwasm_std::SubMsgResult::Err(_) => Err(ContractError::IbcPacketError {
                error: PacketError::InvalidProof,
            }),
        }
    }
}
