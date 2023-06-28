use super::*;

#[cw_serde]
pub enum CallServiceResponseType {
    CallServiceIbcError = -2,
    CallServiceResponseFailure,
    CallServiceResponseSuccess,
}

impl From<CallServiceResponseType> for i8 {
    fn from(val: CallServiceResponseType) -> Self {
        val as i8
    }
}

impl TryFrom<i8> for CallServiceResponseType {
    type Error = rlp::DecoderError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -2 => Ok(CallServiceResponseType::CallServiceIbcError),
            -1 => Ok(CallServiceResponseType::CallServiceResponseFailure),
            0 => Ok(CallServiceResponseType::CallServiceResponseSuccess),
            _ => Err(rlp::DecoderError::Custom("Invalid type")),
        }
    }
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

impl Encodable for CallServiceMessageResponse {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        let code: i8 = self.response_code.clone().into();

        stream
            .begin_list(3)
            .append(&self.sequence_no())
            .append(&code)
            .append(&self.message());
    }
}

impl Decodable for CallServiceMessageResponse {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let code: i8 = rlp.val_at(1)?;

        Ok(Self {
            sequence_no: rlp.val_at(0)?,
            response_code: CallServiceResponseType::try_from(code)?,
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

#[cfg(test)]
mod tests {
    /*
    CSMessageResponse
     sn: 37
     code: CSMessageResponse.FAILURE
     errorMessage: errorMessage
     RLP: D02581FF8C6572726F724D657373616765

     CSMessageResponse
     sn: 22
     code: CSMessageResponse.SUCCESS
     errorMessage: errorMessage
     RLP: CF16008C6572726F724D657373616765
     */

    use common::rlp;

    use super::{CallServiceMessageResponse, CallServiceResponseType};

    #[test]
    fn test_cs_message_response_encoding() {
        let cs_response = CallServiceMessageResponse::new(
            37,
            CallServiceResponseType::CallServiceResponseFailure,
            "errorMessage",
        );
        let encoded = rlp::encode(&cs_response);

        assert_eq!("d02581ff8c6572726f724d657373616765", hex::encode(encoded));

        let cs_response = CallServiceMessageResponse::new(
            22,
            CallServiceResponseType::CallServiceResponseSuccess,
            "errorMessage",
        );
        let encoded = rlp::encode(&cs_response);

        assert_eq!("cf16008c6572726f724d657373616765", hex::encode(encoded));
    }
}
