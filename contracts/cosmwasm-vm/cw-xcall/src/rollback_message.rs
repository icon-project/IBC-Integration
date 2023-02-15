

use schemars::_serde_json::{de, Error};
use std::io::Stderr;
use serde::{Serialize,Deserialize, __private::de::IdentifierDeserializer};
use schemars::JsonSchema;
use cosmwasm_std::{Event, from_binary,IbcMsg, StdError, StdResult};
use cosmwasm_std::{DepsMut,Env,IbcBasicResponse,IbcPacketReceiveMsg,IbcReceiveResponse,IbcChannel,IbcPacketTimeoutMsg,IbcPacketAckMsg};

use crate::msg::IbcExecuteMsg;
use crate::types::request::{CSMessageRequests, CallServiceMessageRequest};
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
        IbcExecuteMsg::Event{sn,rollback,message} => execute_event(deps,sn,rollback,message),
    }
   }

fn execute_event(deps:DepsMut, sn: i128, rollback: Vec<u8> ,message: CallServiceMessageRequest) -> Result<IbcReceiveResponse, ContractError> {
let r  = try_event(message );
Ok(IbcReceiveResponse::new()
.add_attribute("method","execute_event"))
}

fn try_event(message : CallServiceMessageRequest) -> Result<Vec<u8>,ContractError>{
   match message.rollback().is_empty(){
    true => Err(ContractError::Unauthorized { }),
    false => {
        return Ok(message.rollback().to_vec())
    },
}
   
}

  
   

 



 

 