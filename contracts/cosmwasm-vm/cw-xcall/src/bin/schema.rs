use cosmwasm_schema::write_api;

use cw_common::xcall_msg::ExecuteMsg;
use cw_xcall::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
