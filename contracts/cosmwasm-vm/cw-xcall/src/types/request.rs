use common::rlp::Encodable;
use cosmwasm_std::Binary;

use super::*;

#[cw_serde]
pub struct CallServiceMessageRequest {
    from: Address,
    to: String,
    sequence_no: u128,
    rollback: Vec<u8>,
    data: Vec<u8>,
}

impl CallServiceMessageRequest {
    pub fn new(
        from: Address,
        to: String,
        sequence_no: u128,
        rollback: Vec<u8>,
        data: Vec<u8>,
    ) -> Self {
        Self {
            from,
            to,
            sequence_no,
            rollback,
            data,
        }
    }

    pub fn from(&self) -> &Address {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn rollback(&self) -> &[u8] {
        &self.rollback
    }

    pub fn data(&self) -> &[u8] {
        &self.data
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
