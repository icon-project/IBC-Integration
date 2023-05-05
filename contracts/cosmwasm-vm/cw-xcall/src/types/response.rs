use super::*;

#[cw_serde]
pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSuccess,
}

pub fn to_int(response_type: &CallServiceResponseType) -> i8 {
    response_type.clone() as i8
}

#[cw_serde]
pub struct CallServiceMessageResponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
    message: String,
}

impl CallServiceMessageResponse {
    pub fn new(sequence_no: u128, response_code: CallServiceResponseType, message: &str) -> Self {
        Self {
            sequence_no,
            response_code,
            message: message.to_string(),
        }
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn response_code(&self) -> &CallServiceResponseType {
        &self.response_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn set_fields(
        &mut self,
        sequence_no: u128,
        response_code: CallServiceResponseType,
        message: &str,
    ) {
        self.sequence_no.clone_from(&sequence_no);
        self.response_code = response_code;
        self.message = message.to_string()
    }
}

impl Encodable for CallServiceResponseType {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        match self {
            CallServiceResponseType::CallServiceIbcError => stream.append::<i8>(&-2),
            CallServiceResponseType::CallServiceResponseFailure => stream.append::<i8>(&-1),
            CallServiceResponseType::CallServiceResponseSuccess => stream.append::<i8>(&0),
        };
    }
}

impl Decodable for CallServiceResponseType {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        match rlp.as_val::<i8>()? {
            0 => return Ok(Self::CallServiceResponseSuccess),
            -1 => return Ok(Self::CallServiceResponseFailure),
            -2 => return Ok(Self::CallServiceIbcError),
            _ => return Err(rlp::DecoderError::Custom("Invalid Bytes Sequence")),
        }
    }
}

impl Encodable for CallServiceMessageResponse {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream
            .begin_list(3)
            .append(&self.sequence_no())
            .append(self.response_code())
            .append(&self.message());
    }
}

impl Decodable for CallServiceMessageResponse {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            sequence_no: rlp.val_at(0)?,
            response_code: rlp.val_at(1)?,
            message: rlp.val_at(2)?,
        })
    }
}

impl TryFrom<&Vec<u8>> for CallServiceMessageResponse {
    type Error = ContractError;
    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value as &[u8]);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

impl TryFrom<&[u8]> for CallServiceMessageResponse {
    type Error = ContractError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(value);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}
