use super::*;

fn hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha2::Digest;

    sha2::Sha256::digest(&data).to_vec()
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn client_state_path(&self, client_id: &ClientId) -> Vec<u8> {
        ClientStatePath::new(client_id).to_string().into_bytes()
    }
    pub fn consensus_state_path(&self, client_id: &ClientId, height: &Height) -> Vec<u8> {
        ClientConsensusStatePath::new(client_id, height)
            .to_string()
            .into_bytes()
    }
    pub fn connection_path(&self, connection_id: &ConnectionId) -> Vec<u8> {
        ConnectionPath::new(connection_id).to_string().into_bytes()
    }

    pub fn channel_path(&self, port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
        ChannelEndPath::new(port_id, channel_id)
            .to_string()
            .into_bytes()
    }
    pub fn packet_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        CommitmentPath::new(port_id, channel_id, sequence)
            .to_string()
            .into_bytes()
    }
    pub fn packet_acknowledgement_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        AckPath::new(port_id, channel_id, sequence)
            .to_string()
            .into_bytes()
    }

    pub fn packet_receipt_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        ReceiptPath::new(port_id, channel_id, sequence)
            .to_string()
            .into_bytes()
    }
    pub fn next_seq_recv_commitment_path(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Vec<u8> {
        SeqRecvPath::new(port_id, channel_id)
            .to_string()
            .into_bytes()
    }

    pub fn client_state_commitment_key(&self, client_id: &ClientId) -> Vec<u8> {
        hash(&self.client_state_path(client_id))
    }

    pub fn consensus_state_commitment_key(
        &self,
        client_id: &ClientId,
        revision_number: u64,
        revision_height: u64,
    ) -> Vec<u8> {
        let height = Height::new(revision_number, revision_height).unwrap();
        hash(&self.consensus_state_path(client_id, &height))
    }

    pub fn connection_commitment_key(&self, connection_id: &ConnectionId) -> Vec<u8> {
        hash(&self.connection_path(connection_id))
    }

    pub fn channel_commitment_key(&self, port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
        hash(&self.channel_path(port_id, channel_id))
    }

    pub fn packet_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        hash(&self.packet_commitment_path(port_id, channel_id, sequence))
    }

    pub fn packet_acknowledgement_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        hash(&self.packet_acknowledgement_commitment_path(port_id, channel_id, sequence))
    }

    pub fn packet_receipt_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Vec<u8> {
        hash(&self.packet_receipt_commitment_path(port_id, channel_id, sequence))
    }

    pub fn next_sequence_recv_commitment_key(
        &self,
        port_id: &PortId,
        channel_id: &ChannelId,
    ) -> Vec<u8> {
        hash(&self.next_seq_recv_commitment_path(port_id, channel_id))
    }
}
