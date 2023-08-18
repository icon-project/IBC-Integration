use cosmwasm_std::testing::mock_info;
use cw_ibc_core::conversions::{to_ibc_channel_id, to_ibc_port_id};

use super::*;

#[test]
fn test_packet_send() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;

    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    packet.sequence = 1;
    packet.data = vec![0];
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let _dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let _dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(
            to_ibc_port_id(&packet.destination_port).unwrap(),
            Some(to_ibc_channel_id(&packet.destination_channel).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port.clone(),
            &src_channel.clone(),
            &chan_end_on_a,
        )
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, &conn_id_on_a.clone(), &conn_end_on_a)
        .unwrap();
    contract
        .store_next_sequence_send(
            &mut deps.storage,
            &src_port,
            &src_channel,
            &Sequence::from(1),
        )
        .unwrap();

    let client_state = ClientState {
        latest_height: 10,
        ..get_dummy_client_state()
    };

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &env,
            &IbcClientId::default(),
            client,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = RawHeight {
        revision_number: 0,
        revision_height: 10,
    }
    .try_into()
    .unwrap();
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let info = mock_info("moduleaddress", &[]);
    contract
        .bind_port(deps.as_mut().storage, &src_port, info.sender.clone())
        .unwrap();
    let res = contract.send_packet(deps.as_mut(), &mock_env(), info, packet);
    println!("{:?}", res);
    assert!(res.is_ok());
    let res = res.unwrap();
    assert_eq!(res.attributes[0].value, "send_packet");
    assert_eq!(res.events[0].ty, IbcEventType::SendPacket.as_str());

    let packet_heights = contract
        .ibc_store()
        .get_packet_heights(deps.as_ref().storage, &src_port, &src_channel, 0, 10)
        .unwrap();
    println!("{packet_heights:?}");
    let height = packet_heights.get(&1).cloned().unwrap();
    assert_eq!(height, 12345);
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_packet_send_fail_channel_not_found() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());

    packet.sequence = 1;
    packet.data = vec![0];
    let info = mock_info("moduleaddress", &[]);
    contract
        .bind_port(
            deps.as_mut().storage,
            &to_ibc_port_id(&packet.source_port).unwrap(),
            info.sender.clone(),
        )
        .unwrap();
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}

#[test]
#[should_panic(
    expected = "Std(NotFound { kind: \"common::ibc::core::ics04_channel::packet::Sequence\" })"
)]
fn test_packet_send_fail_misiing_sequense() {
    let env = get_mock_env();
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timestamp_future = Timestamp::default();
    let timeout_height_future = 10;
    let mut packet = get_dummy_raw_packet(timeout_height_future, timestamp_future.nanoseconds());
    packet.sequence = 1;
    packet.data = vec![0];

    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let chan_end_on_a = ChannelEnd::new(
        State::TryOpen,
        Order::default(),
        Counterparty::new(dst_port, Some(dst_channel)),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );

    let conn_prefix = common::ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
        "hello".to_string().as_bytes().to_vec(),
    );

    let conn_end_on_a = ConnectionEnd::new(
        ConnectionState::Open,
        ClientId::default(),
        ConnectionCounterparty::new(
            ClientId::default(),
            Some(ConnectionId::default()),
            conn_prefix.unwrap(),
        ),
        get_compatible_versions(),
        ZERO_DURATION,
    );

    contract
        .store_channel_end(&mut deps.storage, &src_port, &src_channel, &chan_end_on_a)
        .unwrap();
    let conn_id_on_a = &chan_end_on_a.connection_hops()[0];
    contract
        .store_connection(&mut deps.storage, &conn_id_on_a.clone(), &conn_end_on_a)
        .unwrap();

    let client_state = ClientState {
        latest_height: 10,
        ..get_dummy_client_state()
    };

    let client = client_state.to_any().encode_to_vec();
    contract
        .store_client_state(
            &mut deps.storage,
            &env,
            &IbcClientId::default(),
            client,
            client_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3, 4],
    }
    .try_into()
    .unwrap();
    let height = RawHeight {
        revision_number: 0,
        revision_height: 10,
    }
    .try_into()
    .unwrap();
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let info = mock_info("moduleaddress", &[]);
    contract
        .bind_port(
            deps.as_mut().storage,
            &to_ibc_port_id(&packet.source_port).unwrap(),
            info.sender.clone(),
        )
        .unwrap();
    contract
        .send_packet(deps.as_mut(), &mock_env(), info, packet)
        .unwrap();
}
