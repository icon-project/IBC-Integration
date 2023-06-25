use super::*;

#[cw_serde]
pub enum CallServiceMessageType {
    CallServiceRequest = 1,
    CallServiceResponse,
}

#[cw_serde]
pub struct CallServiceMessage {
    message_type: CallServiceMessageType,
    payload: Vec<u8>,
}

impl CallServiceMessage {
    pub fn new(message_type: CallServiceMessageType, payload: Vec<u8>) -> Self {
        Self {
            message_type,
            payload: payload.to_vec(),
        }
    }

    pub fn message_type(&self) -> &CallServiceMessageType {
        &self.message_type
    }
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}

impl Encodable for CallServiceMessageType {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(1);
        match self {
            CallServiceMessageType::CallServiceRequest => stream.append::<u128>(&1),
            CallServiceMessageType::CallServiceResponse => stream.append::<u128>(&2),
        };
    }
}

impl Encodable for CallServiceMessage {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream
            .begin_list(2)
            .append(&self.message_type)
            .append(&self.payload);
    }
}

impl Decodable for CallServiceMessageType {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let data = rlp.data()?;
        let rlp = rlp::Rlp::new(data);
        match rlp.as_val::<u8>()? {
            1 => Ok(Self::CallServiceRequest),
            2 => Ok(Self::CallServiceResponse),
            _ => Err(rlp::DecoderError::Custom("Invalid Bytes Sequence")),
        }
    }
}

impl Decodable for CallServiceMessage {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            payload: rlp.val_at(1)?,
            message_type: rlp.val_at(0)?,
        })
    }
}

impl From<CallServiceMessageRequest> for CallServiceMessage {
    fn from(value: CallServiceMessageRequest) -> Self {
        Self {
            message_type: CallServiceMessageType::CallServiceRequest,
            payload: rlp::encode(&value).to_vec(),
        }
    }
}

impl From<CallServiceMessageResponse> for CallServiceMessage {
    fn from(value: CallServiceMessageResponse) -> Self {
        Self {
            message_type: CallServiceMessageType::CallServiceResponse,
            payload: rlp::encode(&value).to_vec(),
        }
    }
}

impl TryFrom<Binary> for CallServiceMessage {
    type Error = ContractError;

    fn try_from(value: Binary) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(&value);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

impl TryFrom<Vec<u8>> for CallServiceMessage {
    type Error = ContractError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let rlp = rlp::Rlp::new(&value);
        Self::decode(&rlp).map_err(|error| ContractError::DecodeFailed {
            error: error.to_string(),
        })
    }
}

impl From<CallServiceMessage> for Binary {
    fn from(value: CallServiceMessage) -> Self {
        Binary(rlp::encode(&value).to_vec())
    }
}
