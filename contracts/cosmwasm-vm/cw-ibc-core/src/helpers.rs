use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{to_binary, Addr, CosmosMsg, StdResult, WasmMsg};

/// CwTemplateContract is a wrapper around Addr that provides a lot of helpers
/// for working with this.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct CwTemplateContract(pub Addr);

impl CwTemplateContract {
    /// The function returns a clone of the address stored in a struct.
    ///
    /// Returns:
    ///
    /// The `addr` function returns a clone of the `Addr` object stored in the first field of `self`.
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    /// This function takes a message, converts it to binary, and returns a CosmosMsg to execute the
    /// message on a contract.
    ///
    /// Arguments:
    ///
    /// * `msg`: The `msg` parameter is a generic type that implements the `Into` trait for the
    /// `cw_common::core_msg::ExecuteMsg` type. It represents the message that will be sent to the
    /// contract when the `call` function is executed. The `Into` trait allows for the message to
    ///
    /// Returns:
    ///
    /// a `StdResult` which can either be `Ok` with a `CosmosMsg` or `Err` with an error message.
    pub fn call<T: Into<cw_common::core_msg::ExecuteMsg>>(&self, msg: T) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg.into())?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }
}
