use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn send_packet(&self, deps: DepsMut, packet: Packet) -> Result<Response, ContractError> {
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        if chan_end_on_a.state_matches(&State::Closed) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::ChannelClosed {
                    channel_id: packet.chan_id_on_a,
                }
                .to_string(),
            });
        }
        let counterparty = Counterparty::new(
            packet.port_id_on_b.clone(),
            Some(packet.chan_id_on_b.clone()),
        );
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: packet.port_id_on_b,
                    channel_id: packet.chan_id_on_b,
                }
                .to_string(),
            });
        }
        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.clone().into())?;
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;
        if client_state_of_b_on_a.is_frozen() {
            return Err(ContractError::IbcPackketError {
                error: PacketError::FrozenClient {
                    client_id: conn_end_on_a.client_id().clone(),
                }
                .to_string(),
            });
        }
        let latest_height_on_a = client_state_of_b_on_a.latest_height();
        if packet.timeout_height_on_b.has_expired(latest_height_on_a) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::LowPacketHeight {
                    chain_height: latest_height_on_a,
                    timeout_height: packet.timeout_height_on_b,
                }
                .to_string(),
            });
        }
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &latest_height_on_a)?;
        let latest_timestamp = consensus_state_of_b_on_a.timestamp();
        let packet_timestamp = packet.timeout_timestamp_on_b;
        if let Expiry::Expired = latest_timestamp.check_expiry(&packet_timestamp) {
            return Err(ContractError::IbcPackketError {
                error: PacketError::LowPacketTimestamp.to_string(),
            });
        }
        let next_seq_send_on_a = self.get_next_sequence_send(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        if packet.seq_on_a != next_seq_send_on_a {
            return Err(ContractError::IbcPackketError {
                error: PacketError::InvalidPacketSequence {
                    given_sequence: packet.seq_on_a,
                    next_sequence: next_seq_send_on_a,
                }
                .to_string(),
            });
        }
        self.increase_next_sequence_send(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        self.store_packet_commitment(
            deps.storage,
            &packet.port_id_on_a.clone().into(),
            &packet.chan_id_on_a.clone().into(),
            packet.seq_on_a,
            compute_packet_commitment(
                &packet.data,
                &packet.timeout_height_on_b,
                &packet.timeout_timestamp_on_b,
            ),
        )?;
        let event = create_send_packet_event(packet, chan_end_on_a.ordering(), conn_id_on_a)?;
        Ok(Response::new()
            .add_attribute("action", "send_packet")
            .add_event(event))
    }
}
