pub mod setup;
use common::ibc::core::ics24_host::identifier::PortId;
use common::utils::keccak256;
use cw_common::commitment;
use cw_common::ibc_types::IbcChannelId;
use cw_ibc_core::{context::CwIbcCoreContext, ics04_channel::ChannelMsg, MsgChannelOpenInit};
use setup::*;
use std::str::{from_utf8, FromStr};
#[test]
fn test_store_module_by_port() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let port_id = PortId::default();
    ctx.store_module_by_port(&mut deps.storage, port_id.clone(), module_id.clone())
        .unwrap();

    let result = ctx.lookup_module_by_port(&mut deps.storage, port_id);
    assert_eq!(result.unwrap(), module_id);
}

#[test]
#[should_panic(expected = "error: UnknownPort { port_id: PortId(\"defaultPort\")")]
fn test_store_module_by_port_fail() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let port_id = PortId::default();

    let result = ctx.lookup_module_by_port(&mut deps.storage, port_id);
    assert_eq!(result.unwrap(), module_id);
}

#[test]
fn check_for_port_path() {
    let _ctx = CwIbcCoreContext::default();

    let port_id = PortId::default();
    let port_path = commitment::port_path(&port_id);

    assert_eq!("ports/defaultPort", String::from_utf8(port_path).unwrap())
}

#[test]
fn check_for_port_path_key() {
    let _ctx = CwIbcCoreContext::default();

    let port_id = PortId::default();
    let port_path = commitment::port_path(&port_id);
    let key: Vec<u8> = keccak256(&port_path.clone()).into();

    let port_path_key = commitment::port_commitment_key(&port_id);

    assert_eq!(key, port_path_key)
}

#[test]
#[should_panic(expected = "InvalidLength { id: \"s\", length: 1, min: 2, max: 128 }")]
fn fails_on_invalid_length_for_port_id() {
    PortId::from_str("s").unwrap();
}

#[test]
fn check_for_port_id() {
    let port_id = PortId::from_str("xcall");

    assert!(port_id.is_ok())
}

#[test]
fn test_lookup_module_channel() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let msg = MsgChannelOpenInit::try_from(get_dummy_raw_msg_chan_open_init(None)).unwrap();
    ctx.store_module_by_port(
        &mut deps.storage,
        msg.port_id_on_a.clone().into(),
        module_id.clone(),
    )
    .unwrap();
    let channel_msg = ChannelMsg::OpenInit(msg);
    let res = ctx.lookup_module_channel(&mut deps.storage, &channel_msg);

    assert!(res.is_ok());
    assert_eq!("contractaddress", res.unwrap().to_string())
}

#[test]
#[should_panic(expected = "UnknownPort")]
fn test_lookup_module_channel_fail() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let msg = MsgChannelOpenInit::try_from(get_dummy_raw_msg_chan_open_init(None)).unwrap();
    let channel_msg = ChannelMsg::OpenInit(msg);
    ctx.lookup_module_channel(&mut deps.storage, &channel_msg)
        .unwrap();
}

#[test]
fn test_bind_port() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let path = commitment::port_path(&PortId::default());

    let res = ctx.bind_port(
        &mut deps.storage,
        &PortId::default(),
        "ContractAddress".to_string(),
    );
    assert!(res.is_ok());
    let expected = ctx.get_capability(&mut deps.storage, path);
    assert!(expected.is_ok());
    assert!(expected.unwrap().contains(&"ContractAddress".to_string()))
}

#[test]
fn channel_capability_path() {
    let ctx = CwIbcCoreContext::default();
    let res = ctx.channel_capability_path(&PortId::default(), &IbcChannelId::default());
    let result = from_utf8(&res);
    assert!(result.is_ok());
    assert_eq!("ports/defaultPort/channels/channel-0", result.unwrap())
}
