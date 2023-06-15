use ibc_proto::google::protobuf::Any;
use ibc_proto::protobuf::Protobuf;
use prost::{DecodeError, Message};

use crate::client_state::get_default_icon_client_state;
use crate::constants::ICON_BTP_HEADER_TYPE_URL;
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
        network_type: u64,
    ) -> [u8; 32] {
        keccak256(&self.get_network_type_section_decision_rlp(src_network_id, network_type))
    }

    pub fn get_network_type_section_decision_rlp(
        &self,
        src_network_id: &str,
        network_type: u64,
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
            ..get_default_icon_client_state()
        }
    }

    pub fn to_consensus_state(&self) -> ConsensusState {
        ConsensusState {
            message_root: self.message_root.clone(),
        }
    }
}

impl From<BtpHeader> for Any {
    fn from(value: BtpHeader) -> Self {
        Any {
            type_url: ICON_BTP_HEADER_TYPE_URL.to_string(),
            value: <BtpHeader as Message>::encode_to_vec(&value),
        }
    }
}

impl Protobuf<Any> for BtpHeader {}
impl TryFrom<Any> for BtpHeader {
    type Error = DecodeError;

    fn try_from(raw: Any) -> Result<Self, Self::Error> {
        use bytes::Buf;
        use core::ops::Deref;

        fn decode_btp_header<B: Buf>(buf: B) -> Result<BtpHeader, DecodeError> {
            <BtpHeader as Message>::decode(buf)
        }

        match raw.type_url.as_str() {
            ICON_BTP_HEADER_TYPE_URL => decode_btp_header(raw.value.deref()),
            _ => Err(DecodeError::new("invalid url")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        constants::{DEFAULT_NETWORK_TYPE_ID, DEFAULT_SRC_NETWORK_ID},
        icon::icon::types::v1::SignedHeader,
    };
    use test_utils::{get_test_headers, load_test_headers};

    use super::*;
    use hex_literal::hex;

    #[test]
    fn relay_bytes_to_btp_header() {
        let relay_bytes=hex!("08a0031a20d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4280130023a2019581108325dcd15dd20fb8054ecd3eb90a010e3cba8d87d77d23f1887f14d3640014a20a483ab0eb8ab40f0a96f3acd3f8cc36941d73986b8705f4810b7be3961bdfde7");
        let header = <BtpHeader as Message>::decode(relay_bytes.as_slice());
        assert!(header.is_ok());
    }

    #[test]
    fn relay_bytes_to_signed_header() {
        let headers = load_test_headers();
        for header in headers {
            let buff = hex::decode(header.encoded_protobuf.replace("0x", "")).unwrap();
            let decoded = SignedHeader::decode(buff.as_slice());

            assert!(decoded.is_ok());
        }
    }

    #[test]
    fn test_network_section() {
        let expected=hex!("f8450102a074463d2395972061ca8807d262b0757454ed160bf43bc98d4d7a713647891a0a04a06fc96aeaecd1ed511dd7ee363638a0d76fc9d19e859f48afde692082909966b3");
        let header = &get_test_headers()[1];
        let rlp_bytes = header.get_network_section_rlp();
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("690319e26cfc39f139791fd9b0cf1015b7923ea40311444dd604f3cb46cc63b2");
        let hash = header.get_network_section_hash();
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_network_type_section() {
        let expected=hex!("f842a0d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4a0690319e26cfc39f139791fd9b0cf1015b7923ea40311444dd604f3cb46cc63b2");
        let header = &get_test_headers()[1];
        let rlp_bytes = header.get_network_type_section_rlp();
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("2b2aa1cc61539d0ef83d0e9997703e18da44a5d44824757b2b38cdbf931c33d6");
        let hash = header.get_network_type_section_hash();
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_get_network_type_section_decision() {
        let expected=hex!("f0883078332e69636f6e01830143b900a02b2aa1cc61539d0ef83d0e9997703e18da44a5d44824757b2b38cdbf931c33d6");
        let header = &get_test_headers()[1];
        let rlp_bytes = header
            .get_network_type_section_decision_rlp(DEFAULT_SRC_NETWORK_ID, DEFAULT_NETWORK_TYPE_ID);
        assert_eq!(hex::encode(expected), hex::encode(rlp_bytes));
        let expected_hash =
            hex!("8490fee35ce9f11a81c776311cfb42956ac0aa19d3c92bb832c2cef88bff4904");
        let hash = header.get_network_type_section_decision_hash(
            DEFAULT_SRC_NETWORK_ID,
            DEFAULT_NETWORK_TYPE_ID,
        );
        assert_eq!(hex::encode(expected_hash), hex::encode(hash));
    }

    #[test]
    fn test_get_network_section_hash_sequence() {
        let headers = get_test_headers();
        for (i, header) in headers.iter().enumerate() {
            if i == headers.len() - 3 {
                break;
            }
            let expected = &headers[i + 1].prev_network_section_hash;
            let current = header.get_network_section_hash();
            assert_eq!(hex::encode(expected), hex::encode(current))
        }
    }
}
