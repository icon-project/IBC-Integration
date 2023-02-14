 
use std::ptr::null;
use serde::{Serialize,Deserialize, __private::de::IdentifierDeserializer};
use schemars::JsonSchema;
use cosmwasm_std::{Event, Never, from_binary,IbcMsg, StdError};
use cosmwasm_std::{DepsMut,Deps,Env,IbcBasicResponse,IbcPacketReceiveMsg,IbcReceiveResponse,IbcChannel,IbcPacketTimeoutMsg,IbcPacketAckMsg,IbcExecuteMsg};

use crate::{ContractError };

 #[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, JsonSchema)]
 pub struct RollbackMessage{
    sn : i128,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg{
   ExecuteRollBack{sn : i128}
}

fn do_ibc_packet_receive(
   deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
)-> Result<IbcReceiveResponse, ContractError>{
   let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::Event{sn,rollback,message} => execute_rollback(sn),
    }
}

fn execute_rollback(sn: i128) -> Result<IbcReceiveResponse, ContractError> {
let count  = try_rollback(sn)?;
Ok(IbcReceiveResponse::new()
.add_attribute("method","execute_rollback"))
.add_attribute("count", count.to_string())
.set_ack(make_ack_success()))


// Some(count)

}

pub fn try_rollback(sn:i128) -> Option<i128> {
  if sn.CallMessage(null::<_>) {
   return None
  } 
  else {
      Some(sn)
  }
//   let DApp = || -> Result<(),StdError> Ok({
//    DApp::handleCallMessage().BtpAddress().get();
//   })
   }

  
   

 



 

 