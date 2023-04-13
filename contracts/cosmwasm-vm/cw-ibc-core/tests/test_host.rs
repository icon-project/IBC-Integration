use std::time::Duration;

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
#[should_panic(expected = "IbcDecodeError { error: \"CapabilityNotFound\" }")]
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
    contract
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();
    let result = contract.get_capability(&mut deps.storage, name);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_claim_capability() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let address = "address".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), vec![address.clone()])
        .unwrap();
    contract
        .get_capability(&mut deps.storage, name.clone())
        .unwrap();
    let result = contract.claim_capability(&mut deps.storage, name, address);
    assert_eq!(result.is_ok(), true)
}

#[test]
#[should_panic(expected = "IbcDecodeError { error: \"CapabilityNotFound\" }")]
fn test_claim_capability_fails() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let contract = CwIbcCoreContext::new();
    contract
        .get_capability(&mut deps.storage, name.clone())
        .unwrap();
}

#[test]
fn test_authenticate_capability_returns_true() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let info = create_mock_info("capability", "umlg", 2000);
    let address = "capability".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), vec![address.clone()])
        .unwrap();
    let result = contract.authenticate_capability(&mut deps.storage, info, name);
    assert_eq!(result, true)
}

#[test]
#[should_panic(expected = "IbcDecodeError { error: \"CapabilityNotFound\" }")]
fn test_authenticate_capability_fails() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let info = create_mock_info("capability", "umlg", 2000);
    let contract = CwIbcCoreContext::new();
    contract.authenticate_capability(&mut deps.storage, info, name);
}

#[test]
fn test_authenticate_capability_returns_false() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let info = create_mock_info("address", "umlg", 2000);
    let address = "capability".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), vec![address.clone()])
        .unwrap();
    let result = contract.authenticate_capability(&mut deps.storage, info, name);
    assert_eq!(result, false)
}

#[test]
fn test_lookup_modules() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let address = "address".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), vec![address.clone()])
        .unwrap();

    let result = contract.lookup_modules(&mut deps.storage, name);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_set_expected_time_per_block() {
    let mut deps = deps();
    let expected_time_per_block = Duration::from_secs(20);
    let contract = CwIbcCoreContext::default();
    let block_delay = contract.calc_block_delay(&expected_time_per_block);
    let result = contract.set_expected_time_per_block(&mut deps.storage, block_delay);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn test_get_expected_time_per_block() {
    let mut deps = deps();
    let expected_time_per_block = Duration::from_secs(60);
    let contract = CwIbcCoreContext::default();
    let block_delay = contract.calc_block_delay(&expected_time_per_block);
    contract
        .set_expected_time_per_block(&mut deps.storage, block_delay)
        .unwrap();
    let result = contract
        .get_expected_time_per_block(&mut deps.storage)
        .unwrap();
    assert_eq!(61, result)
}

#[test]
#[should_panic(expected = "IbcDecodeError { error: \"NotFound\" }")]
fn test_get_expected_time_per_block_fails() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    contract
        .get_expected_time_per_block(&mut deps.storage)
        .unwrap();
}
