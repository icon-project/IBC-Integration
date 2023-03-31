pub mod setup;
use std::str::FromStr;

use cw_ibc_core::{context::CwIbcCoreContext, types::PortId};
use setup::*;

#[test]
fn test_store_module_by_port() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
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
        ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let port_id = PortId::default();

    let result = ctx.lookup_module_by_port(&mut deps.storage, port_id);
    assert_eq!(result.unwrap(), module_id);
}
