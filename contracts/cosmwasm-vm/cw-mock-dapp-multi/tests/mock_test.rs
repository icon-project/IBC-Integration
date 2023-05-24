pub mod setup;
use cosmwasm::serde::to_vec;
use cosmwasm_std::testing::mock_env;
use cw_mock_dapp_multi::{state::{CwMockService, Connection}, types::InstantiateMsg, RollbackData};
use setup::*;

#[test]
fn test_initialization() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("dapp", "umlg", 2000);
    let env = mock_env();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };

    let res = ctx.instantiate(deps.as_mut(), env, info, msg);
    println!("{:?}", res)
}

#[test]
fn test_sequence() {
    let mut deps = deps();
    let ctx = CwMockService::default();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    ctx.increment_sequence(&mut deps.storage).unwrap();
    let res = ctx.get_sequence(&deps.storage);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 1)
}

#[test]
fn test_send_message() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    let env = mock_env();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };
    ctx.instantiate(deps.as_mut(), env, info.clone(), msg)
        .unwrap();
    ctx.add_connection(deps.as_mut().storage, "netid".to_string(), Connection{src_endpoint:"somesrc".into(),dest_endpoint:"somedest".to_owned()}).unwrap();
    let res = ctx.send_call_message(
        deps.as_mut(),
        info,
        "netid/xcall".to_string(),
        vec![1, 2, 3, 4],
        Some(vec![1, 2, 3, 4, 5]),
    );
    
    assert!(res.is_ok());
    assert_eq!(res.unwrap().messages[0].id, 0)
}

#[test]
#[should_panic(expected = "ModuleAddressNotFound")]
fn test_send_message_fail() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    ctx.send_call_message(
        deps.as_mut(),
        info,
        "xcall".to_string(),
        vec![1, 2, 3, 4],
        Some(vec![1, 2, 3, 4, 5]),
    )
    .unwrap();
}

#[test]
fn test_handle_message() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    let env = mock_env();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };
    ctx.instantiate(deps.as_mut(), env, info.clone(), msg)
        .unwrap();
    let res = ctx.handle_call_message(
        deps.as_mut(),
        info,
        "xcall".to_string(),
        "helloError".as_bytes().to_vec(),
        vec![]
    );
    assert!(res.is_ok())
}

#[test]
#[should_panic(expected = "RevertFromDAPP")]
fn test_handle_message_fail_revert() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    let env = mock_env();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };
    ctx.instantiate(deps.as_mut(), env, info.clone(), msg)
        .unwrap();
    ctx.handle_call_message(
        deps.as_mut(),
        info,
        "xcall".to_string(),
        "revertMessage".as_bytes().to_vec(),
        vec![]
    )
    .unwrap();
}

#[test]
fn test_handle_message_pass_true() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    let env = mock_env();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };
    ctx.instantiate(deps.as_mut(), env, info.clone(), msg)
        .unwrap();

    ctx.roll_back()
        .save(&mut deps.storage, 1, &vec![1, 2, 3])
        .unwrap();

    let rollback_data = RollbackData {
        id: 1,
        rollback: vec![1, 2, 3],
    };
    let res = ctx.handle_call_message(
        deps.as_mut(),
        info,
        "hugobyte".to_string(),
        to_vec(&rollback_data).unwrap(),
        vec![]
    );
    assert!(res.is_ok());
    assert_eq!(res.unwrap().attributes[0].value, "RollbackDataReceived")
}

#[test]
#[should_panic(expected = "MisiingRollBack")]
fn test_handle_message_fail_true() {
    let mut deps = deps();
    let ctx = CwMockService::default();
    let info = create_mock_info("hugobyte", "umlg", 2000);
    let env = mock_env();

    ctx.init_sequence(&mut deps.storage, u64::default())
        .unwrap();
    let msg = InstantiateMsg {
        address: "xcall-address".to_string(),
    };
    ctx.instantiate(deps.as_mut(), env, info.clone(), msg)
        .unwrap();

    let rollback_data = RollbackData {
        id: 1,
        rollback: vec![1, 2, 3],
    };
    ctx.handle_call_message(
        deps.as_mut(),
        info,
        "hugobyte".to_string(),
        to_vec(&rollback_data).unwrap(),
        vec![]
    )
    .unwrap();
}
