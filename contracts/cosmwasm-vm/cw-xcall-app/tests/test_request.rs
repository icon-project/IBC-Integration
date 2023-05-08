mod account;
mod setup;

use cw_xcall_app::state::CwCallService;
use setup::*;

#[test]
fn update_sequence() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

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
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .increment_last_sequence_no(mock_deps.as_mut().storage)
        .unwrap();

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

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

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
