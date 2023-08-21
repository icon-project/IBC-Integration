use cw_common::{
    cw_println,
    raw_types::{channel::RawMessageTimeout, to_raw_packet},
    to_checked_address,
};

use crate::conversions::{
    to_ibc_channel_id, to_ibc_height, to_ibc_port_id, to_ibc_timeout_block, to_ibc_timeout_height,
    to_ibc_timestamp,
};

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
        msg: RawMessageTimeout,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet.clone().unwrap();
        let src_port = to_ibc_port_id(&packet.source_port)?;
        let src_channel = to_ibc_channel_id(&packet.source_channel)?;

        let dst_port = to_ibc_port_id(&packet.destination_port)?;
        let dst_channel = to_ibc_channel_id(&packet.destination_channel)?;

        let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;
        let packet_timeout_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;
        let packet_sequence = Sequence::from(packet.sequence);
        let proof_height = to_ibc_height(msg.proof_height.clone())?;
        let next_sequence_recv = Sequence::from(msg.next_sequence_recv);

        let chan_end_on_a = self.get_channel_end(deps.storage, &src_port, &src_channel)?;
        if !chan_end_on_a.state_matches(&State::Open) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ChannelClosed {
                    channel_id: src_channel,
                },
            });
        }
        let counterparty = Counterparty::new(dst_port.clone(), Some(dst_channel.clone()));
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: dst_port,
                    channel_id: dst_channel,
                },
            });
        }
        let conn_id_on_a = chan_end_on_a.connection_hops()[0].clone();
        let conn_end_on_a = self.connection_end(deps.storage, &conn_id_on_a)?;

        let client_id=conn_end_on_a.client_id();
        let client_state= self.client_state(deps.as_ref().storage, &client_id)?;


        if client_state.is_frozen() {
            return Err(ClientError::ClientFrozen {
                client_id: client_id.clone(),
            })
            .map_err(Into::<ContractError>::into);
        }

        let commitment_on_a = match self.get_packet_commitment(
            deps.storage,
            &src_port,
            &src_channel,
            packet_sequence,
        ) {
            Ok(commitment_on_a) => commitment_on_a,

            // This error indicates that the timeout has already been relayed
            // or there is a misconfigured relayer attempting to prove a timeout
            // for a packet never sent. Core IBC will treat this error as a no-op in order to
            // prevent an entire relay transaction from failing and consuming unnecessary fees.
            Err(_) => return Ok(Response::new()),
        };

        let expected_commitment_on_a = commitment::compute_packet_commitment(
            &packet.data,
            &packet_timeout_height,
            &packet_timeout_timestamp,
        );
        if commitment_on_a != expected_commitment_on_a {
            return Err(ContractError::IbcPacketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: packet_sequence,
                },
            });
        }
        let client_id_on_a = conn_end_on_a.client_id();
        let _client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;

        if !packet_timeout_height.has_expired(proof_height) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::PacketTimeoutHeightNotReached {
                    timeout_height: packet_timeout_height,
                    chain_height: proof_height,
                },
            });
        }

        let client = self.get_client(deps.storage, client_id_on_a)?;
        let timestamp_at_height = client.get_timestamp_by_height(
            deps.as_ref(),
            client_id_on_a,
            proof_height.revision_height(),
        )?;
        if timestamp_at_height != 0 {
            let chain_timestamp = to_ibc_timestamp(timestamp_at_height)?;
            if let Expiry::NotExpired = chain_timestamp.check_expiry(&packet_timeout_timestamp) {
                return Err(PacketError::PacketTimeoutTimestampNotReached {
                    timeout_timestamp: packet_timeout_timestamp,
                    chain_timestamp: chain_timestamp,
                })
                .map_err(Into::<ContractError>::into);
            }
            cw_println!(deps, " timestamp check pass");
        }

        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &proof_height)?;

        self.verify_connection_delay_passed(
            deps.storage,
            env,
            proof_height,
            conn_end_on_a.clone(),
        )?;

        let next_seq_recv_verification_result: LightClientPacketMessage =
            if chan_end_on_a.order_matches(&Order::Ordered) {
                if packet_sequence < next_sequence_recv {
                    return Err(ContractError::IbcPacketError {
                        error: PacketError::InvalidPacketSequence {
                            given_sequence: packet_sequence,
                            next_sequence: next_sequence_recv,
                        },
                    });
                }
                let seq_recv_path_on_b =
                    commitment::next_seq_recv_commitment_path(&dst_port, &dst_channel);

                LightClientPacketMessage::VerifyNextSequenceRecv {
                    height: proof_height.to_string(),
                    prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                    proof: msg.proof_unreceived.clone(),
                    root: consensus_state_of_b_on_a.root().into_vec(),
                    seq_recv_path: seq_recv_path_on_b,
                    sequence: packet_sequence.into(),
                }
            } else {
                let receipt_path_on_b =
                    commitment::receipt_commitment_path(&dst_port, &dst_channel, packet_sequence);

                LightClientPacketMessage::VerifyPacketReceiptAbsence {
                    height: proof_height.to_string(),
                    prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                    proof: msg.proof_unreceived.clone(),
                    root: consensus_state_of_b_on_a.root().into_vec(),
                    receipt_path: receipt_path_on_b,
                }
            };

        client.verify_timeout(
            deps.as_ref(),
            client_id_on_a,
            next_seq_recv_verification_result,
        )?;

        // Getting the module address for on packet timeout call
        let contract_address = self.lookup_modules(deps.storage, src_port.as_bytes().to_vec())?;

        let src = CwEndPoint {
            port_id: src_port.to_string(),
            channel_id: src_channel.to_string(),
        };
        let dest = CwEndPoint {
            port_id: dst_port.to_string(),
            channel_id: dst_channel.to_string(),
        };
        let data = Binary::from(packet.data.clone());
        let timeoutblock = to_ibc_timeout_block(&packet_timeout_height);
        let timeout = CwTimeout::with_block(timeoutblock);
        let ibc_packet = CwPacket::new(data, src, dest, packet.sequence, timeout);
        self.store_callback_data(
            deps.storage,
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
            &ibc_packet,
        )?;

        let address = to_checked_address(deps.as_ref(), &msg.signer);
        let cosm_msg = cw_common::xcall_connection_msg::ExecuteMsg::IbcPacketTimeout {
            msg: cosmwasm_std::IbcPacketTimeoutMsg::new(ibc_packet, address),
        };
        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: contract_address,
            msg: to_binary(&cosm_msg).unwrap(),
            funds: info.funds,
        });
        let sub_msg: SubMsg =
            SubMsg::reply_on_success(create_client_message, VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE);

        Ok(Response::new()
            .add_attribute("action", "packet")
            .add_attribute("method", "packet_timeout_module_validation")
            .add_submessage(sub_msg))
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
            cosmwasm_std::SubMsgResult::Ok(_res) => {
                let packet: CwPacket = self.get_callback_data(
                    deps.as_ref().storage,
                    VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
                )?;
                let channel_id = IbcChannelId::from_str(&packet.src.channel_id).unwrap();
                let port_id = IbcPortId::from_str(&packet.src.port_id).unwrap();
                let chan_end_on_a = self.get_channel_end(deps.storage, &port_id, &channel_id)?;
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
                self.clear_callback_data(deps.storage, VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE);
                let chan_end_on_a = {
                    if let Order::Ordered = chan_end_on_a.ordering {
                        let mut chan_end_on_a = chan_end_on_a;
                        chan_end_on_a.state = State::Closed;
                        self.store_channel_end(
                            deps.storage,
                            &port_id,
                            &channel_id,
                            &chan_end_on_a,
                        )?;

                        chan_end_on_a
                    } else {
                        chan_end_on_a
                    }
                };

                let conn_id_on_a = &chan_end_on_a.connection_hops()[0];

                let event = create_packet_event(
                    IbcEventType::Timeout,
                    to_raw_packet(packet),
                    chan_end_on_a.ordering(),
                    conn_id_on_a,
                    None,
                )?;

                Ok(Response::new()
                    .add_attribute("action", "packet")
                    .add_attribute("method", "execute_timeout_packet")
                    .add_event(event))
            }
            cosmwasm_std::SubMsgResult::Err(e) => Err(ContractError::IbcContextError { error: e }),
        }
    }
}
