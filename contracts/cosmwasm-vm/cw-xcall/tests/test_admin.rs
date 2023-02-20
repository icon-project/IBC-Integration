mod account;
mod setup;
use account::*;
use cw_xcall::{state::CwCallservice, types::address::Address};
use setup::*;

#[test]
#[should_panic(expected = "Unauthorized")]
fn add_admin_unauthorized() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();
}

#[test]
fn add_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_owner(
            mock_deps.as_mut(),
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_one())
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn update_admin_unauthorzied() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_owner(
            mock_deps.as_mut(),
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_one());

    let mock_info = create_mock_info(&bob().to_string(), "umlg", 2000);

    contract
        .update_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();
}

#[test]
fn update_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_owner(
            mock_deps.as_mut(),
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_one());

    contract
        .update_admin(mock_deps.as_mut(), mock_info.clone(), admin_two())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_two());
}

#[test]
#[should_panic(expected = "AdminAlreadyExist")]
fn update_existing_admin() {
    let mut mock_deps = deps();

    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_owner(
            mock_deps.as_mut(),
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    contract
        .add_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_one());

    contract
        .update_admin(mock_deps.as_mut(), mock_info.clone(), admin_one())
        .unwrap();

    let result = contract.query_admin(mock_deps.as_ref()).unwrap();

    assert_eq!(result, admin_two());
}
