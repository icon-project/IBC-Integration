use cosmwasm_std::to_vec;
use cw_ibc_core::context::CwIbcCoreContext;
pub mod setup;

use ibc::core::ics24_host::validate::{
    validate_channel_identifier, validate_client_identifier, validate_connection_identifier,
    validate_identifier, validate_port_identifier,
};

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
fn invalid_port_id_min() {
    // invalid min port id
    let id = validate_port_identifier("p");
    assert!(id.is_err())
}

#[test]
fn invalid_port_id_max() {
    let id = validate_port_identifier(
        "9anxkcme6je544d5lnj46zqiiiygfqzf8w4bjecbnyj4lj6s7zlpst67yln64tixp9anxkcme6je544d5lnj46zqiiiygfqzf8w4bjecbnyj4lj6s7zlpst67yln64tixp",
    );
    assert!(id.is_err())
}

#[test]
fn valid_port_id() {
    let id = validate_port_identifier("f8w4bjecbnyj4lj6s7zlpst67yln");
    assert!(id.is_ok())
}

#[test]
fn invalid_connection_id_min() {
    let id = validate_connection_identifier("connect01");
    assert!(id.is_err())
}

#[test]
fn valid_connection_id() {
    let id = validate_connection_identifier("connection");
    assert!(id.is_ok())
}

#[test]
fn invalid_channel_id_min() {
    let id = validate_channel_identifier("channel");
    assert!(id.is_err())
}

#[test]
fn invalid_channel_id_max() {
    // invalid channel id, string length is 65 (Max - 64)
    let id = validate_channel_identifier(
        "ihhankr30iy4nna65hjl2wjod7182io1t2s7u3ip3wqtbbn1sl0rgcntqc540r36r",
    );
    assert!(id.is_err())
}

#[test]
fn parse_invalid_client_id_min() {
    // invalid min client id (Min - 9)
    let id = validate_client_identifier("client");
    assert!(id.is_err())
}

#[test]
fn valid_client_id() {
    // Max - 64
    let id = validate_client_identifier(
        "f0isrs5enif9e4td3r2jcbxoevhz6u1fthn4aforq7ams52jn5m48eiesfht9ckp",
    );
    assert!(id.is_ok())
}

#[test]
fn invalid_id_path_separator() {
    // invalid id with path separator
    let id = validate_identifier("id/1", 1, 10);
    assert!(id.is_err())
}

#[test]
fn invalid_id_chars() {
    let id = validate_identifier("channel@01", 1, 10);
    assert!(id.is_err())
}

#[test]
fn invalid_id_empty() {
    // id empty
    let id = validate_identifier("", 1, 10);
    assert!(id.is_err())
}
