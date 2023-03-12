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
