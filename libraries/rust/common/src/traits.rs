use ibc_proto::google::protobuf::Any;
use prost::{DecodeError, Message};

use crate::utils::keccak256;

pub trait AnyTypes: Message + Default {
    fn get_type_url() -> String;

    fn get_type_url_hash() -> [u8; 32] {
        keccak256(Self::get_type_url().as_bytes())
    }

    fn from_any(any: Any) -> Result<Self, DecodeError> {
        if Self::get_type_url_hash() != keccak256(any.type_url.as_bytes()) {
            return Err(DecodeError::new("Invalid typ"));
        }
        Self::decode(any.value.as_slice())
    }

    fn to_any(&self) -> Any {
        return Any {
            type_url: Self::get_type_url(),
            value: self.encode_to_vec(),
        };
    }

    fn any_from_value(value: &[u8]) -> Any {
        return Any {
            type_url: Self::get_type_url(),
            value: value.to_vec(),
        };
    }

    fn get_keccak_hash(&self) -> [u8; 32] {
        let bytes = self.encode_to_vec();
        return keccak256(&bytes);
    }
    fn get_keccak_hash_string(&self) -> String {
        let hash = self.get_keccak_hash();
        return hex::encode(hash);
    }
}
