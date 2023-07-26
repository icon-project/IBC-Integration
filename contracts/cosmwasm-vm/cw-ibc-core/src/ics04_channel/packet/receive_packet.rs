use common::ibc::core::ics04_channel::{msgs::acknowledgement::Acknowledgement, packet::Receipt};
use cw_common::{
    hex_string::HexString,
    raw_types::{
        channel::{RawMessageRecvPacket, RawPacket},
        to_raw_packet,
    },
};
use debug_print::debug_println;

use crate::conversions::{
    to_ibc_channel_id, to_ibc_height, to_ibc_port_id, to_ibc_timeout_block, to_ibc_timeout_height,
    to_ibc_timestamp,
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
        env: Env,
        msg: &RawMessageRecvPacket,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet.clone().unwrap();
        let src_port = to_ibc_port_id(&packet.source_port)?;
        let src_channel = to_ibc_channel_id(&packet.source_channel)?;

        let dst_port = to_ibc_port_id(&packet.destination_port)?;
        let dst_channel = to_ibc_channel_id(&packet.destination_channel)?;

        let channel_end = self.get_channel_end(deps.storage, &dst_port, &dst_channel)?;
        if !channel_end.state_matches(&State::Open) {
            return Err(PacketError::InvalidChannelState {
                channel_id: dst_channel,
                state: channel_end.state,
            })
            .map_err(Into::<ContractError>::into)?;
        }
        debug_println!("validate recevie packet state_matched");
        let counterparty = Counterparty::new(src_port.clone(), Some(src_channel.clone()));

        if !channel_end.counterparty_matches(&counterparty) {
            return Err(PacketError::InvalidPacketCounterparty {
                port_id: src_port,
                channel_id: src_channel,
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let connection_id = &channel_end.connection_hops()[0];
        let conn_end_on_b = self.connection_end(deps.storage, connection_id)?;
        if !conn_end_on_b.state_matches(&ConnectionState::Open) {
            return Err(PacketError::ConnectionNotOpen {
                connection_id: channel_end.connection_hops()[0].clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        let latest_height = self.host_height(&env)?;
        let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;

        if packet_timeout_height.has_expired(latest_height) {
            return Err(PacketError::LowPacketHeight {
                chain_height: latest_height,
                timeout_height: packet_timeout_height,
            })
            .map_err(Into::<ContractError>::into)?;
        }
        debug_println!("packet height is greater than timeout height");

        let client_id_on_b = conn_end_on_b.client_id();
        let client_state_of_a_on_b = self.client_state(deps.storage, client_id_on_b)?;
        // The client must not be frozen.
        if client_state_of_a_on_b.is_frozen() {
            return Err(PacketError::FrozenClient {
                client_id: client_id_on_b.clone(),
            })
            .map_err(Into::<ContractError>::into)?;
        }
        debug_println!("client state created ",);

        let proof_height = to_ibc_height(msg.proof_height.clone())?;

        let consensus_state_of_a_on_b =
            self.consensus_state(deps.storage, client_id_on_b, &proof_height)?;
        let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;

        let expected_commitment_on_a = commitment::compute_packet_commitment_bytes(
            &packet.data,
            &packet_timeout_height,
            &packet_timestamp,
        );
        debug_println!("packet is -> {:?}", msg.packet);
        debug_println!(
            "packet.data is -> {:?}",
            HexString::from_bytes(&packet.data)
        );
        debug_println!("expected commitement created {:?}", packet.sequence);

        let commitment_path_on_a = commitment::packet_commitment_path(
            &src_port,
            &src_channel,
            Sequence::from(packet.sequence),
        );

        debug_println!("verify connection delay passed");
        let verify_packet_data = VerifyPacketData {
            height: proof_height.to_string(),
            prefix: conn_end_on_b.counterparty().prefix().clone().into_vec(),
            proof: msg.proof_commitment.clone(),
            root: consensus_state_of_a_on_b.root().into_vec(),
            commitment_path: commitment_path_on_a,
            commitment: expected_commitment_on_a.into_vec(),
        };

        let client = self.get_client(deps.as_ref().storage, client_id_on_b)?;
        client.verify_packet_data(deps.as_ref(), verify_packet_data, client_id_on_b)?;
        let packet_sequence = Sequence::from(packet.sequence);

        if channel_end.order_matches(&Order::Ordered) {
            let next_seq_recv =
                self.get_next_sequence_recv(deps.storage, &dst_port, &dst_channel)?;
            if packet_sequence > next_seq_recv {
                return Err(PacketError::InvalidPacketSequence {
                    given_sequence: packet_sequence,
                    next_sequence: next_seq_recv,
                })
                .map_err(Into::<ContractError>::into)?;
            }

            if packet_sequence == next_seq_recv {
                // Case where the recvPacket is successful and an
                // acknowledgement will be written (not a no-op)
                self.validate_write_acknowledgement(deps.storage, packet)?;
            }
        } else {
            self.validate_write_acknowledgement(deps.storage, packet)?;
        };

        let port_id = packet.destination_port.clone();
        // Getting the module address for on packet timeout call
        let contract_address = match self.lookup_modules(deps.storage, port_id.as_bytes().to_vec())
        {
            Ok(addr) => addr,
            Err(error) => return Err(error),
        };

        let src = CwEndPoint {
            port_id: packet.source_port.to_string(),
            channel_id: packet.source_channel.to_string(),
        };
        let dest = CwEndPoint {
            port_id: packet.destination_port.to_string(),
            channel_id: packet.destination_channel.to_string(),
        };
        let data = Binary::from(packet.data.clone());
        let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
        let timeout = CwTimeout::with_block(timeoutblock);
        let ibc_packet = CwPacket::new(data, src, dest, packet.sequence, timeout);
        let address = Addr::unchecked(msg.signer.to_string());
        self.store_callback_data(
            deps.storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &ibc_packet,
        )?;
        let cosm_msg = cw_common::xcall_connection_msg::ExecuteMsg::IbcPacketReceive {
            msg: cosmwasm_std::IbcPacketReceiveMsg::new(ibc_packet, address),
        };
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: contract_address,
            msg: to_binary(&cosm_msg).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg =
            SubMsg::reply_always(create_client_message, VALIDATE_ON_PACKET_RECEIVE_ON_MODULE);

        Ok(Response::new()
            .add_attribute("action", "channel")
            .add_attribute("method", "channel_recieve_packet_validation")
            .add_submessage(sub_msg))
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
    /// about an IBC packet, such as the source and destination channels, ports, and seq_on_a numbers.
    ///
    /// Returns:
    ///
    /// If the condition in the `if` statement is true, then an `Err` variant of `ContractError` is
    /// returned with a `PacketError::AcknowledgementExists` error. Otherwise, `Ok(())` is returned.
    pub fn validate_write_acknowledgement(
        &self,
        store: &mut dyn Storage,
        packet: &RawPacket,
    ) -> Result<(), ContractError> {
        let packet_sequence = Sequence::from(packet.sequence);
        if self
            .get_packet_acknowledgement(
                store,
                &to_ibc_port_id(&packet.destination_port)?,
                &to_ibc_channel_id(&packet.destination_channel)?,
                Sequence::from(packet.sequence),
            )
            .is_ok()
        {
            return Err(ContractError::IbcPacketError {
                error: PacketError::AcknowledgementExists {
                    sequence: packet_sequence,
                },
            });
        }
        Ok(())
    }

    /// This function handles the receiving and processing of an IBC packet and update the seq_on_a accordingly.
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
            cosmwasm_std::SubMsgResult::Ok(res) => {
                let ack: Vec<u8> = match res.data {
                    Some(data) => data.0,
                    None => Vec::new(),
                };
                let packet: CwPacket = self.get_callback_data(
                    deps.as_ref().storage,
                    VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
                )?;
                let port = packet.dest.port_id.clone();
                let chan = packet.dest.channel_id.clone();
                let seq = packet.sequence;
                let channel_id =
                    IbcChannelId::from_str(&chan).map_err(Into::<ContractError>::into)?;
                let port_id = IbcPortId::from_str(&port).unwrap();

                let chan_end_on_b = self.get_channel_end(deps.storage, &port_id, &channel_id)?;
                debug_println!("execute_receive_packet decoding of data successful");
                let packet_already_received = match chan_end_on_b.ordering {
                    // Note: ibc-go doesn't make the check for `Order::None` channels
                    Order::None => false,
                    Order::Unordered => self
                        .get_packet_receipt(deps.storage, &port_id, &channel_id, seq.into())
                        .is_ok(),
                    Order::Ordered => {
                        let next_seq_recv =
                            self.get_next_sequence_recv(deps.storage, &port_id, &channel_id)?;

                        // the seq_on_a number has already been incremented, so
                        // another relayer already relayed the packet
                        seq < Into::<u64>::into(next_seq_recv)
                    }
                };

                debug_println!("before packet already received ");
                if packet_already_received {
                    return Ok(Response::new().add_attribute("message", "Packet already received"));
                }

                debug_println!("before channel ordering check");

                // `recvPacket` core handler state changes
                match chan_end_on_b.ordering {
                    Order::Unordered => {
                        self.store_packet_receipt(
                            deps.storage,
                            &port_id,
                            &channel_id,
                            seq.into(),
                            Receipt::Ok,
                        )?;
                    }
                    Order::Ordered => {
                        self.increase_next_sequence_recv(deps.storage, &port_id, &channel_id)?;
                    }
                    _ => {}
                }
                debug_println!("before after channel ordering check");

                let event_recieve_packet = create_packet_event(
                    IbcEventType::ReceivePacket,
                    to_raw_packet(packet.clone()),
                    chan_end_on_b.ordering(),
                    &chan_end_on_b.connection_hops[0],
                    None,
                )?;

                debug_println!("event recieve packet: {:?}", event_recieve_packet);

                let mut res = Response::new()
                    .add_attribute("action", "channel")
                    .add_attribute("method", "execute_receive_packet")
                    .add_attribute("message", "success: packet receive")
                    .add_event(event_recieve_packet);

                if !ack.is_empty() {
                    self.store_packet_acknowledgement(
                        deps.storage,
                        &port_id,
                        &channel_id,
                        seq.into(),
                        commitment::compute_ack_commitment(&Acknowledgement::from_bytes(&ack)),
                    )?;

                    let write_ack_event = create_packet_event(
                        IbcEventType::WriteAck,
                        to_raw_packet(packet),
                        &chan_end_on_b.ordering,
                        &chan_end_on_b.connection_hops[0],
                        Some(ack),
                    )?;

                    res = res
                        .add_attribute("message", "success: packet write acknowledgement")
                        .add_event(write_ack_event);
                }

                Ok(res)
            }
            cosmwasm_std::SubMsgResult::Err(e) => Err(ContractError::IbcContextError { error: e }),
        }
    }

    pub fn timeout_height_to_str(&self, timeout: CwTimeoutBlock) -> String {
        format!("{0}-{1}", timeout.revision, timeout.height)
    }
}
