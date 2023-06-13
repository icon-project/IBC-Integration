use std::str::FromStr;
use std::time::Duration;

use common::ibc::core::ics02_client::client_type::ClientType;
use common::ibc::core::ics24_host::identifier::ClientId;
use common::ibc::core::ics24_host::identifier::{ConnectionId, PortId};
use cosmwasm_std::to_vec;
use cw_ibc_core::context::CwIbcCoreContext;
pub mod setup;

use common::ibc::core::ics24_host::validate::validate_identifier;

use setup::*;

#[test]
fn test_set_capability() {
    let mut deps = deps();
    let name = to_vec(&u128::default()).unwrap();
    let address = "helo".to_string();
    let contract = CwIbcCoreContext::default();
    let result = contract.store_capability(&mut deps.storage, name, address);
    assert!(result.is_ok())
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"CapabilityNotFound\", stack: [] } }"
)]
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
    let address = "hello".to_string();
    let contract = CwIbcCoreContext::default();
    contract
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();
    let result = contract.get_capability(&mut deps.storage, name);
    assert!(result.is_ok())
}

#[test]
fn test_claim_capability() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let address = "address".to_string();
    let address_to_claim = "address-2".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();
    let result = contract.claim_capability(&mut deps.storage, name, address_to_claim);
    // only one address to one port
    assert!(result.is_err());
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"CapabilityNotFound\", stack: [] } }"
)]
fn test_claim_capability_fails() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let contract = CwIbcCoreContext::new();
    contract.get_capability(&mut deps.storage, name).unwrap();
}

#[test]
fn test_authenticate_capability_returns_true() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let info = create_mock_info("capability", "umlg", 2000);
    let address = "capability".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();
    let result = contract.authenticate_capability(&mut deps.storage, info, name);
    assert!(result)
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"CapabilityNotFound\", stack: [] } }"
)]
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
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();
    let result = contract.authenticate_capability(&mut deps.storage, info, name);
    assert!(!result)
}

#[test]
fn test_lookup_modules() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let address = "address".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), address)
        .unwrap();

    let result = contract.lookup_modules(&mut deps.storage, name);
    assert!(result.is_ok())
}

#[test]
fn test_set_expected_time_per_block() {
    let mut deps = deps();
    let expected_time_per_block = Duration::from_secs(20);
    let contract = CwIbcCoreContext::default();
    let block_delay = contract.calc_block_delay(&expected_time_per_block);
    let result = contract.set_expected_time_per_block(&mut deps.storage, block_delay);
    assert!(result.is_ok())
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
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"NotFound\", stack: [] } }"
)]
fn test_get_expected_time_per_block_fails() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    contract
        .get_expected_time_per_block(&mut deps.storage)
        .unwrap();
}

#[test]
fn test_validate_client_id_fail_invalid_min_length() {
    let client_type = ClientType::new("new".to_string());
    let client_id = ClientId::new(client_type, 1);
    assert!(client_id.is_err())
}

#[test]
fn test_validate_client_id_fail_invalid_max_length() {
    let client_type = ClientType::new(
        "newhauyduiwe73o59jklsjkdnklsnakalkjhdertyuiimnndvxgwgrtyuuropssrt".to_string(),
    );
    let client_id = ClientId::new(client_type, 1);
    assert!(client_id.is_err())
}

#[test]
fn test_validate_connection_id_fail_invalid_min_length() {
    let s = "qwertykey";
    let conn_id = ConnectionId::from_str(s);
    assert!(conn_id.is_err())
}

#[test]
fn test_validate_connection_id_fail_invalid_max_length() {
    let s = "qwertykeywe73o59jklsjkdnklsnakalkjhdertyuiimnndvxgwgrtyuuropsttt5";
    let conn_id = ConnectionId::from_str(s);
    assert!(conn_id.is_err())
}

#[test]
#[should_panic(expected = "Empty")]
fn test_validate_id_empty() {
    let id = "";
    let min = 1;
    let max = 10;
    validate_identifier(id, min, max).unwrap();
}

#[test]
#[should_panic(expected = "ContainSeparator")]
fn test_validate_id_have_path_separator() {
    let id = "id/1";
    let min = 1;
    let max = 10;
    validate_identifier(id, min, max).unwrap();
}

#[test]
#[should_panic(expected = "InvalidCharacter")]
fn test_validate_id_have_invalid_chars() {
    let id = "channel@01";
    let min = 1;
    let max = 10;
    validate_identifier(id, min, max).unwrap();
}

#[test]
fn test_validate_port_id_fail_invalid_min_length() {
    let s = "q";
    let id = PortId::from_str(s);
    assert!(id.is_err())
}

#[test]
#[should_panic(expected = "IbcContextError { error: \"Capability already claimed\" }")]
fn test_already_claimed_capability() {
    let mut deps = deps();
    let name: Vec<u8> = vec![2];
    let address = "address".to_string();
    let contract = CwIbcCoreContext::new();
    contract
        .store_capability(&mut deps.storage, name.clone(), address.clone())
        .unwrap();
    contract
        .get_capability(&mut deps.storage, name.clone())
        .unwrap();
    contract
        .claim_capability(&mut deps.storage, name, address)
        .unwrap();
}
