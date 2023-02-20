use crate::msg::IbcExecuteMsg;
use crate::types::request::CallServiceMessageRequest;
use crate::ContractError;
use cosmwasm_std::{from_binary, Event};
use cosmwasm_std::{DepsMut, Env, IbcPacketReceiveMsg, IbcReceiveResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RollbackMessage {
    sequence_no: i128,
    rollback: Vec<u8>,
    message: String,
}

impl RollbackMessage {
    pub fn new(sequence_no: i128, rollback: Vec<u8>, message: String) -> Self {
        Self {
            sequence_no,
            rollback,
            message,
        }
    }
    pub fn rollback_executed(&self) -> Event {
        Event::new("rollbackexecuted")
            .add_attribute("sequence_no", self.sequence_no.to_string())
            .add_attribute(
                "rollback",
                String::from_utf8(self.rollback.clone()).unwrap(),
            )
            .add_attribute("message", self.message.clone())
    }

    pub fn rollback_message(&self) -> Event {
        Event::new("rollbackmessage").add_attribute("sequence_no", self.sequence_no.to_string())
    }
}

fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::Event {
            sequence_no,
            rollback,
            message,
        } => rollback_executed(message),
    }
}

fn rollback_executed(
    message: CallServiceMessageRequest,
) -> Result<IbcReceiveResponse, ContractError> {
    try_rollbackexecuted(message)?;
    Ok(IbcReceiveResponse::new().add_attribute("method", "execute_rollbackexecuted"))
}

pub fn try_rollbackexecuted(message: CallServiceMessageRequest) -> Result<Vec<u8>, ContractError> {
    match message.rollback().is_empty() {
        true => Err(ContractError::Unauthorized {}),
        false => return Ok(message.rollback().to_vec()),
    }
}
