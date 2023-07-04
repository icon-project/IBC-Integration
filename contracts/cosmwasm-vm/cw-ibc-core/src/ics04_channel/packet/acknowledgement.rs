use common::ibc::core::ics04_channel::{
    msgs::acknowledgement::MsgAcknowledgement, timeout::TimeoutHeight,
};
use cw_common::{
    cw_types::{CwAcknowledgement, CwPacketAckMsg},
    from_binary_response,
};
use debug_print::debug_println;
use prost::DecodeError;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function validates an acknowledgement packet.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which provides access to the contract's dependencies
    /// such as storage, API, and querier.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender and the amount of funds sent with the message.
    /// * `msg`: The `msg` parameter is a reference to a `MsgAcknowledgement` struct, which contains
    /// information about the acknowledgement packet being validated.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// the execution of the function.
    pub fn acknowledgement_packet_validate(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        msg: &MsgAcknowledgement,
    ) -> Result<Response, ContractError> {
        debug_println!("inside acknowledge packet validate ");
        let packet = &msg.packet;
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            packet.port_id_on_a.clone(),
            packet.chan_id_on_a.clone(),
        )?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(PacketError::ChannelClosed {
                channel_id: packet.chan_id_on_a.clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        debug_println!("chan end on a  state matched  ");

        let counterparty = Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(PacketError::InvalidPacketCounterparty {
                port_id: packet.port_id_on_b.clone(),
                channel_id: packet.chan_id_on_b.clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        debug_println!("counterparty matched");

        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.clone())?;
        if !conn_end_on_a.state_matches(&ConnectionState::Open) {
            return Err(PacketError::ConnectionNotOpen {
                connection_id: chan_end_on_a.connection_hops()[0].clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &msg.packet.port_id_on_a.clone(),
            &msg.packet.chan_id_on_a.clone(),
            msg.packet.sequence,
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };
        debug_println!("Commitment on a {:?}", hex::encode(commitment_on_a.clone()));
        debug_println!(
            "from packet the timeout height is :{:?}",
            packet.timeout_height_on_b
        );
        let compouted_packet_commitment = commitment::compute_packet_commitment(
            &packet.data,
            &packet.timeout_height_on_b,
            &packet.timeout_timestamp_on_b,
        );
        debug_println!(
            "computed packet commitment  {:?}",
            hex::encode(compouted_packet_commitment)
        );

        if commitment_on_a
            != commitment::compute_packet_commitment(
                &packet.data,
                &packet.timeout_height_on_b,
                &packet.timeout_timestamp_on_b,
            )
        {
            return Err(PacketError::IncorrectPacketCommitment {
                sequence: packet.sequence,
            })
            .map_err(Into::<ContractError>::into)?;
        }

        debug_println!("packet commitment matched");

        if let Order::Ordered = chan_end_on_a.ordering {
            let next_seq_ack = self.get_next_sequence_ack(
                deps.storage,
                packet.port_id_on_a.clone(),
                packet.chan_id_on_a.clone(),
            )?;
            if packet.sequence != next_seq_ack {
                return Err(PacketError::InvalidPacketSequence {
                    given_sequence: packet.sequence,
                    next_sequence: next_seq_ack,
                })
                .map_err(Into::<ContractError>::into)?;
            }
        }
        debug_println!("packet seq matched");

        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_on_a = self.client_state(deps.storage, client_id_on_a)?;
        // The client must not be frozen.
        if client_state_on_a.is_frozen() {
            return Err(PacketError::FrozenClient {
                client_id: client_id_on_a.clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let consensus_state =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proof_height_on_b)?;
        // let ack_commitment = commitment::compute_ack_commitment();
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
            message_info: cw_common::types::MessageInfo {
                sender: info.sender,
                funds: vec![],
            },
            packet: msg.packet.clone(),
            signer: msg.signer.clone(),
            acknowledgement: Some(msg.acknowledgement.clone()),
        };
        let ack_path_on_b = commitment::acknowledgement_commitment_path(
            &packet.port_id_on_b.clone(),
            &packet.chan_id_on_b,
            packet.sequence,
        );
        let verify_packet_acknowledge = VerifyPacketAcknowledgement {
            height: msg.proof_height_on_b.to_string(),
            prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
            proof: msg.proof_acked_on_b.clone().into(),
            root: consensus_state.root().into_vec(),
            ack_path: ack_path_on_b,
            ack: msg.acknowledgement.clone().into(),
        };
        let packet_data = to_vec(&data)?;
        let light_client_message = LightClientMessage::VerifyPacketAcknowledgement {
            client_id: client_id_on_a.to_string(),
            verify_packet_acknowledge,
            packet_data,
        };
        let light_client_address =
            self.get_client(deps.as_ref().storage, client_id_on_a.clone())?;
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

    /// This function validates a reply from a light client for an acknowledgement packet in an IBC
    /// channel.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// depend on.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to a light client. It is used to validate the acknowledgement packet received from the
    /// light client.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// contract execution and `ContractError` is an enum representing the possible errors that can
    /// occur during contract execution.
    pub fn acknowledgement_packet_validate_reply_from_light_client(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => {
                match res.data {
                    Some(res) => {
                        debug_println!("inside ack validate reply ");
                        let packet_data = from_binary_response::<PacketDataResponse>(&res)
                            .map_err(|e| ContractError::IbcDecodeError {
                                error: DecodeError::new(e.to_string()),
                            })?;
                        let _info = packet_data.message_info;
                        let packet = Packet::from(packet_data.packet.clone());
                        let acknowledgement = match packet_data.acknowledgement {
                            Some(ack) => ack,
                            None => {
                                return Err(PacketError::PacketAcknowledgementNotFound {
                                    sequence: packet.sequence,
                                })
                                .map_err(Into::<ContractError>::into)?;
                            }
                        };
                        debug_println!("after matching ackowledgement ");

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
                        let timestamp = packet_data.packet.timeout_timestamp_on_b.nanoseconds();
                        let ibctimestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
                        let timeout = CwTimeout::with_both(timeoutblock, ibctimestamp);

                        let ibc_packet = CwPacket::new(
                            packet.data,
                            src,
                            dest,
                            packet_data.packet.seq_on_a.into(),
                            timeout,
                        );
                        let address = Addr::unchecked(packet_data.signer.to_string());
                        let ack: CwAcknowledgement =
                            CwAcknowledgement::new(acknowledgement.as_bytes());
                        let cosm_msg = cw_common::xcall_msg::ExecuteMsg::IbcPacketAck {
                            msg: cosmwasm_std::IbcPacketAckMsg::new(ack, ibc_packet, address),
                        };
                        let create_client_message: CosmosMsg =
                            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                                contract_addr: contract_address,
                                msg: to_binary(&cosm_msg).unwrap(),
                                funds: vec![],
                            });
                        debug_println!(
                            "after creating client message {:?} ",
                            create_client_message
                        );

                        let sub_msg: SubMsg = SubMsg::reply_on_success(
                            create_client_message,
                            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
                        );

                        Ok(Response::new()
                            .add_attribute("action", "packet")
                            .add_attribute("method", "packet_acknowledgement_module")
                            .add_submessage(sub_msg))
                    }
                    None => Err(ChannelError::Other {
                        description: "Data from module is Missing".to_string(),
                    })
                    .map_err(Into::<ContractError>::into)?,
                }
            }

            cosmwasm_std::SubMsgResult::Err(e) => Err(ContractError::IbcContextError { error: e }),
        }
    }

    /// This function processes an acknowledgement packet from xcall and produce event for acknowledgement
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` object, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// use.
    /// * `message`: `message` is a `Reply` struct that contains the result of a sub-message sent by the
    /// contract to another module. It is used to extract the acknowledgement packet message and perform
    /// necessary actions based on the result.
    ///
    /// Returns:
    ///
    /// a `Result` with either a `Response` or a `ContractError`.
    pub fn acknowledgement_packet_execute(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError> {
        debug_println!("replying from ack module {:?}", message);
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(res) => match res.data {
                Some(res) => {
                    debug_println!("receiving reply from packet ack ");

                    let reply = from_binary_response::<CwPacketAckMsg>(&res).unwrap();
                    debug_println!("received ack message from module ");

                    let packet = reply.original_packet;
                    let channel_id = IbcChannelId::from_str(&packet.src.channel_id).unwrap();
                    let port_id = IbcPortId::from_str(&packet.src.port_id).unwrap();
                    let chan_end_on_a =
                        self.get_channel_end(deps.storage, port_id.clone(), channel_id.clone())?;
                    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];

                    let timeout_height = packet
                        .timeout
                        .block()
                        .and_then(|b| Height::new(b.revision, b.height).ok())
                        .map(|h| h.to_string())
                        .unwrap_or("0-0".to_string());
                    let event = create_ack_packet_event(
                        &packet.src.port_id,
                        &packet.src.channel_id,
                        &packet.sequence.to_string(),
                        &packet.dest.port_id,
                        &packet.dest.channel_id,
                        &timeout_height,
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

                    debug_println!(" after getting packet commitment ");

                    // TODO: check ack_commitment returned from module
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
                None => Err(ChannelError::Other {
                    description: "Data from module is Missing".to_string(),
                })
                .map_err(Into::<ContractError>::into)?,
            },
            cosmwasm_std::SubMsgResult::Err(_) => {
                debug_println!("error from module ack reply");
                Err(PacketError::InvalidAcknowledgement).map_err(Into::<ContractError>::into)?
            }
        }
    }
}
