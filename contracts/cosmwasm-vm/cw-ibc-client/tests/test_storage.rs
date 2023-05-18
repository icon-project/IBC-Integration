pub mod setup;
use cosmwasm_std::{testing::mock_env, Addr};
use cw_common::types::ModuleId;
use cw_ibc_client::context::CwIbcClientContext;
use setup::*;

#[test]
fn proper_storage_initialisation() {
    let mut deps = deps();
    let contract = CwIbcClientContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 10)
        .unwrap();

    let client_counter = contract.client_counter(deps.as_ref().storage).unwrap();

    assert_eq!(10, client_counter)
}

#[test]
#[test]
fn improper_storage_initialisation() {
    let deps = deps();
    let contract = CwIbcClientContext::default();

    let client_counter = contract.client_counter(deps.as_ref().storage);

    assert!(client_counter.is_err());
}

#[test]
fn check_for_setting_valid_block_height() {
    let mut deps = deps();
    let contract = CwIbcClientContext::default();

    let mock_env = mock_env();

    contract
        .block_height()
        .save(deps.as_mut().storage, &mock_env.block.height)
        .unwrap();

    let result = contract.block_height().load(deps.as_ref().storage).unwrap();

    assert_eq!(mock_env.block.height, result);
}
