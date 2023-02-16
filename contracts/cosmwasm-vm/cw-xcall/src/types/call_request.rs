
use common::rlp::{Encodable, self, RlpStream, Rlp, Decodable};
use cosmwasm_schema::cw_serde;
use serde::{Serialize, Deserialize};

use super::address::{Address, self};


#[derive(Serialize,Deserialize)]
pub struct CallRequest{
    from : Address,
    to : String,
    rollback : Vec<u8>,
    enabled  : bool,
}

impl CallRequest{
    pub fn new(
        from : Address,
        to   : String,
        rollback : Vec<u8>,
        enabled  : bool,
    ) -> Self {
        Self {
            from,
            to,
            rollback,
            enabled,
        }
    }

    pub fn from(&self) -> &Address{
        &self.from()
    }

    pub fn to(&self) -> &String{
        &self.to()
    }

    pub fn rollback(&self) -> &Vec<u8>{
        &self.rollback()
    }

    pub fn enabled(&self) -> &bool{
       &self.enabled()
    }


}

impl Encodable for CallRequest{
    fn rlp_append(&self, s: &mut RlpStream) {
        s.begin_list(4)
        .append(&self.from)
        .append(&self.to)
        .append(&self.rollback)
        .append(&self.enabled);

    }
}

impl Decodable for CallRequest{

fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError>{
Ok(Self {
    from: rlp.val_at::<Address>(0)?,
    to: rlp.val_at::<String>(1)?,
    rollback: rlp.val_at::<Vec<u8>>(2)?,
    enabled: rlp.val_at::<bool>(3)?,
})
}
}

