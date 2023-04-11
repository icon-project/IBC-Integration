mod account;
mod setup;
use account::*;
use cosmwasm_std::{from_binary, testing::mock_env};
use cw_xcall::{state::CwCallService, types::address::Address};
use setup::*;

#[test]
#[should_panic(expected = "Unauthorized")]
fn add_admin_unauthorized() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();
}

#[test]
fn add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    let response = contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one())
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn update_admin_unauthorzied() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    let mock_info = create_mock_info(&bob().to_string(), "umlg", 2000);

    contract
        .update_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();
}

#[test]
fn update_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    contract
        .update_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_two())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two());
}

#[test]
#[should_panic(expected = "AdminAlreadyExist")]
fn update_existing_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    contract
        .update_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two());
}

#[test]
#[should_panic(expected = "AdminAlreadyExist")]
fn add_existing_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    let response = contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();
}

#[test]
fn remove_existing_admin_and_add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    let response = contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    contract
        .remove_admin(mock_deps.as_mut().storage, mock_info.clone())
        .unwrap();
    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());
}

#[test]
#[should_panic(expected = " AdminAddressCannotBeNull")]
fn add_admin_with_empty_address() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            Address::from(""),
        )
        .unwrap();
}

#[test]
fn query_admin() {
    let mock_deps = deps();

    let mock_env = mock_env();

    let contract = CwCallService::default();

    let result = contract.query(
        mock_deps.as_ref(),
        mock_env,
        cw_xcall::msg::QueryMsg::GetAdmin {},
    );

    assert!(result.is_err())
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"*************\"")]
fn add_invalid_char_as_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            "*************".into(),
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"*****%%%%%@@@###!1234hello\"")]
fn update_admin_invalid_chars() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut().storage, mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one());

    contract
        .update_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            "*****%%%%%@@@###!1234hello".into(),
        )
        .unwrap();
}
