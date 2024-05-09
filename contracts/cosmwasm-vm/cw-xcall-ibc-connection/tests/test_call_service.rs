mod account;
mod setup;
use cosmwasm_std::{
    testing::{mock_env, MOCK_CONTRACT_ADDR},
    Addr, Reply, SubMsgResult,
};

use cw_xcall_ibc_connection::{
    instantiate, migrate,
    msg::InstantiateMsg,
    reply,
    state::{CwIbcConnection, XCALL_HANDLE_ERROR_REPLY_ID},
    MigrateMsg,
};
use setup::*;

#[test]
fn proper_instantiate() {
    let mut mock_deps = deps();
    let mock_info = create_mock_info(MOCK_CONTRACT_ADDR, "umlg", 2000);
    let env = mock_env();
    let store = CwIbcConnection::default();

    let res = instantiate(
        mock_deps.as_mut(),
        env,
        mock_info,
        InstantiateMsg {
            ibc_host: Addr::unchecked("someaddress"),
            xcall_address: Addr::unchecked("xcalladdress"),
            denom: "arch".to_string(),
            port_id: "mock".to_string(),
        },
    )
    .unwrap();

    assert_eq!(res.messages.len(), 0);

    let owner = store.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(MOCK_CONTRACT_ADDR, owner)
}

#[test]
fn test_reply() {
    let ctx = TestContext::default();
    let mut deps = deps();

    let sub_msg_res = get_dummy_sub_msg_res();
    let msg = Reply {
        id: XCALL_HANDLE_ERROR_REPLY_ID,
        result: SubMsgResult::Ok(sub_msg_res),
    };

    let res = reply(deps.as_mut(), ctx.env, msg);
    assert!(res.is_ok())
}

#[test]
fn test_migrate() {
    let ctx = TestContext::default();
    let mut deps = deps();

    let res = migrate(deps.as_mut(), ctx.env, MigrateMsg {}).unwrap();
    assert_eq!(res.attributes[0].value, "successful")
}
