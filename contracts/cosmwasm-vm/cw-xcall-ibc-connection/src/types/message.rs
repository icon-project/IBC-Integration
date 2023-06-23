use common::rlp::{self, Decodable, Encodable, Nullable};

pub struct Message {
    pub sn: Nullable<u64>,
    pub fee: u128,
    pub data: Vec<u8>,
}

impl Encodable for Message {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(3);
        stream.append(&self.sn);
        stream.append(&self.fee);
        stream.append(&self.data);
    }
}

impl Decodable for Message {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            sn: rlp.val_at(0)?,
            fee: rlp.val_at(1)?,
            data: rlp.val_at(2)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::rlp::{self, Nullable};

    use super::Message;

    #[test]
    fn test_rlp_decoding() {
        let message = Message {
            sn: Nullable::new(Some(150)),
            fee: 10000000000000000000,
            data: hex::decode("74657374").unwrap(),
        };
        let rlp_bytes = rlp::encode(&message);

        assert_eq!(
            "d282009689008ac7230489e800008474657374",
            hex::encode(rlp_bytes)
        );

        let message = Message {
            sn: Nullable::new(None),
            fee: 200000000000000000000,
            data: hex::decode("7465737432").unwrap(),
        };
        let rlp_bytes = rlp::encode(&message);

        assert_eq!(
            "d2f800890ad78ebc5ac6200000857465737432",
            hex::encode(rlp_bytes)
        )
    }
}
