use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::ExecuteMsg;
use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};
use keccak_hash::keccak_256;

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Into<ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}

pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut data = [0u8; 32];
    keccak_256(input, &mut data);
    data
}
