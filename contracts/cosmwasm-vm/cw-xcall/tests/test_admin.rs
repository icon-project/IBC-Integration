mod account;
mod setup;
use account::*;
use cosmwasm_std::testing::mock_env;
use cw_common::types::Address;
use cw_xcall::state::CwCallService;
use setup::*;

#[test]
#[should_panic(expected = "Unauthorized")]
fn add_admin_unauthorized() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();
}

#[test]
fn add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let response = contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string())
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn update_admin_unauthorzied() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    let mock_info = create_mock_info(&bob().to_string(), "umlg", 2000);

    contract
        .update_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();
}

#[test]
fn update_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .update_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_two().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}

#[test]
#[should_panic(expected = "AdminAlreadyExist")]
fn update_existing_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .update_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}

#[test]
#[should_panic(expected = "AdminAlreadyExist")]
fn add_existing_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let response = contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();
}

#[test]
fn remove_existing_admin_and_add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    let response = contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "add_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .remove_admin(mock_deps.as_mut().storage, mock_info.clone())
        .unwrap();
    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());
}

#[test]
#[should_panic(expected = " AdminAddressCannotBeNull")]
fn add_admin_with_empty_address() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            "".to_string(),
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
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info.clone(),
            cw_common::xcall_msg::ExecuteMsg::SetAdmin {
                address: "*************".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"*****%%%%%@@@###!1234hello\"")]
fn update_admin_invalid_chars() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env.clone(),
            mock_info.clone(),
            cw_common::xcall_msg::ExecuteMsg::SetAdmin {
                address: admin_one().to_string(),
            },
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info.clone(),
            cw_common::xcall_msg::ExecuteMsg::UpdateAdmin {
                address: "*****%%%%%@@@###!1234hello".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too short for this mock implementation (must be >= 3).\" })"
)]
fn validate_address_add_admin_size_lessthan_3() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info.clone(),
            cw_common::xcall_msg::ExecuteMsg::SetAdmin {
                address: "sm".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too long for this mock implementation (must be <= 90).\" })"
)]
fn validate_address_add_admin_size_more_than_45() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info.clone(),
            cw_common::xcall_msg::ExecuteMsg::SetAdmin {
                address: "eddiuo6lbp05golmz3rb5n7hbi4c5hhyh0rb1w6cslyjt5mhwd0chn3x254lyorpx4dzvrvsc9h2em44be2rj193dwe".into(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "InvalidAddress { address: \"new_addmin!@234\" }")]
fn update_admin_fails() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .update_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            "new_addmin!@234".into(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}

#[test]
#[should_panic(expected = "AdminAddressCannotBeNull")]
fn update_admin_fails_on_null_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallService::default();

    contract
        .add_owner(mock_deps.as_mut().storage, mock_info.sender.to_string())
        .unwrap();

    contract
        .add_admin(
            mock_deps.as_mut().storage,
            mock_info.clone(),
            admin_one().to_string(),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .update_admin(mock_deps.as_mut().storage, mock_info.clone(), "".into())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}
