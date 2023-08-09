use common::rlp::{self, Decodable, Encodable, Nullable};

pub struct Message {
    pub sn: Nullable<u64>,
    pub data: Vec<u8>,
}

impl Encodable for Message {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(3);
        stream.append(&self.sn);
        stream.append(&self.data);
    }
}

impl Decodable for Message {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self {
            sn: rlp.val_at(0)?,
            data: rlp.val_at(2)?,
        })
    }
}
