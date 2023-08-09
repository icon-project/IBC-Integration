use crate::conversions::{
    to_ibc_channel_id, to_ibc_height, to_ibc_port_id, to_ibc_timeout_block, to_ibc_timeout_height,
    to_ibc_timestamp,
};

use super::*;

use cosmwasm_std::IbcPacketAckMsg;
use cw_common::{
    cw_types::{CwAcknowledgement, CwPacketAckMsg},
    raw_types::{channel::RawMessageAcknowledgement, to_raw_packet},
};

use cw_common::cw_println;

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
        _info: MessageInfo,
        env: Env,
        msg: &RawMessageAcknowledgement,
    ) -> Result<Response, ContractError> {
        cw_println!(deps, "inside acknowledge packet validate ");
        let packet = msg.packet.clone().unwrap();
        let src_port = to_ibc_port_id(&packet.source_port)?;
        let src_channel = to_ibc_channel_id(&packet.source_channel)?;

        let dst_port = to_ibc_port_id(&packet.destination_port)?;
        let dst_channel = to_ibc_channel_id(&packet.destination_channel)?;
        let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;
        let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;
        let packet_sequence = Sequence::from(packet.sequence);
        let proof_height = to_ibc_height(msg.proof_height.clone())?;

        let chan_end_on_a = self.get_channel_end(deps.storage, &src_port, &src_channel)?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ChannelClosed {
                    channel_id: src_channel,
                },
            });
        }
        cw_println!(deps, "chan end on a  state matched  ");

        let counterparty = Counterparty::new(dst_port.clone(), Some(dst_channel.clone()));
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: dst_port,
                    channel_id: dst_channel,
                },
            });
        }
        cw_println!(deps, "counterparty matched");

        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a)?;
        if !conn_end_on_a.state_matches(&ConnectionState::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ConnectionNotOpen {
                    connection_id: chan_end_on_a.connection_hops()[0].clone(),
                },
            });
        }
        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &src_port,
            &src_channel,
            Sequence::from(packet.sequence),
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };
        cw_println!(
            deps,
            "Commitment on a {:?}",
            hex::encode(commitment_on_a.clone())
        );

        cw_println!(
            deps,
            "from packet the timeout height is :{:?}",
            packet_timeout_height
        );
        let compouted_packet_commitment = commitment::compute_packet_commitment(
            &packet.data,
            &packet_timeout_height,
            &packet_timestamp,
        );
        cw_println!(
            deps,
            "computed packet commitment  {:?}",
            hex::encode(&compouted_packet_commitment)
        );

        if commitment_on_a != compouted_packet_commitment {
            return Err(ContractError::IbcPacketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: packet_sequence,
                },
            });
        }

        cw_println!(deps, "packet commitment matched");

        if let Order::Ordered = chan_end_on_a.ordering {
            let next_seq_ack = self.get_next_sequence_ack(deps.storage, &src_port, &src_channel)?;
            if packet_sequence != next_seq_ack {
                return Err(ContractError::IbcPacketError {
                    error: PacketError::InvalidPacketSequence {
                        given_sequence: packet_sequence,
                        next_sequence: next_seq_ack,
                    },
                });
            }
        }
        cw_println!(deps, "packet seq matched");

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
        let consensus_state = self.consensus_state(deps.storage, client_id_on_a, &proof_height)?;
        self.verify_connection_delay_passed(
            deps.storage,
            env,
            proof_height,
            conn_end_on_a.clone(),
        )?;

        let ack_path_on_b =
            commitment::acknowledgement_commitment_path(&dst_port, &dst_channel, packet_sequence);
        let verify_packet_acknowledge = VerifyPacketAcknowledgement {
            height: proof_height.to_string(),
            prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
            proof: msg.proof_acked.clone(),
            root: consensus_state.root().into_vec(),
            ack_path: ack_path_on_b,
            ack: msg.acknowledgement.clone(),
        };

        let client = self.get_client(deps.as_ref().storage, client_id_on_a)?;

        client.verify_packet_acknowledge(
            deps.as_ref(),
            verify_packet_acknowledge,
            client_id_on_a,
        )?;

        let acknowledgement = msg.acknowledgement.clone();
        cw_println!(deps, "after matching ackowledgement ");

        // Getting the module address for on packet timeout call
        let contract_address = self.lookup_modules(deps.storage, dst_port.as_bytes().to_vec())?;

        let src = CwEndPoint {
            port_id: src_port.to_string(),
            channel_id: src_channel.to_string(),
        };
        let dest = CwEndPoint {
            port_id: dst_port.to_string(),
            channel_id: dst_channel.to_string(),
        };
        let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
        let timestamp = packet_timestamp.nanoseconds();
        let cw_timestamp = cosmwasm_std::Timestamp::from_nanos(timestamp);
        let timeout = CwTimeout::with_both(timeoutblock, cw_timestamp);

        let cw_packet = CwPacket::new(packet.data, src, dest, packet.sequence, timeout);
        let address = Addr::unchecked(msg.signer.to_string());
        let ack: CwAcknowledgement = CwAcknowledgement::new(acknowledgement);
        let packet_ack_msg: CwPacketAckMsg =
            cosmwasm_std::IbcPacketAckMsg::new(ack, cw_packet, address);
        self.store_callback_data(
            deps.storage,
            VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
            &packet_ack_msg,
        )?;
        let cosm_msg = cw_common::xcall_connection_msg::ExecuteMsg::IbcPacketAck {
            msg: packet_ack_msg,
        };

        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: contract_address,
            msg: to_binary(&cosm_msg).unwrap(),
            funds: vec![],
        });
        cw_println!(
            deps,
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
        cw_println!(deps, "replying from ack module {:?}", message);
        match message.result {
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                cw_println!(deps, "receiving reply from packet ack ");

                let reply: IbcPacketAckMsg = self.get_callback_data(
                    deps.as_ref().storage,
                    VALIDATE_ON_PACKET_ACKNOWLEDGEMENT_ON_MODULE,
                )?;
                cw_println!(deps, "received ack message from module ");

                let packet = reply.original_packet;
                let channel_id = IbcChannelId::from_str(&packet.src.channel_id).unwrap();
                let port_id = IbcPortId::from_str(&packet.src.port_id).unwrap();
                let chan_end_on_a = self.get_channel_end(deps.storage, &port_id, &channel_id)?;
                let conn_id_on_a = &chan_end_on_a.connection_hops()[0];

                let event = create_packet_event(
                    IbcEventType::AckPacket,
                    to_raw_packet(packet.clone()),
                    &chan_end_on_a.ordering,
                    conn_id_on_a,
                    None,
                )?;
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

                cw_println!(deps, " after getting packet commitment ");

                // TODO: check ack_commitment returned from module
                self.delete_packet_commitment(
                    deps.storage,
                    &port_id,
                    &channel_id,
                    packet.sequence.into(),
                )?;
                // reset height to zero once packet is ack
                self.ibc_store().store_sent_packet(
                    deps.storage,
                    &port_id,
                    &channel_id,
                    packet.sequence,
                    0,
                )?;

                if let Order::Ordered = chan_end_on_a.ordering {
                    // Note: in validation, we verified that `msg.packet.sequence == nextSeqRecv`
                    // (where `nextSeqRecv` is the value in the store)
                    self.increase_next_sequence_ack(deps.storage, &port_id, &channel_id)?;
                }
                Ok(Response::new()
                    .add_attribute("action", "packet")
                    .add_attribute("method", "execute_acknowledgement_packet")
                    .add_event(event))
            }
            cosmwasm_std::SubMsgResult::Err(e) => {
                cw_println!(deps, "error from module ack reply");
                Err(PacketError::Other(e)).map_err(Into::<ContractError>::into)?
            }
        }
    }
}
