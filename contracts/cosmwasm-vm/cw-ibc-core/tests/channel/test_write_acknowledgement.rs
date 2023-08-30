use common::ibc::core::ics04_channel::{channel::Order, Version};
use cosmwasm_std::{testing::mock_dependencies, Env, MessageInfo, OwnedDeps};
use cw_common::{
    ibc_types::{IbcClientId, IbcConnectionId},
    raw_types::channel::RawMessageRecvPacket,
};
use cw_ibc_core::{
    context::CwIbcCoreContext,
    conversions::{to_ibc_channel_id, to_ibc_packet, to_ibc_port_id},
    ics04_channel::{Counterparty, State},
    ChannelEnd, ConnectionEnd, VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
};

pub use super::*;
use crate::channel::{deps, get_mock_env, test_receive_packet::get_dummy_raw_msg_recv_packet};
type MockDeps = OwnedDeps<
    cosmwasm_std::MemoryStorage,
    cosmwasm_std::testing::MockApi,
    cosmwasm_std::testing::MockQuerier,
>;
pub fn setup_test(
    _info: MessageInfo,
    env: &Env,
    msg: RawMessageRecvPacket,
) -> (CwIbcCoreContext<'static>, MockDeps) {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

    let packet = msg.packet.clone().unwrap();

    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::default(),
        Counterparty::new(src_port, Some(src_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_b = ConnectionEnd::new(
        ConnectionState::Open,
        IbcClientId::default(),
        ConnectionCounterparty::new(
            IbcClientId::default(),
            Some(IbcConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    contract
        .store_channel_end(
            &mut deps.storage,
            &dst_port.clone(),
            &dst_channel.clone(),
            &chan_end_on_b,
        )
        .unwrap();
    let conn_id_on_b = &chan_end_on_b.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, &conn_id_on_b.clone(), &conn_end_on_b)
        .unwrap();

    let client_state: ClientState = get_dummy_client_state();

    let _client = client_state.to_any().encode_to_vec();
    contract
        .store_client_commitment(
            &mut deps.storage,
            env,
            &IbcClientId::default(),
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();

    let proof_height = to_ibc_height(msg.proof_height).unwrap();
    let _consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_commitment(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();
    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(&mut deps.storage, &dst_port, "moduleaddress".to_string())
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, &IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);

    contract
        .store_channel_end(
            &mut deps.storage,
            &dst_port.clone(),
            &dst_channel,
            &chan_end_on_b.clone(),
        )
        .unwrap();
    (contract, deps)
}

#[test]
fn test_write_acknowledgement() {
    let env = get_mock_env();
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    test_context.init_receive_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
    let packet = msg.packet.unwrap();
    let ibc_packet = to_ibc_packet(packet).unwrap();

    let info = create_mock_info("moduleaddress", "umlg", 2000000000);
    let ack_result = contract.write_acknowledgement(deps.as_mut(), info, ibc_packet, vec![11, 22]);
    println!("{ack_result:?}");
    assert!(ack_result.is_ok())
}

#[test]
#[should_panic(expected = "Unauthorized")]
pub fn test_write_acknowledgement_fails_unauthorized() {
    let env = get_mock_env();
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::default();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    test_context.init_receive_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);
    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
    let packet = msg.packet.unwrap();
    let ibc_packet = to_ibc_packet(packet).unwrap();

    let info = create_mock_info("invalidmoduleaddress", "umlg", 2000000000);
    let ack_result = contract.write_acknowledgement(deps.as_mut(), info, ibc_packet, vec![11, 22]);
    println!("{ack_result:?}");
    ack_result.unwrap();
}

#[test]
#[should_panic(expected = "IbcPacketError { error: InvalidAcknowledgement }")]
pub fn test_write_acknowledgement_fails_invalid_ack() {
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::default();
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);
    test_context.init_receive_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries, &mut deps);

    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
    let packet = msg.packet.unwrap();
    let ibc_packet = to_ibc_packet(packet).unwrap();

    let info = create_mock_info("moduleaddress", "umlg", 2000000000);
    let ack_result = contract.write_acknowledgement(deps.as_mut(), info, ibc_packet, vec![]);
    println!("{ack_result:?}");
    ack_result.unwrap();
}

#[test]
#[should_panic(
    expected = "IbcChannelError { error: InvalidChannelState { channel_id: ChannelId(\"channel-3\"), state: Closed } }"
)]
pub fn test_write_acknowledgement_fails_invalid_channel_state() {
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let mut deps = mock_dependencies();
    let contract = CwIbcCoreContext::default();
    let mut test_context = TestContext::for_receive_packet(env.clone(), &msg);

    test_context.init_receive_packet(deps.as_mut().storage, &contract);
    mock_lightclient_query(test_context.mock_queries.clone(), &mut deps);
    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
    let packet = msg.packet.unwrap();

    let mut channel_end = test_context.channel_end();
    channel_end.state = State::Closed;
    test_context.channel_end = Some(channel_end);
    test_context.save_channel_end(deps.as_mut().storage, &contract);

    let ibc_packet = to_ibc_packet(packet).unwrap();

    let info = create_mock_info("moduleaddress", "umlg", 2000000000);
    let ack_result = contract.write_acknowledgement(deps.as_mut(), info, ibc_packet, vec![11, 22]);
    println!("{ack_result:?}");
    ack_result.unwrap();
}
