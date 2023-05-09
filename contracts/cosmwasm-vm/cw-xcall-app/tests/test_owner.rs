mod account;
mod setup;
use account::*;
use cw_common::types::Address;
use cw_xcall_app::state::CwCallService;
use setup::test::*;
#[test]
fn add_owner() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string())
}

#[test]
fn update_owner() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string());

    contract
        .update_owner(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            bob().to_string(),
        )
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, bob().to_string());
}

#[test]
#[should_panic(expected = "OwnerAlreadyExist")]
fn add_existing_owner() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string());

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn update_owner_unauthorized() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let result = contract.query_owner(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, mock_info.sender.to_string());

    let mock_info = create_mock_info(&bob().to_string(), "umlg", 2000);

    contract
        .update_owner(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            bob().to_string(),
        )
        .unwrap();
}
