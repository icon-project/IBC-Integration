use cosmwasm_std::{IbcAcknowledgement, IbcPacketAckMsg};
use ibc::core::ics04_channel::msgs::acknowledgement::MsgAcknowledgement;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn acknowledgement_packet_validate(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: &MsgAcknowledgement,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet;
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ChannelClosed {
                    channel_id: packet.chan_id_on_a.clone(),
                },
            });
        }
        let counterparty = Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: packet.port_id_on_b.clone(),
                    channel_id: packet.chan_id_on_b.clone(),
                },
            });
        }
        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.clone().into())?;
        if !conn_end_on_a.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ConnectionNotOpen {
                    connection_id: chan_end_on_a.connection_hops()[0].clone(),
                },
            });
        }
        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &msg.packet.port_id_on_a.clone().into(),
            &msg.packet.chan_id_on_a.clone().into(),
            msg.packet.seq_on_a,
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };
        if commitment_on_a
            != compute_packet_commitment(
                &packet.data,
                &packet.timeout_height_on_b,
                &packet.timeout_timestamp_on_b,
            )
        {
            return Err(ContractError::IbcPacketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: packet.seq_on_a,
                },
            });
        }

        if let Order::Ordered = chan_end_on_a.ordering {
            let next_seq_ack = self.get_next_sequence_ack(
                deps.storage,
                packet.port_id_on_a.clone().into(),
                packet.chan_id_on_a.clone().into(),
            )?;
            if packet.seq_on_a != next_seq_ack {
                return Err(ContractError::IbcPacketError {
                    error: PacketError::InvalidPacketSequence {
                        given_sequence: packet.seq_on_a,
                        next_sequence: next_seq_ack,
                    },
                });
            }
        }
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_on_a = self.client_state(deps.storage, client_id_on_a)?;
        // The client must not be frozen.
        if client_state_on_a.is_frozen() {
            return Err(ContractError::IbcPacketError {
                error: PacketError::FrozenClient {
                    client_id: client_id_on_a.clone(),
                },
            });
        }
        let consensus_state =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proof_height_on_b)?;
        let ack_commitment = compute_ack_commitment(&msg.acknowledgement);
        self.verify_connection_delay_passed(
            deps.storage,
            msg.proof_height_on_b,
            conn_end_on_a.clone(),
        )?;
        let data = PacketData {
            packet: msg.packet.clone(),
            signer: msg.signer.clone(),
            acknowledgement: Some(msg.acknowledgement.clone()),
        };
        let ack_path_on_b = self.packet_acknowledgement_commitment_path(
            &packet.port_id_on_b.clone(),
            &packet.chan_id_on_b,
            packet.seq_on_a,
        );
        let verify_packet_acknowledge = VerifyPacketAcknowledgement {
            height: msg.proof_height_on_b.to_string(),
            prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
            proof: msg.proof_acked_on_b.clone().into(),
            root: consensus_state.root().clone().into_vec(),
            ack_path: ack_path_on_b,
            ack: ack_commitment.into_vec(),
        };
        let packet_data = to_vec(&data)?;
        let light_client_message = LightClientMessage::VerifyPacketAcknowledgement {
            verify_packet_acknowledge,
            packet_data,
        };
        let light_client_address =
            self.get_client(deps.as_ref().storage, client_id_on_a.clone().into())?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&light_client_message).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_LIGHT_CLIENT,
        );

        Ok(Response::new()
            .add_attribute("action", "Light client packet acklowledgement call")
            .add_submessage(sub_msg))
    }

    pub fn acknowledgement_packet_validate_reply_from_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
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
                    let packet = Packet::from(packet_data.packet.clone());
                    let acknowledgement = match packet_data.acknowledgement {
                        Some(ack) => ack,
                        None => {
                            return Err(ContractError::IbcPacketError {
                                error: PacketError::PacketAcknowledgementNotFound {
                                    sequence: packet.seq_on_a,
                                },
                            })
                        }
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
                        port_id: packet_data.packet.port_id_on_a.to_string(),
                        channel_id: packet_data.packet.chan_id_on_a.to_string(),
                    };
                    let dest = IbcEndpoint {
                        port_id: packet_data.packet.port_id_on_b.to_string(),
                        channel_id: packet_data.packet.chan_id_on_b.to_string(),
                    };
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
                    let timestamp = packet_data.packet.timeout_timestamp_on_b.nanoseconds();
                    let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
                    let timeout = IbcTimeout::with_both(timeoutblock, ibctimestamp);

                    let ibc_packet = IbcPacket::new(
                        packet.data,
                        src,
                        dest,
                        packet_data.packet.seq_on_a.into(),
                        timeout,
                    );
                    let address = Addr::unchecked(packet_data.signer.to_string());
                    let ack = IbcAcknowledgement::new(acknowledgement.as_bytes());
                    let cosm_msg = cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address);
                    let create_client_message: CosmosMsg =
                        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                            contract_addr: contract_address.to_string(),
                            msg: to_binary(&cosm_msg).unwrap(),
                            funds: info.funds,
                        });
                    let sub_msg: SubMsg = SubMsg::reply_on_success(
                        create_client_message,
                        VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "packet")
                        .add_attribute("method", "packet_acknowledgement_module")
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

    pub fn acknowledgement_packet_execute(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let reply = from_binary::<IbcPacketAckMsg>(&res).unwrap();
                    let packet = reply.original_packet;
                    let channel_id =
                        ChannelId::from(IbcChannelId::from_str(&packet.src.channel_id).unwrap());
                    let port_id = PortId::from(IbcPortId::from_str(&packet.src.port_id).unwrap());
                    let chan_end_on_a =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
                    let event = create_ack_packet_event(
                        &packet.src.port_id,
                        &packet.src.channel_id,
                        &packet.sequence.to_string(),
                        &packet.dest.port_id,
                        &packet.dest.channel_id,
                        &packet.timeout.block().unwrap().height.to_string(),
                        &packet.timeout.timestamp().unwrap().to_string(),
                        chan_end_on_a.ordering.as_str(),
                        conn_id_on_a.as_str(),
                    );
                    if self
                        .get_packet_commitment(
                            deps.storage,
                            &port_id,
                            &channel_id,
                            packet.sequence.into(),
                        )
                        .is_err()
                    {
                        return Ok(Response::new());
                    }
                    self.delete_packet_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        packet.sequence.into(),
                    )?;
                    if let Order::Ordered = chan_end_on_a.ordering {
                        // Note: in validation, we verified that `msg.packet.sequence == nextSeqRecv`
                        // (where `nextSeqRecv` is the value in the store)
                        self.increase_next_sequence_ack(deps.storage, port_id, channel_id)?;
                    }
                    Ok(Response::new()
                        .add_attribute("action", "packet")
                        .add_attribute("method", "execute_acknowledgement_packet")
                        .add_event(event))
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
