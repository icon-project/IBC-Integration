use cosmwasm_std::Binary;

use super::*;

#[cw_serde]
pub struct CallServiceMessageRequest {
    from: Address,
    to: String,
    sequence_no: u128,
    rollback: Binary,
    data: Binary,
}

impl CallServiceMessageRequest {
    pub fn new(
        from: Address,
        to: String,
        sequence_no: u128,
        rollback: Binary,
        data: Binary,
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
