pub mod setup;
use cosmwasm_std::Addr;
use cw_common::ibc_types::IbcModuleId;
use cw_ibc_core::context::CwIbcCoreContext;
use setup::*;

#[test]
fn proper_storage_initialisation() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    contract
        .init_client_counter(deps.as_mut().storage, 10)
        .unwrap();
    contract
        .init_connection_counter(deps.as_mut().storage, 10)
        .unwrap();

    contract
        .init_channel_counter(deps.as_mut().storage, 10)
        .unwrap();

    let connection_counter = contract.connection_counter(deps.as_ref().storage).unwrap();

    assert_eq!(10, connection_counter);

    let channel_counter = contract.channel_counter(deps.as_ref().storage).unwrap();

    assert_eq!(10, channel_counter);

    let client_counter = contract.client_counter(deps.as_ref().storage).unwrap();

    assert_eq!(10, client_counter)
}

#[test]
fn proper_router_initialisation() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let module_id = IbcModuleId::new("newModule".to_string().into()).unwrap();

    let module_address = Addr::unchecked("moduleaddress");

    contract
        .add_route(deps.as_mut().storage, module_id.clone(), &module_address)
        .unwrap();

    let result = contract
        .get_route(deps.as_ref().storage, module_id)
        .unwrap();

    assert_eq!(module_address, result)
}

#[test]
fn improper_storage_initialisation() {
    let deps = deps();
    let contract = CwIbcCoreContext::default();

    let connection_counter = contract.connection_counter(deps.as_ref().storage);

    assert!(connection_counter.is_err());

    let channel_counter = contract.channel_counter(deps.as_ref().storage);

    assert!(channel_counter.is_err());

    let client_counter = contract.client_counter(deps.as_ref().storage);

    assert!(client_counter.is_err());
}

#[test]
#[should_panic(
    expected = "IbcDecodeError { error: DecodeError { description: \"Module Id Not Found\", stack: [] } }"
)]
fn improper_router_initalisation() {
    let deps = deps();
    let contract = CwIbcCoreContext::default();

    let module_id = IbcModuleId::new("newModule".to_string().into()).unwrap();
    contract
        .get_route(deps.as_ref().storage, module_id)
        .unwrap();
}

#[test]
fn check_for_route() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let module_id = IbcModuleId::new("newModule".to_string().into()).unwrap();

    let module_address = Addr::unchecked("moduleaddress");

    contract
        .add_route(deps.as_mut().storage, module_id.clone(), &module_address)
        .unwrap();

    let result = contract.has_route(deps.as_mut().storage, module_id);

    assert!(result)
}

#[test]
fn check_for_invalid_route() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();
    let module_id = IbcModuleId::new("newModule".to_string().into()).unwrap();
    let module_id_new = IbcModuleId::new("newModule1".to_string().into()).unwrap();

    let module_address = Addr::unchecked("moduleaddress");

    contract
        .add_route(deps.as_mut().storage, module_id, &module_address)
        .unwrap();

    let result = contract.has_route(deps.as_mut().storage, module_id_new);

    assert!(!result)
}

#[test]
fn check_for_setting_valid_block_height() {
    let mut deps = deps();
    let contract = CwIbcCoreContext::default();

    let mock_env = get_mock_env();

    contract
        .block_height()
        .save(deps.as_mut().storage, &mock_env.block.height)
        .unwrap();

    let result = contract.block_height().load(deps.as_ref().storage).unwrap();

    assert_eq!(mock_env.block.height, result);
}
