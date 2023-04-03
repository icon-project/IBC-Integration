use crate::icon::icon::lightclient::v1::{ClientState, ConsensusState};
use crate::rlp::RlpStream;
use crate::{
    icon::icon::types::v1::BtpHeader,
    utils::{calculate_root, keccak256},
};

impl BtpHeader {
    pub fn get_network_type_section_decision_hash(
        &self,
        src_network_id: &str,
        network_type: u128,
    ) -> [u8; 32] {
        keccak256(&self.get_network_type_section_decision_rlp(src_network_id, network_type))
    }

    pub fn get_network_type_section_decision_rlp(
        &self,
        src_network_id: &str,
        network_type: u128,
    ) -> Vec<u8> {
        let mut ntsd = RlpStream::new_list(5);

        ntsd.append(&src_network_id);
        ntsd.append(&network_type);
        ntsd.append(&self.main_height);
        ntsd.append(&self.round);
        ntsd.append(&self.get_network_type_section_hash().as_slice());

        let encoded = ntsd.as_raw().to_vec();
        encoded
    }

    pub fn get_network_section_rlp(&self) -> Vec<u8> {
        let mut ns = RlpStream::new_list(5);

        ns.append(&Into::<u128>::into(self.network_id));
        ns.append(&self.update_number);
        ns.append(&self.prev_network_section_hash);
        ns.append(&self.message_count);
        ns.append(&self.message_root);

        let encoded = ns.as_raw().to_vec();
        encoded
    }

    pub fn get_network_section_hash(&self) -> [u8; 32] {
        keccak256(&self.get_network_section_rlp())
    }

    pub fn get_network_type_section_hash(&self) -> [u8; 32] {
        keccak256(&self.get_network_type_section_rlp())
    }

    pub fn get_network_type_section_rlp(&self) -> Vec<u8> {
        let mut nts = RlpStream::new_list(2);
        nts.append(&self.next_proof_context_hash);
        nts.append(&self.get_network_section_root().as_slice());

        let encoded = nts.as_raw().to_vec();
        encoded
    }

    pub fn get_network_section_root(&self) -> [u8; 32] {
        calculate_root(
            self.get_network_section_hash(),
            &self.network_section_to_root,
        )
    }

    pub fn to_client_state(&self, trusting_period: u64, max_clock_drift: u64) -> ClientState {
        ClientState {
            trusting_period,
            frozen_height: 0,
            max_clock_drift,
            latest_height: self.main_height,
            network_section_hash: self.get_network_section_hash().to_vec(),
            validators: self.next_validators.clone(),
        }
    }

    pub fn to_consensus_state(&self) -> ConsensusState {
        ConsensusState {
            message_root: self.message_root.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::get_test_headers;

    use super::*;
    use hex_literal::hex;
    use prost::Message;

    #[test]
    fn relay_bytes_to_btp_header() {
        let relay_bytes=hex!("08a0031a20d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4280130023a2019581108325dcd15dd20fb8054ecd3eb90a010e3cba8d87d77d23f1887f14d3640014a20a483ab0eb8ab40f0a96f3acd3f8cc36941d73986b8705f4810b7be3961bdfde7");
        let header = BtpHeader::decode(relay_bytes.as_slice());
        assert!(header.is_ok());
    }

    #[test]
    fn test_network_section() {
        let expected=hex!("f8450102a0d3cd05ddbec4846c124bff569ab5531f6284a521f64c80b82110aa016937ed7601a084d8e19eb09626e4a94212d3a9db54bc16a75dfd791858c0fab3032b944f657a");
        let header = &get_test_headers()[1];
        let rlp_bytes = header.get_network_section_rlp();
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("688587a1efabcb22f532a8bf0cb541a1e487365b0ac77ce166c211296900f68d");
        let hash = header.get_network_section_hash();
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_network_type_section() {
        let expected=hex!("f842a0d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4a0688587a1efabcb22f532a8bf0cb541a1e487365b0ac77ce166c211296900f68d");
        let header = &get_test_headers()[1];
        let rlp_bytes = header.get_network_type_section_rlp();
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("d6be1d816f18e5e134f5bfe5de755d5aba7af6988a67c1e0fe9bf1a7965dea94");
        let hash = header.get_network_type_section_hash();
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_get_network_type_section_decision() {
        let expected=hex!("ef883078332e69636f6e0182507200a0d6be1d816f18e5e134f5bfe5de755d5aba7af6988a67c1e0fe9bf1a7965dea94");
        let header = &get_test_headers()[1];
        let rlp_bytes = header.get_network_type_section_decision_rlp("0x3.icon", 1);
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("d3441d4cc7b6472976a357cd81f77a54c25d914b3adf99721309bc705fa116c2");
        let hash = header.get_network_type_section_decision_hash("0x3.icon", 1);
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_get_network_section_hash_sequence() {
        let headers = get_test_headers();
        for (i, header) in headers.iter().enumerate() {
            if i == headers.len() - 1 {
                break;
            }
            let expected = &headers[i + 1].prev_network_section_hash;
            let current = header.get_network_section_hash();
            assert_eq!(hex::encode(expected), hex::encode(current))
        }
    }
}
