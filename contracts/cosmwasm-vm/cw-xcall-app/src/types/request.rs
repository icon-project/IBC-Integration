use common::rlp::Nullable;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CallServiceMessageRequest {
    from: String,
    to: String,
    sequence_no: u128,
    protocols: Vec<String>,
    rollback: bool,
    data: Nullable<Vec<u8>>,
}

impl CallServiceMessageRequest {
    // TODO : Change to Option of Bytes
    pub fn new(
        from: String,
        to: String,
        sequence_no: u128,
        protocols: Vec<String>,
        rollback: bool,
        data: Vec<u8>,
    ) -> Self {
        let data_bytes = match data.is_empty() {
            true => None,
            false => Some(data),
        };
        Self {
            from,
            to,
            sequence_no,
            rollback,
            protocols,
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

    pub fn data(&self) -> Result<&[u8], ContractError> {
        Ok(self
            .data
            .get()
            .map_err(|error| ContractError::DecodeFailed {
                error: error.to_string(),
            })?)
    }

    pub fn protocols(&self) -> &Vec<String> {
        &self.protocols
    }
}

impl Encodable for CallServiceMessageRequest {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(6);
        stream.append(&self.from);
        stream.append(&self.to);
        stream.begin_list(self.protocols.len());
        for protocol in self.protocols.iter() {
            stream.append(protocol);
        }

        stream.append(&self.sequence_no);
        stream.append(&self.rollback);
        stream.append(&self.data);
    }
}

impl Decodable for CallServiceMessageRequest {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let rlp_protocols = rlp.at(2)?;
        let list: Vec<String> = rlp_protocols.as_list()?;
        Ok(Self {
            from: rlp.val_at(0)?,
            to: rlp.val_at(1)?,
            protocols: list,
            sequence_no: rlp.val_at(3)?,
            rollback: rlp.val_at(4)?,
            data: rlp.val_at(5)?,
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
