mod setup;
use std::str::FromStr;

use anyhow::Error as AppError;

use cw_multi_test::AppResponse;
use cw_multi_test::Executor;

use cw_xcall_lib::network_address::NetworkAddress;
use setup::{
    init_mock_ibc_core_contract, init_xcall_app_contract, init_xcall_ibc_connection_contract,
    TestContext,
};
use test_utils::get_event;

use crate::setup::setup_context;
const MOCK_CONTRACT_TO_ADDR: &str = "cosmoscontract";

fn setup_contracts(mut ctx: TestContext) -> TestContext {
    ctx = init_mock_ibc_core_contract(ctx);
    ctx = init_xcall_app_contract(ctx);
    ctx = init_xcall_ibc_connection_contract(ctx);
    ctx
}

fn setup_test() -> TestContext {
    let mut context = setup_context(None);
    context = setup_contracts(context);
    context
}

pub fn call_send_call_message(
    ctx: &mut TestContext,
    to: &str,
    sources: Vec<String>,
    destinations: Vec<String>,
    data: Vec<u8>,
    rollback: Option<Vec<u8>>,
) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_app(),
        &cw_xcall_lib::xcall_msg::ExecuteMsg::SendCallMessage {
            to: NetworkAddress::from_str(to).unwrap(),
            data,
            rollback,
            sources: Some(sources),
            destinations: Some(destinations),
        },
        &[],
    )
}

pub fn call_set_xcall_host(ctx: &mut TestContext) -> Result<AppResponse, AppError> {
    ctx.app.execute_contract(
        ctx.sender.clone(),
        ctx.get_xcall_ibc_connection(),
        &cw_common::xcall_connection_msg::ExecuteMsg::SetXCallHost {
            address: ctx.get_xcall_app().to_string(),
        },
        &[],
    )
}

// not possible without handshake
#[ignore]
#[test]
fn send_packet_success() {
    let mut ctx = setup_test();
    call_set_xcall_host(&mut ctx).unwrap();
    let src = ctx.get_xcall_ibc_connection().to_string();
    let result = call_send_call_message(
        &mut ctx,
        &format!("nid/{}", MOCK_CONTRACT_TO_ADDR),
        vec![src],
        vec!["somedestination".to_string()],
        vec![1, 2, 3],
        None,
    );
    println!("{result:?}");
    assert!(result.is_ok());
    let result = result.unwrap();
    let event = get_event(&result, "wasm-xcall_app_send_call_message_reply").unwrap();
    assert_eq!("success", event.get("status").unwrap());
}
