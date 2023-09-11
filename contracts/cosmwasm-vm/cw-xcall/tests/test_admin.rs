mod account;
mod setup;
use account::*;
use cosmwasm_std::{testing::mock_env, Addr};

use cw_xcall::state::CwCallService;
use cw_xcall_lib::xcall_msg::ExecuteMsg;
use setup::test::*;

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn set_admin_unauthorized() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(bob().to_string()),
        )
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            ExecuteMsg::SetAdmin {
                address: bob().to_string(),
            },
        )
        .unwrap();
}

#[test]
fn set_admin() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    let response = contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "set_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string())
}

#[test]
fn update_existing_admin() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(admin_two().to_string()),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_two().to_string());
}

#[test]
fn remove_existing_admin_and_set_admin() {
    let mut mock_deps = deps();

    let contract = CwCallService::default();

    let response = contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "set_admin");

    assert_eq!(response.attributes[1].value, admin_one().to_string());

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    contract
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());
}

#[test]
#[should_panic(expected = "Invalid input:")]
fn set_admin_with_empty_address() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            ExecuteMsg::SetAdmin {
                address: "".to_string(),
            },
        )
        .unwrap();

    contract
        .set_admin(mock_deps.as_mut().storage, Addr::unchecked(""))
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
        .set_admin(
            mock_deps.as_mut().storage,
            Addr::unchecked(alice().to_string()),
        )
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            ExecuteMsg::SetAdmin {
                address: "*************".to_string(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too short for this mock implementation (must be >= 3).\" })"
)]
fn validate_address_set_admin_size_lessthan_3() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            ExecuteMsg::SetAdmin {
                address: "sm".to_string(),
            },
        )
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(GenericErr { msg: \"Invalid input: human address too long for this mock implementation (must be <= 90).\" })"
)]
fn validate_address_set_admin_size_more_than_45() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);
    let mock_env = mock_env();

    let mut contract = CwCallService::default();

    contract
        .set_admin(mock_deps.as_mut().storage, mock_info.sender.clone())
        .unwrap();

    contract
        .execute(
            mock_deps.as_mut(),
            mock_env,
            mock_info,
            ExecuteMsg::SetAdmin {
                address: "eddiuo6lbp05golmz3rb5n7hbi4c5hhyh0rb1w6cslyjt5mhwd0chn3x254lyorpx4dzvrvsc9h2em44be2rj193dwe".to_string(),
            },
        )
        .unwrap();
}
