use cosmwasm_std::to_vec;
use cw_ibc_core::context::CwIbcCoreContext;
pub mod setup;
use setup::*;

#[test]
fn test_set_capability() {
    let mut deps = deps();
    let name = to_vec(&u128::default()).unwrap();
    let address = vec!["helo".to_string()];
    let contract = CwIbcCoreContext::default();
    let result = contract.store_capability(&mut deps.storage, name, address);
    assert_eq!(result.is_ok(), true)
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"alloc::vec::Vec<alloc::string::String>\" })")]
fn test_get_capability_fail() {
    let mut deps = deps();
    let name = to_vec(&u128::default()).unwrap();
    let contract = CwIbcCoreContext::default();
    contract.get_capability(&mut deps.storage, name).unwrap();
}

#[test]
fn test_get_capability() {
    let mut deps = deps();
    let name = to_vec(&u128::default()).unwrap();
    let address = vec!["hello".to_string()];
    let contract = CwIbcCoreContext::default();
    contract.store_capability(&mut deps.storage, name.clone(), address).unwrap();
    let result = contract.get_capability(&mut deps.storage, name);
    assert_eq!(result.is_ok(), true)
}
