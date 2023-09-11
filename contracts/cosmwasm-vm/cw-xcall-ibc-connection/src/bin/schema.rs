use cosmwasm_schema::write_api;

use cw_common::xcall_connection_msg::{ExecuteMsg, QueryMsg};
use cw_xcall_ibc_connection::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
