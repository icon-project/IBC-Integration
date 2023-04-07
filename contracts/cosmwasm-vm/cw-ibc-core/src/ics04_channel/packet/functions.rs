use super::*;

pub fn compute_packet_commitment(
    packet_data: &[u8],
    timeout_height: &TimeoutHeight,
    timeout_timestamp: &Timestamp,
) -> PacketCommitment {
    let mut hash_input = timeout_timestamp.nanoseconds().to_be_bytes().to_vec();

    let revision_number = timeout_height.commitment_revision_number().to_be_bytes();
    hash_input.append(&mut revision_number.to_vec());

    let revision_height = timeout_height.commitment_revision_height().to_be_bytes();
    hash_input.append(&mut revision_height.to_vec());

    let packet_data_hash = hash(packet_data);
    hash_input.append(&mut packet_data_hash.to_vec());

    hash(&hash_input).into()
}

fn hash(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha2::Digest;

    sha2::Sha256::digest(&data).to_vec()
}
