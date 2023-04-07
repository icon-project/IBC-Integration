use cosmwasm_std::{IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock};
use ibc::timestamp::Expiry;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn timeout_packet_validate_to_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: MsgTimeout,
    ) -> Result<Response, ContractError> {
        let chan_end_on_a = self.channel_end(
            deps.storage,
            &msg.packet.port_id_on_a.clone().into(),
            &msg.packet.chan_id_on_a.clone().into(),
        )?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::ChannelClosed {
                    channel_id: msg.packet.chan_id_on_a.clone(),
                },
            });
        }
        let counterparty = Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: msg.packet.port_id_on_b.clone(),
                    channel_id: msg.packet.chan_id_on_b.clone(),
                },
            });
        }
        let conn_id_on_a = chan_end_on_a.connection_hops()[0].clone();
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.into())?;
        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &msg.packet.port_id_on_a.clone().into(),
            &msg.packet.chan_id_on_a.clone().into(),
            msg.packet.seq_on_a.into(),
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };

        let expected_commitment_on_a = compute_packet_commitment(
            &msg.packet.data,
            &msg.packet.timeout_height_on_b,
            &msg.packet.timeout_timestamp_on_b,
        );
        if commitment_on_a != expected_commitment_on_a {
            return Err(ContractError::IbcPackketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: msg.packet.seq_on_a,
                },
            });
        }
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;

        if msg
            .packet
            .timeout_height_on_b
            .has_expired(msg.proof_height_on_b)
        {
            return Err(ContractError::IbcPackketError {
                error: PacketError::PacketTimeoutHeightNotReached {
                    timeout_height: msg.packet.timeout_height_on_b,
                    chain_height: msg.proof_height_on_b,
                },
            });
        }
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, &client_id_on_a, &msg.proof_height_on_b)?;
        let timestamp_of_b = consensus_state_of_b_on_a.timestamp();
        if let Expiry::Expired = msg
            .packet
            .timeout_timestamp_on_b
            .check_expiry(&timestamp_of_b)
        {
            return Err(ContractError::IbcPackketError {
                error: PacketError::PacketTimeoutTimestampNotReached {
                    timeout_timestamp: msg.packet.timeout_timestamp_on_b,
                    chain_timestamp: timestamp_of_b,
                },
            });
        }

        // self.verify_conn_delay_passed(msg.proof_height_on_b, &conn_end_on_a)?;

        let next_seq_recv_verification_result = if chan_end_on_a.order_matches(&Order::Ordered) {
            if msg.packet.seq_on_a < msg.next_seq_recv_on_b {
                return Err(ContractError::IbcPackketError {
                    error: PacketError::InvalidPacketSequence {
                        given_sequence: msg.packet.seq_on_a,
                        next_sequence: msg.next_seq_recv_on_b,
                    },
                });
            }
            let seq_recv_path_on_b = self.next_seq_recv_commitment_path(
                &msg.packet.port_id_on_b.clone(),
                &msg.packet.chan_id_on_b.clone(),
            );
            // ATTENTION: storing packet in the committment
            let data = PacketData {
                packet: msg.packet.clone(),
                signer: msg.signer,
            };
            let data = to_vec(&data).map_err(|e| ContractError::IbcDecodeError {
                error: e.to_string(),
            })?;
            self.ibc_store()
                .commitments()
                .save(deps.storage, seq_recv_path_on_b.clone(), &data)
                .map_err(|error| ContractError::Std(error))?;
            // ATTENTION: storing packet in the committmen for reply

            LightClientPacketMessage::VerifyNextSequenceRecv {
                height: msg.proof_height_on_b.to_string(),
                prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_unreceived_on_b.into(),
                root: consensus_state_of_b_on_a.root().clone().into_vec(),
                seq_recv_path: seq_recv_path_on_b,
                sequence: msg.packet.seq_on_a.into(),
            }
        } else {
            let receipt_path_on_b = self.packet_receipt_commitment_path(
                &msg.packet.port_id_on_b,
                &msg.packet.chan_id_on_b,
                msg.packet.seq_on_a,
            );
            // ATTENTION: storing packet in the committment
            let data = PacketData {
                packet: msg.packet.clone(),
                signer: msg.signer,
            };
            let data = to_vec(&data).map_err(|e| ContractError::IbcDecodeError {
                error: e.to_string(),
            })?;
            self.ibc_store()
                .commitments()
                .save(deps.storage, receipt_path_on_b.clone(), &data)
                .map_err(|error| ContractError::Std(error))?;
            // ATTENTION: storing packet in the committmen for reply

            LightClientPacketMessage::VerifyPacketReceiptAbsence {
                height: msg.proof_height_on_b.to_string(),
                prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_unreceived_on_b.into(),
                root: consensus_state_of_b_on_a.root().clone().into_vec(),
                receipt_path: receipt_path_on_b,
            }
        };
        let client_type = ClientType::from(client_state_of_b_on_a.client_type());
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&next_seq_recv_verification_result).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg = SubMsg::reply_always(
            create_client_message,
            VALIDATE_ON_PACKET_TIMEOUT_ON_LIGHT_CLIENT,
        );

        Ok(Response::new()
            .add_attribute("action", "Light client packet timeout call")
            .add_submessage(sub_msg))
    }

    pub fn timeout_packet_validate_reply_from_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let path = from_binary::<Vec<u8>>(&res).unwrap();

                    let data_bytes = self
                        .ibc_store()
                        .commitments()
                        .load(deps.storage, path.clone())
                        .map_err(|error| ContractError::Std(error))?;

                    let packet_data: PacketData = serde_json_wasm::from_slice(&data_bytes)
                        .map_err(|error| ContractError::IbcDecodeError {
                            error: error.to_string(),
                        })?;

                    //ATTENTION
                    self.ibc_store().commitments().remove(deps.storage, path);
                    //

                    let port_id = PortId::from(packet_data.packet.port_id_on_a.clone());
                    // Getting the module address for on packet timeout call
                    let module_id = match self.lookup_module_by_port(deps.storage, port_id.clone())
                    {
                        Ok(addr) => addr,
                        Err(error) => return Err(error),
                    };
                    let contract_address =
                        match self.get_route(deps.storage, types::ModuleId::from(module_id)) {
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
                    let data = Binary::from(packet_data.packet.data);
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
                    let cosm_msg = cosmwasm_std::IbcPacketTimeoutMsg::new(ibc_packet, address);
                    let create_client_message: CosmosMsg =
                        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                            contract_addr: contract_address.to_string(),
                            msg: to_binary(&cosm_msg).unwrap(),
                            funds: info.funds,
                        });
                    let sub_msg: SubMsg = SubMsg::reply_on_success(
                        create_client_message,
                        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "channel_opne_init_module_validation")
                        .add_submessage(sub_msg))
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
                return Err(ContractError::IbcPackketError {
                    error: PacketError::InvalidProof,
                })
            }
        }
    }

    pub fn execute_timeout_packet(
        &mut self,
        deps: DepsMut,
        info: MessageInfo,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary::<IbcPacket>(&res).unwrap();
                    let channel_id =
                        ChannelId::from(IbcChannelId::from_str(&data.src.channel_id).unwrap());
                    let port_id = PortId::from(IbcPortId::from_str(&data.src.port_id).unwrap());
                    let chan_end_on_a =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    if self
                        .get_packet_commitment(
                            deps.storage,
                            &port_id,
                            &channel_id,
                            data.sequence.into(),
                        )
                        .is_err()
                    {
                        return Ok(Response::new());
                    }
                    self.delete_packet_commitment(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        data.sequence.into(),
                    )?;
                    let chan_end_on_a = {
                        if let Order::Ordered = chan_end_on_a.ordering {
                            let mut chan_end_on_a = chan_end_on_a;
                            chan_end_on_a.state = State::Closed;
                            self.store_channel_end(
                                deps.storage,
                                port_id,
                                channel_id,
                                chan_end_on_a.clone(),
                            )?;

                            chan_end_on_a
                        } else {
                            chan_end_on_a
                        }
                    };

                    let events = Event::new(IbcEventType::Timeout.as_str())
                        .add_attribute("channel_order", chan_end_on_a.ordering().as_str());

                    if let Order::Ordered = chan_end_on_a.ordering {
                        let conn_id_on_a = chan_end_on_a.connection_hops()[0].clone();
                        // EVENT
                    }

                    Ok(Response::new()
                        .add_attribute("action", "channel")
                        .add_attribute("method", "execute_timeout_packet")
                        .add_event(events))
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
                return Err(ContractError::IbcPackketError {
                    error: PacketError::InvalidProof,
                })
            }
        }
    }
}
