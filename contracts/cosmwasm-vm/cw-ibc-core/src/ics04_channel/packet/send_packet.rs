use common::ibc::core::ics04_channel::timeout::TimeoutHeight;
use cw_common::{
    ibc_types::IbcTimestamp,
    raw_types::{channel::RawPacket, RawHeight},
};
use debug_print::debug_println;

use crate::conversions::{to_ibc_port, to_ibc_channel, to_ibc_timestamp, to_ibc_timeout_height};

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
    pub fn send_packet(&self, deps: DepsMut, packet: RawPacket) -> Result<Response, ContractError> {
        let src_port = to_ibc_port(&packet.source_port)?;
        let src_channel = to_ibc_channel(&packet.source_channel)?;

        let dst_port = to_ibc_port(&packet.destination_port)?;
        let dst_channel = to_ibc_channel(&packet.destination_channel)?;

        let chan_end_on_a = self.get_channel_end(deps.storage, &src_port, &src_channel)?;
        debug_println!("fetched channel_end");
        if chan_end_on_a.state_matches(&State::Closed) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::ChannelClosed {
                    channel_id: src_channel.clone(),
                },
            });
        }
        debug_println!(" channel_end matched");

        let counterparty = Counterparty::new(dst_port.clone(), Some(dst_channel.clone()));
        if !chan_end_on_a.counterparty_matches(&counterparty) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketCounterparty {
                    port_id: dst_port.clone(),
                    channel_id: dst_channel.clone(),
                },
            });
        }
        debug_println!(" counterparty_matched");

        let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
        let conn_end_on_a = self.connection_end(deps.storage, conn_id_on_a.clone())?;
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
        let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone())?;

        if packet_timeout_height.has_expired(latest_height_on_a) {
            return Err(ContractError::IbcPacketError {
                error: PacketError::LowPacketHeight {
                    chain_height: latest_height_on_a,
                    timeout_height: packet_timeout_height.clone(),
                },
            });
        }

        debug_println!(" check pass: packet exipred");

        let consensus_state_of_b_on_a =
            self.consensus_state(deps.storage, client_id_on_a, &latest_height_on_a)?;
        let latest_timestamp = consensus_state_of_b_on_a.timestamp();
        let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp)?;
        if let Expiry::Expired = latest_timestamp.check_expiry(&packet_timestamp) {
            return Err(PacketError::LowPacketTimestamp).map_err(Into::<ContractError>::into);
        }
        debug_println!(" timestamp check pass");

        let next_seq_send_on_a =
            self.get_next_sequence_send(deps.storage, src_port.clone(), src_channel.clone())?;
        debug_println!(" fetched next seq send {:?}", next_seq_send_on_a);

        if Sequence::from(packet.sequence) != next_seq_send_on_a {
            return Err(ContractError::IbcPacketError {
                error: PacketError::InvalidPacketSequence {
                    given_sequence: packet.sequence.into(),
                    next_sequence: next_seq_send_on_a,
                },
            });
        }

        debug_println!(" packet seq and next seq matched");

        self.increase_next_sequence_send(deps.storage, src_port.clone(), src_channel.clone())?;
        self.store_packet_commitment(
            deps.storage,
            &src_port.clone(),
            &src_channel.clone(),
            packet.sequence.into(),
            commitment::compute_packet_commitment(
                &packet.data,
                &packet_timeout_height,
                &packet_timestamp,
            ),
        )?;
        debug_println!(" packet commitment stored");

        let event = create_packet_event(
            IbcEventType::SendPacket,
            RawPacket::from(packet),
            chan_end_on_a.ordering(),
            conn_id_on_a,
            None,
        )?;
        Ok(Response::new()
            .add_attribute("action", "send_packet")
            .add_event(event))
    }
}
