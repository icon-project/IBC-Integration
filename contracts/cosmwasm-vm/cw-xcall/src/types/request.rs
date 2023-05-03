use common::rlp::Nullable;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallServiceMessageRequest {
    from: String,
    to: String,
    sequence_no: u128,
    rollback: bool,
    data: Nullable<Vec<u8>>,
}

impl CallServiceMessageRequest {
    pub fn new(from: String, to: String, sequence_no: u128, rollback: bool, data: Vec<u8>) -> Self {
        let data_bytes = match data.is_empty() {
            true => None,
            false => Some(data),
        };
        Self {
            from,
            to,
            sequence_no,
            rollback,
            data: Nullable::new(data_bytes),
        }
    }

    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn rollback(&self) -> bool {
        self.rollback
    }

    pub fn data(&self) -> &[u8] {
        //TODO: Handle error
        &self.data.get().unwrap()
    }
}

impl Encodable for CallServiceMessageRequest {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream
            .begin_list(5)
            .append(&self.from)
            .append(&self.to)
            .append(&self.sequence_no)
            .append(&self.rollback)
            .append(&self.data);
    }
}

impl Decodable for CallServiceMessageRequest {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            from: rlp.val_at(0)?,
            to: rlp.val_at(1)?,
            sequence_no: rlp.val_at(2)?,
            rollback: rlp.val_at(3)?,
            data: rlp.val_at(4)?,
        })
    }
}

impl TryFrom<&Vec<u8>> for CallServiceMessageRequest {
    type Error = ContractError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value as &[u8]);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

impl TryFrom<&[u8]> for CallServiceMessageRequest {
    type Error = ContractError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}
