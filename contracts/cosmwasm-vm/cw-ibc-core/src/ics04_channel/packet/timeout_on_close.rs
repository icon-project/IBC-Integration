use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn timeout_on_close_packet_validate_to_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        msg: &MsgTimeoutOnClose,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet.clone();
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            msg.packet.port_id_on_a.clone().into(),
            msg.packet.chan_id_on_a.clone().into(),
        )?;
        let counterparty = Counterparty::new(
            msg.packet.port_id_on_b.clone(),
            Some(msg.packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
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
            msg.packet.seq_on_a,
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
            return Err(ContractError::IbcPacketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: msg.packet.seq_on_a,
                },
            });
        }
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;

        if client_state_of_b_on_a.is_frozen() {
            return Err(ContractError::IbcPacketError {
                error: PacketError::FrozenClient {
                    client_id: client_id_on_a.clone(),
                },
            });
        }
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &msg.proof_height_on_b)?;
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
        let port_id_on_b = chan_end_on_a.counterparty().port_id.clone();
        let chan_id_on_b =
            chan_end_on_a
                .counterparty()
                .channel_id()
                .ok_or(ContractError::IbcPacketError {
                    error: PacketError::Channel(ChannelError::InvalidCounterpartyChannelId),
                })?;
        let conn_id_on_b =
            conn_end_on_a
                .counterparty()
                .connection_id()
                .ok_or(ContractError::IbcPacketError {
                    error: PacketError::UndefinedConnectionCounterparty {
                        connection_id: chan_end_on_a.connection_hops()[0].clone(),
                    },
                })?;
        let expected_conn_hops_on_b = vec![conn_id_on_b.clone()];
        let expected_counterparty = Counterparty::new(
            packet.port_id_on_a.clone(),
            Some(packet.chan_id_on_a.clone()),
        );
        let expected_chan_end_on_b = ChannelEnd::new(
            State::Closed,
            *chan_end_on_a.ordering(),
            expected_counterparty,
            expected_conn_hops_on_b,
            chan_end_on_a.version().clone(),
        );
        let chan_end_path_on_b = self.channel_path(&port_id_on_b, chan_id_on_b);
        let vector = to_vec(&expected_chan_end_on_b);

        self.verify_connection_delay_passed(
            deps.storage,
            msg.proof_height_on_b,
            conn_end_on_a.clone(),
        )?;
        let verify_channel_state = VerifyChannelState {
            proof_height: msg.proof_height_on_b.to_string(),
            counterparty_prefix: prefix_on_b.clone().into_vec(),
            proof: msg.proof_unreceived_on_b.clone().into(),
            root: consensus_state_of_b_on_a.clone().root().clone().into_vec(),
            counterparty_chan_end_path: chan_end_path_on_b,
            expected_counterparty_channel_end: vector.unwrap(),
        };
        let data = PacketData {
            packet: msg.packet.clone(),
            signer: msg.signer.clone(),
            acknowledgement: None,
        };
        let packet_data = to_vec(&data).map_err(|e| ContractError::IbcDecodeError {
            error: e.to_string(),
        })?;

        let next_seq_recv_verification_result = if chan_end_on_a.order_matches(&Order::Ordered) {
            if msg.packet.seq_on_a < msg.next_seq_recv_on_b {
                return Err(ContractError::IbcPacketError {
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

            LightClientPacketMessage::VerifyNextSequenceRecv {
                height: msg.proof_height_on_b.to_string(),
                prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_unreceived_on_b.clone().into(),
                root: consensus_state_of_b_on_a.root().clone().into_vec(),
                seq_recv_path: seq_recv_path_on_b,
                sequence: msg.packet.seq_on_a.into(),
                packet_data,
            }
        } else {
            let receipt_path_on_b = self.packet_receipt_commitment_path(
                &msg.packet.port_id_on_b,
                &msg.packet.chan_id_on_b,
                msg.packet.seq_on_a,
            );
            LightClientPacketMessage::VerifyPacketReceiptAbsence {
                height: msg.proof_height_on_b.to_string(),
                prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_unreceived_on_b.clone().into(),
                root: consensus_state_of_b_on_a.root().clone().into_vec(),
                receipt_path: receipt_path_on_b,
                packet_data,
            }
        };
        let client_type = ClientType::from(client_state_of_b_on_a.client_type());
        let light_client_address =
            self.get_client_from_registry(deps.as_ref().storage, client_type)?;

        let light_client_message = LightClientMessage::TimeoutOnCLose {
            client_id: client_id_on_a.to_string(),
            verify_channel_state,
            next_seq_recv_verification_result,
        };

        let create_client_message: CosmosMsg = CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
            contract_addr: light_client_address,
            msg: to_binary(&light_client_message).unwrap(),
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
}
