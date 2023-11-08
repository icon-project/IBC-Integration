mod account;
mod setup;

use std::str::FromStr;

use cosmwasm_std::testing::mock_env;
use cw_xcall::{error::ContractError, state::CwCallService};
use cw_xcall_lib::network_address::NetId;
use setup::test::*;

#[test]
fn update_sequence() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract.sn().save(mock_deps.as_mut().storage, &0).unwrap();

    let result = contract
        .query_last_sequence_no(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, 0);

    let updated = contract
        .increment_last_sequence_no(mock_deps.as_mut().storage)
        .unwrap();

    let result = contract
        .query_last_sequence_no(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, updated);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]

fn update_sequence_without_proper_initialisation() {
    let mock_deps = deps();

    let contract = CwCallService::default();

    let result = contract
        .query_last_sequence_no(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, 1);
}

#[test]
fn update_request_id() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let result = contract
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, 0);

    contract
        .increment_last_request_id(mock_deps.as_mut().storage)
        .unwrap();

    let result = contract
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, 1);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn update_request_id_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .increment_last_request_id(mock_deps.as_mut().storage)
        .unwrap();

    let result = contract
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, 1);
}

#[test]
fn set_sequence() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract.sn().save(mock_deps.as_mut().storage, &0).unwrap();

    let updated = contract
        .set_last_sequence_no(mock_deps.as_mut().storage, 20)
        .unwrap();

    let result = contract
        .query_last_sequence_no(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, updated);
}

#[test]
fn set_request_id() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let updated = contract
        .set_last_request_id(mock_deps.as_mut().storage, 20)
        .unwrap();

    let result = contract
        .query_last_request_id(mock_deps.as_ref().storage)
        .unwrap();

    assert_eq!(result, updated);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn set_sequence_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .set_last_sequence_no(mock_deps.as_mut().storage, 20)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn set_request_id_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .set_last_request_id(mock_deps.as_mut().storage, 20)
        .unwrap();
}

#[test]
fn invalid_network_id() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info("test", "umlg", 2000);

    let env = mock_env();

    let contract = CwCallService::default();
    contract
        .instantiate(
            mock_deps.as_mut(),
            env,
            mock_info.clone(),
            cw_xcall::msg::InstantiateMsg {
                network_id: "nid".to_string(),
                denom: "arch".to_string(),
            },
        )
        .unwrap();

    let response = contract.handle_message(
        mock_deps.as_mut(),
        mock_info,
        NetId::from_str("nid").unwrap(),
        vec![],
    );

    assert!(response.is_err());
    let error = response.err().unwrap();
    assert_eq!(
        error.to_string(),
        ContractError::ProtocolsMismatch.to_string()
    );
}
