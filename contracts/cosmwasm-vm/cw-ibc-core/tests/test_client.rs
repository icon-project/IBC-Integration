pub mod setup;

use cw_ibc_core::context::CwIbcCoreContext;
use setup::*;

#[test]

fn get_client_next_sequence() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::new();

    contract
        .init_next_client_sequence(mock.as_mut().storage, 0)
        .unwrap();

    let result = contract
        .get_next_client_sequence(mock.as_ref().storage)
        .unwrap();

    assert_eq!(result, 0)
}

#[test]
fn increment_next_client_sequence() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::new();

    contract
        .init_next_client_sequence(mock.as_mut().storage, 0)
        .unwrap();

    let increment = contract
        .increment_next_client_sequence(mock.as_mut().storage)
        .unwrap();

    let result = contract
        .get_next_client_sequence(mock.as_ref().storage)
        .unwrap();

    assert_eq!(increment, result)
}
