use ibc_proto::{google::protobuf::Timestamp, ibc::core::client::v1::Height};

use super::{sha256, keccak256};

pub fn get_packet_commitment(
    packet_data: &[u8],
    timeout_height: &Height,
    timeout_timestamp: u64,
) -> Vec<u8> {
    let mut hash_input = timeout_timestamp.to_be_bytes().to_vec();

    let revision_number = timeout_height.revision_number.to_be_bytes();
    hash_input.append(&mut revision_number.to_vec());

    let revision_height = timeout_height.revision_height.to_be_bytes();
    hash_input.append(&mut revision_height.to_vec());

    let packet_data_hash = packet_data;
    hash_input.append(&mut packet_data_hash.to_vec());

    sha256(&hash_input).to_vec()
}
