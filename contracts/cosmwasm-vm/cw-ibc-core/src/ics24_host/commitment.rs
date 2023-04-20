use ibc::{
    core::ics04_channel::{commitment::PacketCommitment, timeout::TimeoutHeight},
    timestamp::Timestamp,
};
use cw_common::commitment as cw_commitment;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn client_state_path(&self, client_id: &ClientId) -> Vec<u8> {
        cw_commitment::client_state_path(client_id)
    }
    pub fn consensus_state_path(&self, client_id: &ClientId, height: &Height) -> Vec<u8> {
       cw_commitment::consensus_state_path(client_id, height)
    }
    pub fn connection_path(&self, connection_id: &ConnectionId) -> Vec<u8> {
        cw_commitment::connection_path(connection_id)
    }

    pub fn channel_path(&self, port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
        cw_commitment::channel_path(port_id, channel_id)
    }
    pub fn packet_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        return cw_commitment::packet_commitment_path(port_id, channel_id, sequence)
            
    }
    pub fn packet_acknowledgement_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        cw_commitment::acknowledgement_commitment_path(port_id, channel_id, sequence)
    }

    pub fn packet_receipt_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        cw_commitment::receipt_commitment_path(port_id, channel_id, sequence)
    }
    pub fn next_seq_recv_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Vec<u8> {
        cw_commitment::next_seq_recv_commitment_path(port_id, channel_id)
    }

    pub fn client_state_commitment_key(&self, client_id: &ClientId) -> Vec<u8> {
        cw_commitment::client_state_commitment_key(client_id)
    }

    pub fn consensus_state_commitment_key(
        &self,
        client_id: &ClientId,
        revision_number: u64,
        revision_height: u64,
    ) -> Vec<u8> {
        cw_commitment::consensus_state_commitment_key(client_id, revision_number, revision_height)
    }

    pub fn connection_commitment_key(&self, connection_id: &ConnectionId) -> Vec<u8> {
        cw_commitment::connection_commitment_key(connection_id)
    }

    pub fn channel_commitment_key(&self, port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
        cw_commitment::channel_commitment_key(port_id, channel_id)
    }

    pub fn packet_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
       cw_commitment::packet_commitment_key(port_id, channel_id, sequence)
    }

    pub fn packet_acknowledgement_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        cw_commitment::packet_acknowledgement_commitment_key(port_id, channel_id, sequence)
    }

    pub fn packet_receipt_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        cw_commitment::packet_receipt_commitment_key(port_id, channel_id, sequence)
    }

    pub fn next_sequence_recv_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Vec<u8> {
        cw_commitment::next_sequence_recv_commitment_key(port_id, channel_id)
    }

    pub fn port_path(&self, port_id: &PortId) -> Vec<u8> {
       cw_commitment::port_path(port_id)
    }

    pub fn port_commitment_key(&self, port_id: &PortId) -> Vec<u8> {
       cw_commitment::port_commitment_key(port_id)
    }
}

pub fn sha256(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(&data).to_vec()
}

pub fn keccak256(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

pub fn compute_packet_commitment(
    packet_data: &[u8],
    timeout_height: &TimeoutHeight,
    timeout_timestamp: &Timestamp,
) -> PacketCommitment {
    cw_commitment::create_packet_commitment(
        packet_data, timeout_height.commitment_revision_number(), 
        timeout_height.commitment_revision_height(), 
        timeout_timestamp.nanoseconds()).into()
}
