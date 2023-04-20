use std::str::FromStr;

use common::utils::{keccak256, sha256};
use ibc::core::ics02_client::height::Height;
use ibc::core::ics04_channel::packet::Sequence;
use ibc::core::ics24_host::identifier::{ChannelId, PortId, ClientId, ConnectionId};
use ibc::core::ics24_host::path::{self, CommitmentPath, ClientStatePath, ClientConsensusStatePath, ConnectionPath, ChannelEndPath, AckPath, ReceiptPath, SeqRecvPath, PortPath};
use ibc_proto::ibc::core::channel::v1::Packet;
use ibc_proto::{google::protobuf::Timestamp};

pub trait ICommitment {
    fn commitment_path(&self) -> Vec<u8>;
    fn commitment(&self) -> Vec<u8>;

    fn commitment_key(&self) -> Vec<u8> {
        return commitment_path_hash(&self.commitment_path()).into();
    }
    
}
pub fn commitment_path_hash(path_bytes: &[u8]) -> Vec<u8> {
    return keccak256(path_bytes).into();
}

pub fn client_state_path(client_id:&ClientId) -> Vec<u8> {
    ClientStatePath::new(client_id).to_string().into_bytes()
}
pub fn consensus_state_path(client_id: &ClientId, height: &Height) -> Vec<u8> {
    ClientConsensusStatePath::new(client_id, height)
        .to_string()
        .into_bytes()
}
pub fn connection_path(connection_id: &ConnectionId) -> Vec<u8> {
    ConnectionPath::new(connection_id).to_string().into_bytes()
}

pub fn channel_path(port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
    ChannelEndPath::new(port_id, channel_id)
        .to_string()
        .into_bytes()
}

pub fn acknowledgement_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    AckPath::new(port_id, channel_id, sequence)
        .to_string()
        .into_bytes()
}

pub fn receipt_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    ReceiptPath::new(port_id, channel_id, sequence)
        .to_string()
        .into_bytes()
}
pub fn next_seq_recv_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Vec<u8> {
    SeqRecvPath::new(port_id, channel_id)
        .to_string()
        .into_bytes()
}





pub fn packet_commitment_path(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    CommitmentPath::new(&port_id, &channel_id, sequence)
        .to_string()
        .into_bytes()
}

pub fn create_packet_commitment(
    packet_data: &[u8],
    revision_number: u64,
    revision_height: u64,
    timeout_timestamp: u64,
) -> Vec<u8> {
    let mut hash_input = timeout_timestamp.to_be_bytes().to_vec();

    let revision_number = revision_number.to_be_bytes();
    hash_input.append(&mut revision_number.to_vec());

    let revision_height = revision_height.to_be_bytes();
    hash_input.append(&mut revision_height.to_vec());

    let packet_data_hash = packet_data;
    hash_input.append(&mut packet_data_hash.to_vec());

    sha256(&hash_input).to_vec()
}

pub fn client_state_commitment_key(client_id: &ClientId) -> Vec<u8> {
    commitment_path_hash(&client_state_path(client_id))
}

pub fn consensus_state_commitment_key(
    client_id: &ClientId,
    revision_number: u64,
    revision_height: u64,
) -> Vec<u8> {
    let height = Height::new(revision_number, revision_height).unwrap();
    commitment_path_hash(&consensus_state_path(client_id, &height))
}

pub fn connection_commitment_key(connection_id: &ConnectionId) -> Vec<u8> {
    commitment_path_hash(&connection_path(connection_id))
}

pub fn channel_commitment_key(port_id: &PortId, channel_id: &ChannelId) -> Vec<u8> {
    commitment_path_hash(&channel_path(port_id, channel_id))
}

pub fn packet_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    commitment_path_hash(&packet_commitment_path(port_id, channel_id, sequence))
}

pub fn packet_acknowledgement_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    commitment_path_hash(&acknowledgement_commitment_path(port_id, channel_id, sequence))
}

pub fn packet_receipt_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
    sequence: Sequence,
) -> Vec<u8> {
    commitment_path_hash(&receipt_commitment_path(port_id, channel_id, sequence))
}

pub fn next_sequence_recv_commitment_key(
    port_id: &PortId,
    channel_id: &ChannelId,
) -> Vec<u8> {
    commitment_path_hash(&next_seq_recv_commitment_path(port_id, channel_id))
}

pub fn port_path(port_id: &PortId) -> Vec<u8> {
    PortPath(port_id.clone()).to_string().into_bytes()
}

pub fn port_commitment_key(port_id: &PortId) -> Vec<u8> {
    commitment_path_hash(&port_path(port_id))
}


impl ICommitment for Packet {
    fn commitment_path(&self) -> Vec<u8> {
        let port_id = PortId::from_str(&self.source_port).unwrap();
        let channel_id = ChannelId::from_str(&self.source_channel).unwrap();
        let sequence = Sequence::from_str(&self.sequence.to_string()).unwrap();
        return packet_commitment_path(&port_id, &channel_id, sequence);
    }

    fn commitment(&self) -> Vec<u8> {
        let packet_data = self.data.clone();
        let revision_number = self.timeout_height.clone().and_then(|h|Some(h.revision_number)).unwrap_or(0);
        let revision_height = self.timeout_height.clone().and_then(|h|Some(h.revision_height)).unwrap_or(0);
        return create_packet_commitment(
            &packet_data,
            revision_number,
            revision_height,
            self.timeout_timestamp,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::icon::icon::types::v1::MerkleNode;
    use common::utils::calculate_root;
    use common::utils::keccak256;
    use common::utils::sha256;
    use prost::Message;
    use test_utils::load_test_messages;
    // 00000000000000000000000000000000000000000000000074657374
    #[test]
    fn test_packet_message_data() {
        let data = load_test_messages();
        for (i, msg) in data.iter().enumerate() {
            if i == 0 {
                continue;
            }
            let msg_path = hex::decode(&msg.commitment_path).unwrap();
            let expected_key = keccak256(&msg_path);
            let msg_key = hex::decode(&msg.commitment_key).unwrap();
            assert_eq!(hex::encode(&expected_key), hex::encode(&msg_key));

            let packet =
                Packet::decode(hex::decode(&msg.packet_encoded).unwrap().as_slice()).unwrap();
            let calc_path = packet.commitment_path();
            assert_eq!(hex::encode(msg_path), hex::encode(calc_path));

            let message_bytes = hex::decode(&msg.messages[0]).unwrap();
            let packet_bytes = packet.encode_to_vec();
            assert_eq!(msg.packet_encoded, hex::encode(&packet_bytes));

            let packet_commitment_hash = packet.commitment();

            assert_eq!(
                hex::encode(&message_bytes[32..]),
                hex::encode(&packet_commitment_hash)
            );

            let leaf = keccak256(
                [msg_key, keccak256(&message_bytes).into()]
                    .concat()
                    .as_slice(),
            );
            let proof = msg
                .proof
                .iter()
                .map(|tn| {
                    let node: MerkleNode = tn.try_into().unwrap();
                    node
                })
                .collect::<Vec<MerkleNode>>();
            let root = calculate_root(leaf, &proof);
            assert_eq!("", hex::encode(root));
        }
    }

    #[test]
    fn test_sha256() {
        let bytes = b"Hello World";
        let result = sha256(bytes);
        assert_eq!(
            "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e",
            hex::encode(result)
        );
    }
}
