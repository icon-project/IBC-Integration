use bytes::BytesMut;

use crate::{
    icon::icon::types::v1::BtpHeader,
    rlp,
    utils::{calculate_root, keccak256},
};

impl BtpHeader {
    pub fn get_network_type_section_decision_hash(
        &self,
        src_network_id: &str,
        network_type: u128,
    ) -> [u8; 32] {
        let mut ntsd = Vec::with_capacity(5);

        ntsd.push(rlp::encode(&src_network_id));
        ntsd.push(rlp::encode(&network_type));
        ntsd.push(rlp::encode(&self.main_height));
        ntsd.push(rlp::encode(&self.round));
        ntsd.push(rlp::encode(
            &self.get_network_type_section_hash().as_slice(),
        ));
        let encoded = rlp::encode_list::<BytesMut, BytesMut>(&ntsd);
        keccak256(&encoded)
    }

    pub fn get_network_section_hash(&self) -> [u8; 32] {
        let mut ns = Vec::with_capacity(5);

        ns.push(rlp::encode(&self.network_id));

        ns.push(rlp::encode(&self.round));

        ns.push(rlp::encode(&self.prev_network_section_hash));

        ns.push(rlp::encode(&self.message_count));
        ns.push(rlp::encode(&self.message_root));
        let encoded = rlp::encode_list::<BytesMut, BytesMut>(&ns);

        keccak256(&encoded)
    }

    pub fn get_network_type_section_hash(&self) -> [u8; 32] {
        let mut nts = Vec::with_capacity(2);
        nts.push(rlp::encode(&self.next_proof_context_hash));
        nts.push(rlp::encode(&self.get_network_section_root().as_slice()));
        let encoded = rlp::encode_list::<BytesMut, BytesMut>(&nts);
        keccak256(&rlp::encode_list(&encoded))
    }

    pub fn get_network_section_root(&self) -> [u8; 32] {
        calculate_root(
            self.get_network_section_hash(),
            &self.network_section_to_root,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use ibc_proto::google::protobuf::Any;
    use prost::Message;

    #[test]
    fn relay_bytes_to_btp_header() {
        let relay_bytes=hex!("08a0031a20d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4280130023a2019581108325dcd15dd20fb8054ecd3eb90a010e3cba8d87d77d23f1887f14d3640014a20a483ab0eb8ab40f0a96f3acd3f8cc36941d73986b8705f4810b7be3961bdfde7");
        let header = BtpHeader::decode(relay_bytes.as_slice());
        assert!(header.is_ok());
    }

    #[test]
    fn test_get_network_type_section_decision_hash() {
        let header = BtpHeader {
            network_id: 1,
            main_height: 1234,
            round: 5678,
            prev_network_section_hash: [0; 32].to_vec(),
            message_count: 5,
            message_root: [0; 32].to_vec(),
            next_proof_context_hash: [0; 32].to_vec(),
            network_section_to_root: vec![],
            update_number: todo!(),
            next_validators: todo!(),
        };
        let src_network_id = "foo";
        let network_type = 12345;
        let expected_hash = [
            190, 96, 183, 170, 36, 171, 128, 60, 17, 34, 137, 16, 54, 74, 150, 155, 209, 33, 112,
            107, 201, 200, 55, 105, 94, 121, 105, 197, 3, 187, 130, 233,
        ];
        let hash = header.get_network_type_section_decision_hash(src_network_id, network_type);
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_get_network_section_hash() {
        let header = BtpHeader {
            network_id: 1,
            main_height: 1234,
            round: 5678,
            prev_network_section_hash: [0; 32].to_vec(),
            message_count: 5,
            message_root: [0; 32].to_vec(),
            next_proof_context_hash: [0; 32].to_vec(),
            network_section_to_root: vec![],
            update_number: todo!(),
            next_validators: todo!(),
        };
        let expected_hash = [
            192, 78, 178, 69, 183, 178, 167, 32, 62, 191, 88, 100, 214, 216, 23, 132, 114, 215,
            179, 74, 31, 190, 110, 193, 190, 22, 102, 160, 114, 24, 235, 226,
        ];
        let hash = header.get_network_section_hash();
        assert_eq!(hash, expected_hash);
    }

    #[test]
    fn test_get_network_type_section_hash() {
        let header = BtpHeader {
            network_id: 1,
            main_height: 1234,
            round: 5678,
            prev_network_section_hash: [0; 32].to_vec(),
            message_count: 5,
            message_root: [0; 32].to_vec(),
            next_proof_context_hash: [0; 32].to_vec(),
            network_section_to_root: vec![],
            update_number: todo!(),
            next_validators: todo!(),
        };
        let expected_hash = [
            165, 193, 202, 176, 167, 227, 192, 73, 161, 85, 182, 153, 150, 221, 59, 174, 109, 231,
            33, 238, 186, 88, 180, 143, 71, 187, 153, 148, 110, 40, 85, 6,
        ];
        let hash = header.get_network_type_section_hash();
        assert_eq!(hash, expected_hash);
    }
}
