pub mod setup;
use cosmwasm_std::{testing::mock_env, Env};
use cosmwasm_std::{
    testing::{mock_dependencies, mock_info, MockApi, MockQuerier},
    Addr, MemoryStorage, OwnedDeps, Uint128,
};
use cosmwasm_std::{Coin, Event};
use cw_centralized_connection::{
    execute, msg::ExecuteMsg, state::CwCentralizedConnection, types::InstantiateMsg,
};
use cw_xcall_lib::network_address::NetId;
use std::str::FromStr;

const XCALL: &str = "xcall";
const DENOM: &str = "denom";
const RELAYER: &str = "relayer";
const OWNER: &str = "owner";

fn instantiate(
    sender: &str,
) -> (
    OwnedDeps<MemoryStorage, MockApi, MockQuerier>,
    Env,
    CwCentralizedConnection<'_>,
) {
    let mut deps: OwnedDeps<MemoryStorage, MockApi, MockQuerier> = mock_dependencies();
    let mut ctx: CwCentralizedConnection<'_> = CwCentralizedConnection::default();
    let env = mock_env();
    let info = mock_info(sender, &[]);
    let msg = InstantiateMsg {
        relayer: RELAYER.to_string(),
        xcall_address: XCALL.to_string(),
        denom: DENOM.to_string(),
    };
    let res = ctx.instantiate(deps.as_mut(), env.clone(), info, msg);
    assert!(res.is_ok());

    (deps, env, ctx)
}

#[test]
fn test_initialization() {
    instantiate(OWNER);
}

#[test]
fn test_set_admin() {
    let (mut deps, env, ctx) = instantiate("sender");
    let msg = ExecuteMsg::SetAdmin {
        address: Addr::unchecked("admin"),
    };

    let info = mock_info(OWNER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    let info = mock_info(RELAYER, &[]);

    let res = execute(deps.as_mut(), env, info, msg);
    assert!(res.is_ok());

    let admin = ctx.query_admin(deps.as_mut().storage).unwrap();
    assert_eq!(admin, Addr::unchecked("admin"));
}

#[test]
fn test_set_fee() {
    let (mut deps, env, ctx) = instantiate(OWNER);
    let nid = NetId::from_str("test").unwrap();
    let message_fee: u128 = 200;
    let response_fee: u128 = 100;
    let msg = ExecuteMsg::SetFee {
        network_id: nid.clone(),
        message_fee,
        response_fee,
    };

    let info = mock_info(OWNER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    let info = mock_info(RELAYER, &[]);

    let res = execute(deps.as_mut(), env, info, msg);
    assert!(res.is_ok());

    let res = ctx
        .get_fee(deps.as_mut().storage, nid.clone(), false)
        .unwrap();
    assert_eq!(res, Uint128::from(message_fee));

    let res = ctx.get_fee(deps.as_mut().storage, nid, true).unwrap();
    assert_eq!(res, Uint128::from(message_fee + response_fee));
}

#[test]
pub fn test_send_message() {
    let (mut deps, env, _ctx) = instantiate(OWNER);
    let msg = ExecuteMsg::SendMessage {
        to: NetId::from_str("nid").unwrap(),
        sn: 0,
        msg: vec![],
    };

    let info = mock_info(OWNER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    let info: cosmwasm_std::MessageInfo = mock_info(XCALL, &[]);

    let res = execute(deps.as_mut(), env, info, msg);
    let event = Event::new("Message")
        .add_attribute("targetNetwork", "nid")
        .add_attribute("connSn", 1.to_string())
        .add_attribute("msg", "null");
    assert_eq!(res.unwrap().events[0], event);
}

#[test]
pub fn test_recv_message() {
    let (mut deps, env, mut _ctx) = instantiate(OWNER);
    let src_network = NetId::from_str("nid").unwrap();
    let msg = ExecuteMsg::RecvMessage {
        src_network,
        conn_sn: 1,
        msg: "".to_string(),
    };

    let info = mock_info(OWNER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());
    assert_eq!("Only Relayer(Admin)", res.unwrap_err().to_string());

    let info = mock_info(RELAYER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());

    assert!(res.is_ok());

    let res = execute(deps.as_mut(), env, info, msg);

    assert!(res.is_err());

    assert_eq!("Duplicate Message", res.unwrap_err().to_string());
}

#[test]

pub fn test_revert_message() {
    let (mut deps, env, mut _ctx) = instantiate(OWNER);
    let msg = ExecuteMsg::RevertMessage { sn: 1 };

    let info = mock_info(OWNER, &[]);

    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    let info = mock_info(RELAYER, &[]);

    let res = execute(deps.as_mut(), env, info, msg);

    assert!(res.is_ok());
}

#[test]

pub fn test_get_receipts() {
    let (mut deps, env, ctx) = instantiate(OWNER);
    let src_network = NetId::from_str("nid").unwrap();
    let msg = ExecuteMsg::RecvMessage {
        src_network: src_network.clone(),
        conn_sn: 1,
        msg: "".to_string(),
    };

    let receipt = ctx.get_receipt(deps.as_mut().storage, src_network.clone(), 1);
    assert!(!receipt);

    let _ = execute(deps.as_mut(), env, mock_info(RELAYER, &[]), msg);

    let receipt = ctx.get_receipt(deps.as_mut().storage, src_network, 1);
    assert!(receipt);
}

#[test]
pub fn test_claim_fees() {
    let (mut deps, env, _ctx) = instantiate(OWNER);
    let claim_msg = ExecuteMsg::ClaimFees {};
    let info = mock_info(OWNER, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, claim_msg.clone());
    assert!(res.is_err());
    assert_eq!("Only Relayer(Admin)", res.unwrap_err().to_string());

    let msg = ExecuteMsg::SendMessage {
        to: NetId::from_str("nid").unwrap(),
        sn: 0,
        msg: vec![],
    };

    let info = mock_info(XCALL, &[]);

    let _ = execute(deps.as_mut(), env.clone(), info, msg);

    let amount: u128 = 100;
    let coin: Coin = Coin {
        denom: DENOM.to_string(),
        amount: Uint128::from(amount),
    };
    let info = mock_info(RELAYER, &[coin]);
    let res = execute(deps.as_mut(), env, info, claim_msg);
    assert!(res.is_ok());
}
