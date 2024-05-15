use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Coin,
};
use cw_wasm_light_client::contract::instantiate;
use cw_wasm_light_client::msg::InstantiateMsg;
use test_utils::get_test_headers;

use crate::setup::TestContext;
mod setup;

#[test]
pub fn instantiate_success() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[Coin::new(100, "test")]);
    let header = &get_test_headers()[0];
    let context = TestContext::for_instantiate();
    context.init(deps.as_mut().storage, header);
    let msg = InstantiateMsg {};
    let result = instantiate(deps.as_mut(), env, info, msg);
    println!("{result:?}");
    assert!(result.is_ok())
}

#[test]
#[should_panic(expected = " ClientStateNotFound(\"08-iconwasm-0\")")]
pub fn instantiate_fails_on_uninitialized_storage() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info("sender", &[Coin::new(100, "test")]);
    let _header = &get_test_headers()[0];
    let _context = TestContext::for_instantiate();

    let msg = InstantiateMsg {};
    let result = instantiate(deps.as_mut(), env, info, msg);
    result.unwrap();
}
