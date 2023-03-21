pub mod setup;

use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::events::{
        client_misbehaviour_event, create_client_event, update_client_event, upgrade_client_event,
    },
    types::{ClientId, ClientType},
    MsgUpdateClient, MsgUpgradeClient,
};
use ibc::{
    core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour,
    mock::{
        client_state::MockClientState, consensus_state::MockConsensusState, header::MockHeader,
    },
    Height,
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

#[test]
fn test_create_client_event() {
    let height = Height::new(15, 10).unwrap();

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 1).unwrap();
    let result = create_client_event(
        client_id.ibc_client_id().clone(),
        client_type.client_type(),
        height,
    );

    assert_eq!("create_client", result.ty)
}

#[test]
fn test_raw_update_client_event() {
    let raw_message = get_dummy_raw_msg_update_client_message();
    let message: MsgUpdateClient = MsgUpdateClient::try_from(raw_message.clone()).unwrap();
    let height = Height::new(15, 10).unwrap();
    let client_type = ClientType::new("new_cleint_type".to_string());
    let result = update_client_event(
        client_type.client_type(),
        height,
        vec![height],
        message.clone(),
    );

    assert_eq!("update_client", result.ty);

    assert_eq!(raw_message, message.into())
}

#[test]
fn test_upgrade_client_event() {
    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();
    let signer = get_dummy_account_id();

    let height = Height::new(1, 1).unwrap();

    let client_state = MockClientState::new(MockHeader::new(height));
    let consensus_state = MockConsensusState::new(MockHeader::new(height));

    let proof = get_dummy_merkle_proof();

    let msg = MsgUpgradeClient {
        client_id: client_id.ibc_client_id().clone(),
        client_state: client_state.into(),
        consensus_state: consensus_state.into(),
        proof_upgrade_client: proof.clone(),
        proof_upgrade_consensus_state: proof,
        signer,
    };

    let raw_message: ibc_proto::ibc::core::client::v1::MsgUpgradeClient =
        ibc_proto::ibc::core::client::v1::MsgUpgradeClient::try_from(msg.clone()).unwrap();

    let upgrade_message_from_raw_message = MsgUpgradeClient::try_from(raw_message).unwrap();

    assert_eq!(upgrade_message_from_raw_message, msg);

    let event = upgrade_client_event(client_type.client_type(), height, msg);

    assert_eq!("upgrade_client", event.ty)
}

#[test]
fn create_misbehaviour_event_test() {
    let raw_message = get_dummy_raw_msg_client_mishbehaviour();
    let misbehaviour: MsgSubmitMisbehaviour =
        MsgSubmitMisbehaviour::try_from(raw_message.clone()).unwrap();

    let raw_message_from_mb: ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour =
        ibc_proto::ibc::core::client::v1::MsgSubmitMisbehaviour::try_from(misbehaviour).unwrap();

    assert_eq!(raw_message, raw_message_from_mb);

    let client_type = ClientType::new("new_cleint_type".to_string());
    let client_id = ClientId::new(client_type.clone(), 10).unwrap();

    let event =
        client_misbehaviour_event(client_id.ibc_client_id().clone(), client_type.client_type());

    assert_eq!("client_misbehaviour", event.ty)
}
