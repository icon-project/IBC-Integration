use super::*;

#[cw_serde]
pub enum CallServiceMessageType {
    CallServiceRequest = 1,
    CallServiceResponse,
}

#[cw_serde]
pub struct CallServiceMessage {
    pub message_type: CallServiceMessageType,
    pub payload: Vec<u8>,
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

// impl Encodable for CallServiceMessageType {
//     fn rlp_append(&self, stream: &mut rlp::RlpStream) {
//         match self {
//             CallServiceMessageType::CallServiceRequest => stream.append::<u128>(&1),
//             CallServiceMessageType::CallServiceResponse => stream.append::<u128>(&2),
//         };
//     }
// }

impl Encodable for CallServiceMessage {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        let msg_type: u8 = match self.message_type {
            CallServiceMessageType::CallServiceRequest => 1,
            CallServiceMessageType::CallServiceResponse => 2,
        };
        stream.begin_list(2).append(&msg_type).append(&self.payload);
    }
}

// impl Decodable for CallServiceMessageType {
//     fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
//         let data = rlp.data()?;
//         let rlp = rlp::Rlp::new(data);
//         match rlp.as_val::<u8>()? {
//             1 => Ok(Self::CallServiceRequest),
//             2 => Ok(Self::CallServiceResponse),
//             _ => Err(rlp::DecoderError::Custom("Invalid Bytes Sequence")),
//         }
//     }
// }

impl Decodable for CallServiceMessage {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let msg_type: u8 = rlp.val_at(0)?;

        Ok(Self {
            message_type: match msg_type {
                1 => Ok(CallServiceMessageType::CallServiceRequest),
                2 => Ok(CallServiceMessageType::CallServiceResponse),
                _ => Err(rlp::DecoderError::Custom("Invalid type")),
            }?,
            payload: rlp.val_at(1)?,
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
