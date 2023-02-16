
use serde::{Serialize,Deserialize};
use schemars::JsonSchema;
use cosmwasm_std::{Event, from_binary};
use cosmwasm_std::{DepsMut,Env,IbcPacketReceiveMsg,IbcReceiveResponse,IbcChannel,IbcPacketTimeoutMsg,IbcPacketAckMsg};

use crate::msg::{IbcExecuteMsg, self};
use crate::types::address::Address;
use crate::types::request::{CallServiceMessageRequest, CSMessageRequests, self};
use crate::types::response::CSMessageResponse;
use crate::{ContractError };

 #[derive(Serialize, Deserialize,Clone, Debug, PartialEq, Eq, JsonSchema)]
 pub struct RollbackMessage{
    sn : i128,
    rollback : Vec<u8>,
    message : String,
 }

 impl RollbackMessage {
   
    fn rollbackexecuted(&self) -> Event{
      Event::new("rollbackexecuted").add_attribute("sn", self.sn.to_string())
                                .add_attribute("rollback", String::from_utf8(self.rollback.clone()).unwrap())
                                .add_attribute("message", self.message.clone())
    }
 

   fn rollbackmessage(&self) -> Event{
      Event::new("rollbackmessage").add_attribute("sn", self.sn.to_string())
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
        IbcExecuteMsg::Event{sn,rollback,message} => rollbackexecuted(deps,sn,rollback,message),
      //   IbcExecuteMsg::Event { sn, rollback, message } => execute_rollback(deps,sn),
    }
   }


fn rollbackexecuted(deps:DepsMut, sn: i128, rollback: Vec<u8> ,message: CallServiceMessageRequest) -> Result<IbcReceiveResponse, ContractError> {
let r  = try_rollbackexecuted(message );
Ok(IbcReceiveResponse::new()
.add_attribute("method","execute_rollbackexecuted"))
}

fn try_rollbackexecuted(message : CallServiceMessageRequest) -> Result<Vec<u8>,ContractError>{
   match message.rollback().is_empty(){
    true => Err(ContractError::Unauthorized { }),
    false => {
        return Ok(message.rollback().to_vec())
    },
}
   
}
// fn execute_rollback(deps:DepsMut, sn : i128) -> Result<IbcReceiveResponse,ContractError>{
// let e = try_execute_rollback( sn);
// Ok(IbcReceiveResponse::new()
// .add_attribute("method", "execute_rollback"))
// }
   
pub struct ExecuteRollBack{
   sequence_no : u128,
}

impl ExecuteRollBack{
fn execute_rollback( &mut self, sequence_no : u128) {

   let req = self.get(sequence_no).expect("InvalidSerialNum");
   assert_ne!(req.from, Address::zero(), "InvalidSerialNum");
  

  
   let mut msg_res = CSMessageResponse::default();

   // match self. () {
       
   // }

   
}
}
   


 



 

 