
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

