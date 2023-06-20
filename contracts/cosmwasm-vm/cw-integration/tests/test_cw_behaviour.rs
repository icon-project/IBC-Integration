use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use setup::init_mock_dapp_contract;

use crate::setup::setup_context;

mod setup;
#[test]
fn test_cross_contract_rollback() {
    let mut app = setup_context(None, None);
    app = init_mock_dapp_contract(app);
    let caller = app.get_dapp();
    app = init_mock_dapp_contract(app);
    let success = app.get_dapp();
    app = init_mock_dapp_contract(app);
    let fail = app.get_dapp();
    let sender = Addr::unchecked("sender");
    let count_before: u64 = app
        .app
        .wrap()
        .query_wasm_smart(&success, &cw_mock_dapp::msg::QueryMsg::GetSequence {})
        .unwrap();
    let result = app.app.execute_contract(
        sender,
        caller,
        &cw_mock_dapp::ExecuteMsg::TestCall {
            success_addr: success.to_string(),
            fail_addr: fail.to_string(),
        },
        &[],
    );

    println!("{result:?}");

    let count_after: u64 = app
        .app
        .wrap()
        .query_wasm_smart(&success, &cw_mock_dapp::msg::QueryMsg::GetSequence {})
        .unwrap();
    assert_eq!(count_before, count_after);
}
