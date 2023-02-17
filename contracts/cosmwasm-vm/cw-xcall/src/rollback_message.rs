use std::vec;

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
    pub fn rollbackexecuted(&self) -> Event {
        Event::new("rollbackexecuted")
            .add_attribute("sequence_no", self.sequence_no.to_string())
            .add_attribute(
                "rollback",
                String::from_utf8(self.rollback.clone()).unwrap(),
            )
            .add_attribute("message", self.message.clone())
    }

    pub fn rollbackmessage(&self) -> Event {
        Event::new("rollbackmessage").add_attribute("sequence_no", self.sequence_no.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ExecuteRollBack { sequence_no: i128 },
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
        } => rollbackexecuted(deps, sequence_no, rollback, message),
    }
}

fn rollbackexecuted(
    deps: DepsMut,
    sn: i128,
    rollback: Vec<u8>,
    message: CallServiceMessageRequest,
) -> Result<IbcReceiveResponse, ContractError> {
    let r = try_rollbackexecuted(message);
    Ok(IbcReceiveResponse::new().add_attribute("method", "execute_rollbackexecuted"))
}

pub fn try_rollbackexecuted(message: CallServiceMessageRequest) -> Result<Vec<u8>, ContractError> {
    match message.rollback().is_empty() {
        true => Err(ContractError::Unauthorized {}),
        false => return Ok(message.rollback().to_vec()),
    }
}
