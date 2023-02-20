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

mod test {
    #[test]
    fn test_add() {}
}
