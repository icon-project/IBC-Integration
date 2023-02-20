mod account;
mod setup;

use cw_xcall::state::CwCallservice;
use setup::*;

#[test]
fn update_sequence() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let result = contract.query_last_sequence_no(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 0);

    contract
        .increment_last_sequence_no(mock_deps.as_mut())
        .unwrap();

    let result = contract.query_last_sequence_no(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 1);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]

fn update_sequence_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .increment_last_sequence_no(mock_deps.as_mut())
        .unwrap();

    let result = contract.query_last_sequence_no(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 1);
}

#[test]
fn update_request_id() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    let result = contract.query_last_request_id(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 0);

    contract
        .increment_last_request_id(mock_deps.as_mut())
        .unwrap();

    let result = contract.query_last_request_id(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 1);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn update_request_id_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .increment_last_request_id(mock_deps.as_mut())
        .unwrap();

    let result = contract.query_last_request_id(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 1);
}

#[test]
fn set_sequence() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .last_sequence_no()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    contract
        .set_last_sequence_no(mock_deps.as_mut(), 20)
        .unwrap();

    let result = contract.query_last_sequence_no(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 20);
}

#[test]
fn set_request_id() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .last_request_id()
        .save(mock_deps.as_mut().storage, &0)
        .unwrap();

    contract
        .set_last_request_id(mock_deps.as_mut(), 20)
        .unwrap();

    let result = contract.query_last_request_id(mock_deps.as_ref()).unwrap();

    assert_eq!(result, 20);
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn set_sequence_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .set_last_sequence_no(mock_deps.as_mut(), 20)
        .unwrap();
}

#[test]
#[should_panic(expected = "Std(NotFound { kind: \"u128\" })")]
fn set_request_id_without_proper_initialisation() {
    let mut mock_deps = deps();

    let contract = CwCallservice::default();

    contract
        .set_last_request_id(mock_deps.as_mut(), 20)
        .unwrap();
}
