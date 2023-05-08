use super::*;

impl<'a> CwIbcCoreContext<'a> {
    /// This function sends a packet and performs various checks to ensure the validity of the packet
    /// and its associated data.
    /// 
    /// Arguments:
    /// 
    /// * `deps`: `deps` is a `DepsMut` struct, which is a mutable reference to the dependencies of the
    /// contract. These dependencies include the storage, API, and other modules that the contract may
    /// need to interact with.
    /// * `packet`: The `packet` parameter is of type `Packet` and represents the packet to be sent over
    /// the IBC channel. It contains information such as the source and destination port and channel
    /// IDs, packet sequence number, packet data, and timeout height and timestamp.
    /// 
    /// Returns:
    /// 
    /// a `Result<Response, ContractError>` where `Response` is a struct representing the response to a
    /// message and `ContractError` is an enum representing the possible errors that can occur during
    /// the execution of the function.
    pub fn send_packet(&self, deps: DepsMut, packet: Packet) -> Result<Response, ContractError> {
        let chan_end_on_a = self.get_channel_end(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        if chan_end_on_a.state_matches(&State::Closed) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ChannelClosed {
                    channel_id: packet.chan_id_on_a,
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
                    port_id: packet.port_id_on_b,
                    channel_id: packet.chan_id_on_b,
                },
            });
        }
        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.clone().into())?;
        let client_id_on_a = conn_end_on_a.client_id();
        let client_state_of_b_on_a = self.client_state(deps.storage, client_id_on_a)?;
        if client_state_of_b_on_a.is_frozen() {
            return Err(ContractError::IbcPacketError {
                error: PacketError::FrozenClient {
                    client_id: conn_end_on_a.client_id().clone(),
                },
            });
        }
        let latest_height_on_a = client_state_of_b_on_a.latest_height();
        if packet.timeout_height_on_b.has_expired(latest_height_on_a) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::LowPacketHeight {
                    chain_height: latest_height_on_a,
                    timeout_height: packet.timeout_height_on_b,
                },
            });
        }
        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &latest_height_on_a)?;
        let latest_timestamp = consensus_state_of_b_on_a.timestamp();
        let packet_timestamp = packet.timeout_timestamp_on_b;
        if let Expiry::Expired = latest_timestamp.check_expiry(&packet_timestamp) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::LowPacketTimestamp,
            });
        }
        let next_seq_send_on_a = self.get_next_sequence_send(
            deps.storage,
            packet.port_id_on_a.clone().into(),
            packet.chan_id_on_a.clone().into(),
        )?;
        if packet.seq_on_a != next_seq_send_on_a {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketSequence {
                    given_sequence: packet.seq_on_a,
                    next_sequence: next_seq_send_on_a,
                },
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
            commitment::compute_packet_commitment(
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
