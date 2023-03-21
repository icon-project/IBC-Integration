pub mod setup;

use cw_ibc_core::{
    context::CwIbcCoreContext,
    types::{ClientId, ClientType},
};
use setup::*;

#[test]

fn get_client_next_sequence() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(mock.as_mut().storage, 0)
        .unwrap();

    let result = contract.client_counter(mock.as_ref().storage).unwrap();

    assert_eq!(result, 0)
}

#[test]
fn increment_next_client_sequence() {
    let mut mock = deps();

    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(mock.as_mut().storage, 0)
        .unwrap();

    let increment = contract
        .increase_client_counter(mock.as_mut().storage)
        .unwrap();

    let result = contract.client_counter(mock.as_ref().storage).unwrap();

    assert_eq!(increment, result)
}

#[test]
fn store_client_implement_success() {
    let mut mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_cleint_type".to_string());

    let client_id = ClientId::new(client_type, 1).unwrap();

    let light_client_address = "light-client".to_string();

    contract
        .store_client_impl(
            mock.as_mut().storage,
            client_id.clone(),
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_impls(mock.as_ref().storage, client_id)
        .unwrap();

    assert_eq!(light_client_address, result)
}

#[test]
#[should_panic(expected = "InvalidClientId { client_id: \"new_cleint_type-1\" }")]
fn store_client_implement_failure() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type, 1).unwrap();

    contract
        .get_client_impls(mock.as_ref().storage, client_id)
        .unwrap();
}

#[test]
fn store_client_into_registry() {
    let mut mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_cleint_type".to_string());
    let light_client_address = "light-client".to_string();
    contract
        .store_client_into_registry(
            mock.as_mut().storage,
            client_type.clone(),
            light_client_address.clone(),
        )
        .unwrap();

    let result = contract
        .get_client_from_registry(mock.as_ref().storage, client_type)
        .unwrap();

    assert_eq!(light_client_address, result);
}
#[test]
#[should_panic(expected = "InvalidClientType { client_type: \"new_cleint_type\" }")]
fn fails_on_querying_client_from_registry() {
    let mock = deps();
    let contract = CwIbcCoreContext::default();
    let client_type = ClientType::new("new_cleint_type".to_string());
    contract
        .get_client_from_registry(mock.as_ref().storage, client_type)
        .unwrap();
}
