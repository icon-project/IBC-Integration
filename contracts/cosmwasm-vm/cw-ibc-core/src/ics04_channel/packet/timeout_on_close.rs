use cw_common::raw_types::channel::RawMessageTimeoutOnclose;
use debug_print::debug_println;

use crate::conversions::{
    to_ibc_channel_id, to_ibc_height, to_ibc_port_id, to_ibc_timeout_height, to_ibc_timestamp,
};

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function validates a timeout on close packet and sends a submessage to a light client
    /// for further verification.
    ///
    /// Arguments:
    ///
    /// * `deps`: `deps` is a `DepsMut` struct, which provides mutable access to the contract's
    /// dependencies such as storage, API, and querier. It is used to interact with the blockchain and
    /// other contracts.
    /// * `info`: `info` is a struct of type `MessageInfo` which contains information about the message
    /// being processed, such as the sender's address and the amount of funds sent with the message.
    /// * `msg`: `msg` is a `MsgTimeoutOnClose` struct which contains information about a timeout on a
    /// packet that was sent on a channel that is being closed. It includes the packet itself, the
    /// signer of the message, and proof information related to the packet and the channel.
    ///
    /// Returns:
    ///
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// contract execution.
    pub fn timeout_on_close_packet_validate_to_light_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        msg: RawMessageTimeoutOnclose,
    ) -> Result<Response, ContractError> {
        let packet = &msg.packet.clone().unwrap();
        let src_port = to_ibc_port_id(&packet.source_port)?;
        let src_channel = to_ibc_channel_id(&packet.source_channel)?;

        let dst_port = to_ibc_port_id(&packet.destination_port)?;
        let dst_channel = to_ibc_channel_id(&packet.destination_channel)?;

        let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;
        let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;

        let packet_sequence = Sequence::from(packet.sequence);
        let proof_height = to_ibc_height(msg.proof_height.clone().unwrap())?;

        let next_sequence_recv = Sequence::from(msg.next_sequence_recv);

        let chan_end_on_a = self.get_channel_end(deps.storage, &src_port, &src_channel)?;
        let counterparty = Counterparty::new(dst_port.clone(), Some(dst_channel.clone()));
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: dst_port.clone(),
                    channel_id: dst_channel,
                },
            });
        }
        let conn_id_on_a = chan_end_on_a.connection_hops()[0].clone();
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a)?;
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
            &packet_timestamp,
        );
        if commitment_on_a != expected_commitment_on_a {
            return Err(ContractError::IbcPacketError {
                error: PacketError::IncorrectPacketCommitment {
                    sequence: packet_sequence,
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
            self.consensus_state(deps.storage, client_id_on_a, &proof_height)?;
        let prefix_on_b = conn_end_on_a.counterparty().prefix();
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
        let expected_counterparty = Counterparty::new(src_port.clone(), Some(src_channel.clone()));
        let expected_chan_end_on_b = ChannelEnd::new(
            State::Closed,
            *chan_end_on_a.ordering(),
            expected_counterparty,
            expected_conn_hops_on_b,
            chan_end_on_a.version().clone(),
        );
        let chan_end_path_on_b = commitment::channel_path(&dst_port, &dst_channel);
        let vector = to_vec(&expected_chan_end_on_b);

        self.verify_connection_delay_passed(
            deps.storage,
            env,
            proof_height,
            conn_end_on_a.clone(),
        )?;
        let verify_channel_state = VerifyChannelState {
            proof_height: proof_height.to_string(),
            counterparty_prefix: prefix_on_b.clone().into_vec(),
            proof: msg.proof_close.clone().into(),
            root: consensus_state_of_b_on_a.clone().root().into_vec(),
            counterparty_chan_end_path: chan_end_path_on_b,
            expected_counterparty_channel_end: vector.unwrap(),
            client_id: conn_end_on_a.client_id().to_string(),
        };
        let next_seq_recv_verification_result = if chan_end_on_a.order_matches(&Order::Ordered) {
            if packet_sequence < next_sequence_recv {
                return Err(ContractError::IbcPacketError {
                    error: PacketError::InvalidPacketSequence {
                        given_sequence: packet_sequence,
                        next_sequence: next_sequence_recv,
                    },
                });
            }
            let seq_recv_path_on_b =
                commitment::next_seq_recv_commitment_path(&dst_port.clone(), &dst_channel.clone());

            LightClientPacketMessage::VerifyNextSequenceRecv {
                height: proof_height.to_string(),
                prefix: conn_end_on_a.counterparty().prefix().clone().into_vec(),
                proof: msg.proof_unreceived.clone().into(),
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
                proof: msg.proof_unreceived.clone().into(),
                root: consensus_state_of_b_on_a.root().into_vec(),
                receipt_path: receipt_path_on_b,
            }
        };

        let client = self.get_client(deps.as_ref().storage, client_id_on_a.clone())?;
        client.verify_timeout_on_close(
            deps.as_ref(),
            client_id_on_a,
            verify_channel_state,
            next_seq_recv_verification_result,
        )?;
        debug_println!("Light Client Validation Passed");

        // let port_id = packet.src_port.clone();
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
        let timeoutblock = match packet_timeout_height {
            common::ibc::core::ics04_channel::timeout::TimeoutHeight::Never => CwTimeoutBlock {
                revision: 1,
                height: 1,
            },
            common::ibc::core::ics04_channel::timeout::TimeoutHeight::At(x) => CwTimeoutBlock {
                revision: x.revision_number(),
                height: x.revision_height(),
            },
        };
        let timeout = CwTimeout::with_block(timeoutblock);
        let ibc_packet = CwPacket::new(data, src, dest, packet.sequence.into(), timeout);
        self.store_callback_data(
            deps.storage,
            VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
            &ibc_packet,
        )?;

        let address = Addr::unchecked(msg.signer.to_string());
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
}
