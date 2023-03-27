// @generated
pub mod icon {
    pub mod lightclient {
        // @@protoc_insertion_point(attribute:icon.lightclient.v1)
        pub mod v1 {
            include!("icon.lightclient.v1.rs");
            // @@protoc_insertion_point(icon.lightclient.v1)
        }
    }
    pub mod types {
        // @@protoc_insertion_point(attribute:icon.types.v1)
        pub mod v1 {
            include!("icon.types.v1.rs");
            // @@protoc_insertion_point(icon.types.v1)
        }
    }
}

use bytes::BytesMut;

use crate::{
    rlp,
    utils::{calculate_root, keccak256},
};

use self::icon::{lightclient::v1::ClientState, types::v1::BtpHeader};

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
        // let relay_bytes=hex!("7b224074797065223a222f69636f6e2e74797065732e76312e425450486561646572222c226d61696e5f686569676874223a22343136222c22726f756e64223a302c226e6578745f70726f6f665f636f6e746578745f68617368223a22304a4177516d5475376a7731596855764c634e565942734c516a71556943543943674573456350384c37513d222c226e6574776f726b5f73656374696f6e5f746f5f726f6f74223a5b5d2c226e6574776f726b5f6964223a2231222c227570646174655f6e756d626572223a2232222c22707265765f6e6574776f726b5f73656374696f6e5f68617368223a224756675243444a647a52586449507541564f7a5436354367454f504c714e68396439492f474966785454593d222c226d6573736167655f636f756e74223a2231222c226d6573736167655f726f6f74223a2270494f724472697251504370627a724e50347a44615548584f59613463463949454c652b4f5747392f65633d222c226e65787456616c696461746f7273223a5b5d7d");

        // let header: Any = Any::decode(relay_bytes.as_slice()).unwrap();
        // assert_eq!(header.type_url, "hhh");

        // let test_any = Any {
        //     type_url: "someurl".to_string(),
        //     value: "some value".as_bytes().to_vec(),
        // };
        // let hex_bytes = hex::encode(test_any.encode_to_vec());
        // assert_eq!(hex_bytes, "hhh".to_string())

        let btpheader =
            hex!("080a10011a030a141e2206080112020a14280130143a030a141e400a4a0314141e52030a141e");
        let mut header = BtpHeader::decode(btpheader.as_slice()).unwrap();
        println!("btpheader {:?}", header);
        header.main_height = 11;
        header.next_validators = vec![];
        println!("new header {:?}", hex::encode(&header.encode_to_vec()))
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
