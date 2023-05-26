use cosmwasm_std::Addr;
use cw_multi_test::{App, Executor};
use setup::init_mock_dapp_contract;

mod setup;
#[test]
fn test_cross_contract_rollback() {
    let mut app = App::default();
    let caller = init_mock_dapp_contract(&mut app);
    let success = init_mock_dapp_contract(&mut app);
    let fail = init_mock_dapp_contract(&mut app);
    let sender = Addr::unchecked("sender");
    let count_before: u64 = app
        .wrap()
        .query_wasm_smart(&success, &cw_mock_dapp::msg::QueryMsg::GetSequence {})
        .unwrap();
    let result = app.execute_contract(
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
        .wrap()
        .query_wasm_smart(&success, &cw_mock_dapp::msg::QueryMsg::GetSequence {})
        .unwrap();
    assert_eq!(count_before, count_after);
}
