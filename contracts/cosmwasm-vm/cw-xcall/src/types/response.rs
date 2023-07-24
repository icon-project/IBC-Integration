use super::*;

#[cw_serde]
pub enum CallServiceResponseType {
    CallServiceResponseFailure,
    CallServiceResponseSuccess,
}

impl From<CallServiceResponseType> for u8 {
    fn from(val: CallServiceResponseType) -> Self {
        val as u8
    }
}

impl TryFrom<u8> for CallServiceResponseType {
    type Error = rlp::DecoderError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(CallServiceResponseType::CallServiceResponseFailure),
            1 => Ok(CallServiceResponseType::CallServiceResponseSuccess),
            _ => Err(rlp::DecoderError::Custom("Invalid type")),
        }
    }
}

#[cw_serde]
pub struct CallServiceMessageResponse {
    sequence_no: u128,
    response_code: CallServiceResponseType,
}

impl CallServiceMessageResponse {
    pub fn new(sequence_no: u128, response_code: CallServiceResponseType) -> Self {
        Self {
            sequence_no,
            response_code,
        }
    }

    pub fn sequence_no(&self) -> u128 {
        self.sequence_no
    }

    pub fn response_code(&self) -> &CallServiceResponseType {
        &self.response_code
    }

    pub fn set_fields(
        &mut self,
        sequence_no: u128,
        response_code: CallServiceResponseType,
        message: &str,
    ) {
        self.sequence_no.clone_from(&sequence_no);
        self.response_code = response_code;
    }
}

impl Encodable for CallServiceMessageResponse {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        let code: u8 = self.response_code.clone().into();

        stream
            .begin_list(2)
            .append(&self.sequence_no())
            .append(&code);
    }
}

impl Decodable for CallServiceMessageResponse {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let code: u8 = rlp.val_at(1)?;

        Ok(Self {
            sequence_no: rlp.val_at(0)?,
            response_code: CallServiceResponseType::try_from(code)?,
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

#[cfg(test)]
mod tests {
    /*
    CSMessageResponse
     sn: 1
     code: CSMessageResponse.SUCCESS
     errorMessage: errorMessage
     RLP: C20101

     CSMessageResponse
     sn: 2
     code: CSMessageResponse.FAILURE
     errorMessage: errorMessage
     RLP: C20200
     */

    use common::rlp;

    use super::{CallServiceMessageResponse, CallServiceResponseType};

    #[test]
    fn test_cs_message_response_encoding() {
        let cs_response = CallServiceMessageResponse::new(
            1,
            CallServiceResponseType::CallServiceResponseSuccess,
        );
        let encoded = rlp::encode(&cs_response);

        assert_eq!("c20101", hex::encode(encoded));

        let cs_response = CallServiceMessageResponse::new(
            2,
            CallServiceResponseType::CallServiceResponseFailure,
        );
        let encoded = rlp::encode(&cs_response);

        assert_eq!("c20200", hex::encode(encoded));
    }
}
