use std::str::FromStr;

use cosmwasm_std::to_vec;
use cw_ibc_core::{
    context::CwIbcCoreContext,
    types::{ChannelId, ClientId, ClientType, ConnectionId, PortId},
};
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
fn test_validate_client_id() {
    let id = ClientId::default();
    let result = validate_client_identifier(id.as_str());
    assert_eq!(result.is_ok(), true)
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn test_validate_client_id_fail_invalid_min_length() {
    let client_type = ClientType::new("new".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();
    validate_client_identifier(client_id.as_str()).unwrap();
}

#[test]
#[should_panic(expected = "InvalidLength")]
fn test_validate_client_id_fail_invalid_max_length() {
    let client_type = ClientType::new(
        "newhauyduiwe73o59jklsjkdnklsnakalkjhdertyuiimnndvxgwgrtyuuropssrt".to_string(),
    );
    let client_id = ClientId::new(client_type, 1).unwrap();
    validate_client_identifier(client_id.as_str()).unwrap();
}

#[test]
fn test_validate_connection_id() {
    let conn_id = ConnectionId::default();
    let result = validate_connection_identifier(conn_id.as_str());
    assert_eq!(result.is_ok(), true)
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn test_validate_connection_id_fail_invalid_min_length() {
    let s = "qwertykey";
    let conn_id = ConnectionId::from_str(s).unwrap();
    validate_connection_identifier(conn_id.as_str()).unwrap();
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn test_validate_connection_id_fail_invalid_max_length() {
    let s = "qwertykeywe73o59jklsjkdnklsnakalkjhdertyuiimnndvxgwgrtyuuropsttt5";
    let conn_id = ConnectionId::from_str(s).unwrap();
    validate_connection_identifier(conn_id.as_str()).unwrap();
}

#[test]
fn test_validate_channel_id() {
    let channel_id = ChannelId::default();
    let result = validate_channel_identifier(channel_id.as_str());
    assert_eq!(result.is_ok(), true)
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
fn test_port_id() {
    let port_id = PortId::default();
    let result = validate_port_identifier(port_id.as_str());
    assert_eq!(result.is_ok(), true)
}

#[test]
#[should_panic(expected = "IbcDecodeError")]
fn test_validate_port_id_fail_invalid_min_length() {
    let s = "q";
    let id = PortId::from_str(s).unwrap();
    let result = validate_port_identifier(id.as_str());
    assert_eq!(result.is_ok(), true)
}
