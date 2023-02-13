 
use serde::{Serialize,Deserialize, __private::de::IdentifierDeserializer};
use schemars::JsonSchema;
use cosmwasm_std::Event;
use std::str::from_utf8;

 #[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, JsonSchema)]
 pub struct RollbackMessage{
    sn : u128,
    rollback : Vec<u8>,
    message : String,
 }

 impl RollbackMessage {
   
    fn event(&self) -> Event{
      Event::new("rollbackmessage").add_attribute("sn", self.sn.to_string())
                                .add_attribute("rollback", String::from_utf8(self.rollback.clone()).unwrap())
                                .add_attribute("message", self.message.clone())
    }
 }





 



 

 