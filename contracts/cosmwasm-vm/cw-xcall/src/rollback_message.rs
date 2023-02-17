use cosmwasm_std::{from_binary, Event};
use cosmwasm_std::{
    DepsMut, Env, IbcChannel, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg,
    IbcReceiveResponse,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::{self, IbcExecuteMsg};
use crate::types::address::Address;
use crate::types::request::{self, CSMessageRequests, CallServiceMessageRequest};
use crate::types::response::CSMessageResponse;
use crate::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct RollbackMessage {
    sn: i128,
    rollback: Vec<u8>,
    message: String,
}

impl RollbackMessage {
    fn rollbackexecuted(&self) -> Event {
        Event::new("rollbackexecuted")
            .add_attribute("sn", self.sn.to_string())
            .add_attribute(
                "rollback",
                String::from_utf8(self.rollback.clone()).unwrap(),
            )
            .add_attribute("message", self.message.clone())
    }

    fn rollbackmessage(&self) -> Event {
        Event::new("rollbackmessage").add_attribute("sn", self.sn.to_string())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ExecuteRollBack { sn: i128 },
}

fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::Event {
            sn,
            rollback,
            message,
        } => rollbackexecuted(deps, sn, rollback, message),
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

fn try_rollbackexecuted(message: CallServiceMessageRequest) -> Result<Vec<u8>, ContractError> {
    match message.rollback().is_empty() {
        true => Err(ContractError::Unauthorized {}),
        false => return Ok(message.rollback().to_vec()),
    }
}
