use cosmwasm_std::{Addr, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

pub fn mock_dapp_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw_mock_dapp::execute,
        cw_mock_dapp::instantiate,
        cw_mock_dapp::query,
    );
    Box::new(contract)
}

pub fn init_mock_dapp_contract(app: &mut App) -> Addr {
    let code_id = app.store_code(mock_dapp_contract());
    let sender = Addr::unchecked("sender");

    app.instantiate_contract(
        code_id,
        sender.clone(),
        &cw_mock_dapp::types::InstantiateMsg {
            address: "someaddr".to_string(),
        },
        &[],
        "MockApp",
        Some(sender.to_string()),
    )
    .unwrap()
}
