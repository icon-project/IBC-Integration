pub mod setup;
use common::ibc::core::ics24_host::identifier::PortId;
use common::utils::keccak256;
use cosmwasm_std::Addr;
use cw_common::commitment;
use cw_common::ibc_types::IbcChannelId;
use cw_ibc_core::context::CwIbcCoreContext;
use setup::*;
use std::str::{from_utf8, FromStr};
#[test]
fn test_store_module_by_port() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let port_id = PortId::default();
    ctx.store_module_by_port(&mut deps.storage, &port_id.clone(), module_id.clone())
        .unwrap();

    let result = ctx.lookup_module_by_port(&mut deps.storage, &port_id);
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

    let result = ctx.lookup_module_by_port(&mut deps.storage, &port_id);
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
    let key: Vec<u8> = keccak256(&port_path).into();

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
fn test_bind_port() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let path = PortId::default().to_string().as_bytes().to_vec();

    let res = ctx.bind_port(
        &mut deps.storage,
        &PortId::default(),
        Addr::unchecked("ContractAddress"),
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
