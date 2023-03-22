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

use self::icon::types::v1::BtpHeader;

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
