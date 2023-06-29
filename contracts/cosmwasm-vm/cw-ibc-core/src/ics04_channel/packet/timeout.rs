use cw_common::from_binary_response;
use prost::DecodeError;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function validates a timeout packet and sends a submessage to a light client for further
    /// verification.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// use.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender and the amount of funds sent with the message.
    /// * `msg`: `msg` is a struct of type `MsgTimeout` which contains information about a timeout
    /// packet. It has the following fields:
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// the execution of the function.
    pub fn timeout_packet_validate_to_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        msg: MsgTimeout,
    ) -> Result<Response, ContractError> {
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            msg.packet.port_id_on_a.clone(),
            msg.packet.chan_id_on_a.clone(),
        )?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(PacketError::ChannelClosed {
                channel_id: msg.packet.chan_id_on_a,
            })
            .map_err(Into::<ContractError>::into);
        }
        let counterparty = Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(PacketError::InvalidPacketCounterparty {
                port_id: msg.packet.port_id_on_b.clone(),
                channel_id: msg.packet.chan_id_on_b,
            })
            .map_err(Into::<ContractError>::into);
        }
        let conn_id_on_a = chan_end_on_a.connection_hops()[0].clone();
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a)?;
        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &msg.packet.port_id_on_a,
            &msg.packet.chan_id_on_a,
            msg.packet.sequence,
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };

        let expected_commitment_on_a = commitment::compute_packet_commitment(
            &msg.packet.data,
            &msg.packet.timeout_height_on_b,
            &msg.packet.timeout_timestamp_on_b,
        );
        if commitment_on_a != expected_commitment_on_a {
            return Err(PacketError::IncorrectPacketCommitment {
                sequence: msg.packet.sequence,
            })
            .map_err(Into::<ContractError>::into);
        }
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;

        if !msg
            .packet
            .timeout_height_on_b
            .has_expired(msg.proof_height_on_b)
        {
            return Err(PacketError::PacketTimeoutHeightNotReached {
                timeout_height: msg.packet.timeout_height_on_b,
                chain_height: msg.proof_height_on_b,
            })
            .map_err(Into::<ContractError>::into);
        }
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proof_height_on_b)?;
        // let timestamp_of_b = consensus_state_of_b_on_a.timestamp();
        // if let Expiry::Expired = msg
        //     .packet
        //     .timeout_timestamp_on_b
        //     .check_expiry(&timestamp_of_b)
        // {
        //     return Err(PacketError::PacketTimeoutTimestampNotReached {
        //         timeout_timestamp: msg.packet.timeout_timestamp_on_b,
        //         chain_timestamp: timestamp_of_b,
        //     })
        //     .map_err(Into::<ContractError>::into);
        // }

        self.verify_connection_delay_passed(
            deps.storage,
            env,
            msg.proof_height_on_b,
            conn_end_on_a.clone(),
        )?;
        // let fee = self.calculate_fee(GAS_FOR_SUBMESSAGE_LIGHTCLIENT);
        //
        // let funds = self.update_fee(info.funds.clone(), fee)?;

        let data = PacketData {
            packet: msg.packet.clone(),
            signer: msg.signer.clone(),
            acknowledgement: None,
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
        };
        let packet_data = to_vec(&data).map_err(|e| ContractError::IbcDecodeError {
            error: DecodeError::new(e.to_string()),
        })?;

        let next_seq_recv_verification_result: LightClientPacketMessage =
            if chan_end_on_a.order_matches(&Order::Ordered) {
                if msg.packet.sequence < msg.next_seq_recv_on_b {
                    return Err(PacketError::InvalidPacketSequence {
                        given_sequence: msg.packet.sequence,
                        next_sequence: msg.next_seq_recv_on_b,
                    })
                    .map_err(Into::<ContractError>::into);
                }
                let seq_recv_path_on_b = commitment::next_seq_recv_commitment_path(
                    &msg.packet.port_id_on_b.clone(),
                    &msg.packet.chan_id_on_b.clone(),
                );

                LightClientPacketMessage::VerifyNextSequenceRecv {
                    height: msg.proof_height_on_b.to_string(),
                    prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                    proof: msg.proof_unreceived_on_b.clone().into(),
                    root: consensus_state_of_b_on_a.root().into_vec(),
                    seq_recv_path: seq_recv_path_on_b,
                    sequence: msg.packet.sequence.into(),
                    packet_data,
                }
            } else {
                let receipt_path_on_b = commitment::receipt_commitment_path(
                    &msg.packet.port_id_on_b,
                    &msg.packet.chan_id_on_b,
                    msg.packet.sequence,
                );

                LightClientPacketMessage::VerifyPacketReceiptAbsence {
                    height: msg.proof_height_on_b.to_string(),
                    prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                    proof: msg.proof_unreceived_on_b.clone().into(),
                    root: consensus_state_of_b_on_a.root().into_vec(),
                    receipt_path: receipt_path_on_b,
                    packet_data,
                }
            };
        let client_type = client_state_of_b_on_a.client_type();
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;

        let payload = cw_common::client_msg::ExecuteMsg::PacketTimeout {
            client_id: client_id_on_a.to_string(),
            next_seq_recv_verification_result: next_seq_recv_verification_result,
        };
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&payload).unwrap(),
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

    /// This function validates a reply from a light client for a timeout packet in channel.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage and other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message execution.
    /// It is used to extract the data returned by the sub-message and perform further actions based on
    /// it.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can occur
    /// during contract execution.
    pub fn timeout_packet_validate_reply_from_light_client(
        &self,
        deps: DepsMut,

        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let packet_data =
                        from_binary_response::<PacketDataResponse>(&res).map_err(|e| {
                            ContractError::IbcDecodeError {
                                error: DecodeError::new(e.to_string()),
                            }
                        })?;
                    let info = packet_data.message_info;
                    let data = Packet::from(packet_data.packet.clone());
                    let port_id = packet_data.packet.port_id_on_a.clone();
                    // Getting the module address for on packet timeout call
                    let contract_address =
                        match self.lookup_modules(deps.storage, port_id.as_bytes().to_vec()) {
                            Ok(addr) => addr,
                            Err(error) => return Err(error),
                        };

                    let src = CwEndPoint {
                        port_id: packet_data.packet.port_id_on_a.to_string(),
                        channel_id: packet_data.packet.chan_id_on_a.to_string(),
                    };
                    let dest = CwEndPoint {
                        port_id: packet_data.packet.port_id_on_b.to_string(),
                        channel_id: packet_data.packet.chan_id_on_b.to_string(),
                    };
                    let data = Binary::from(data.data);
                    let timeoutblock = match packet_data.packet.timeout_height_on_b {
                        common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => {
                            CwTimeoutBlock {
                                revision: 1,
                                height: 1,
                            }
                        }
                        common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => {
                            CwTimeoutBlock {
                                revision: x.revision_number(),
                                height: x.revision_height(),
                            }
                        }
                    };
                    let timeout = CwTimeout::with_block(timeoutblock);
                    let ibc_packet =
                        CwPacket::new(data, src, dest, packet_data.packet.seq_on_a.into(), timeout);
                    let address = Addr::unchecked(packet_data.signer.to_string());
                    let cosm_msg = cw_common::xcall_msg::ExecuteMsg::IbcPacketTimeout {
                        msg: cosmwasm_std::IbcPacketTimeoutMsg::new(ibc_packet, address),
                    };
                    let create_client_message: CosmosMsg =
                        CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                            contract_addr: contract_address,
                            msg: to_binary(&cosm_msg).unwrap(),
                            funds: info.funds,
                        });
                    let sub_msg: SubMsg = SubMsg::reply_on_success(
                        create_client_message,
                        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
                    );

                    Ok(Response::new()
                        .add_attribute("action", "packet")
                        .add_attribute("method", "packet_timeout_module_validation")
                        .add_submessage(sub_msg))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },

            cosmwasm_std::SubMsgResult::Err(e) => Err(ContractError::IbcContextError { error: e }),
            // cosmwasm_std::SubMsgResult::Err(_) => {
            // Err(PacketError::InvalidProof).map_err(Into::<ContractError>::into)
            // }
        }
    }

    /// This function handles the execution of a timeout packet after successfull validation of
    /// light client and xcall.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a mutable reference to the dependencies of the contract, which includes
    /// access to the storage and other modules.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to process the result of an IBC packet timeout.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn execute_timeout_packet(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    let data = from_binary_response::<CwPacket>(&res).unwrap();
                    let channel_id = IbcChannelId::from_str(&data.src.channel_id).unwrap();
                    let port_id = IbcPortId::from_str(&data.src.port_id).unwrap();
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

                    let event = Event::new(IbcEventType::Timeout.as_str())
                        .add_attribute("channel_order", chan_end_on_a.ordering().as_str());
                    let events = vec![event];
                    if let Order::Ordered = chan_end_on_a.ordering {
                        // let close_init = create_packet_timeout_event(,chann);
                        // events.push(close_init);
                    }

                    Ok(Response::new()
                        .add_attribute("action", "packet")
                        .add_attribute("method", "execute_timeout_packet")
                        .add_events(events))
                }
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(Into::<ContractError>::into),
            },
            cosmwasm_std::SubMsgResult::Err(e) => Err(ContractError::IbcContextError { error: e }),
            // cosmwasm_std::SubMsgResult::Err(_) => {
            // Err(PacketError::InvalidProof).map_err(Into::<ContractError>::into)
            // }
        }
    }
}
