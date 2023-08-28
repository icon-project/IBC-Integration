use cosmwasm_std::testing::mock_env;

use cw_common::core_msg::InstantiateMsg;
use cw_ibc_core::context::CwIbcCoreContext;
use setup::{create_mock_info, deps};

mod setup;

#[test]
pub fn only_owner_can_set_expected_time_per_block() {
    let mut deps = deps();
    let env = mock_env();
    let info = create_mock_info("sender", "test", 0);
    let mut contract = CwIbcCoreContext::default();
    let init_msg = InstantiateMsg {};
    let _init = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg)
        .unwrap();
    let set_msg = cw_common::core_msg::ExecuteMsg::SetExpectedTimePerBlock { block_time: 10 };

    let res = contract.execute(deps.as_mut(), env.clone(), info, set_msg.clone());

    assert!(res.is_ok());
    let info = create_mock_info("nonowner", "test", 0);
    let res = contract.execute(deps.as_mut(), env, info, set_msg);
    assert!(res.is_err());
}
