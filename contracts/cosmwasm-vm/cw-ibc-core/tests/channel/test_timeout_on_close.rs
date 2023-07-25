use cw_ibc_core::{
    conversions::{to_ibc_channel_id, to_ibc_timeout_height, to_ibc_timestamp},
    light_client::light_client::LightClient,
    VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE,
};

use super::*;

#[test]
fn test_timeout_on_close_packet_validate_to_light_client() {
    use common::ibc::core::ics03_connection::connection::Counterparty as ConnectionCounterparty;
    use common::ibc::core::ics03_connection::connection::State as ConnectionState;

    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 20000000);

    let height = 2;
    let timeout_timestamp = 5;
    let msg = get_dummy_raw_msg_timeout_on_close(height, timeout_timestamp);
    let packet = &msg.packet.clone().unwrap();
    let src_port = to_ibc_port_id(&packet.source_port).unwrap();
    let src_channel = to_ibc_channel_id(&packet.source_channel).unwrap();

    let dst_port = to_ibc_port_id(&packet.destination_port).unwrap();
    let dst_channel = to_ibc_channel_id(&packet.destination_channel).unwrap();

    let packet_timeout_height = to_ibc_timeout_height(packet.timeout_height.clone()).unwrap();
    let packet_timestamp = to_ibc_timestamp(packet.timeout_timestamp).unwrap();

    let packet_sequence = Sequence::from(packet.sequence);
    let proof_height = to_ibc_height(msg.proof_height.clone().unwrap()).unwrap();

    let chan_end_on_a_ordered = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(dst_port.clone(), Some(dst_channel.clone())),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &src_port.clone(),
            &src_channel.clone(),
            &chan_end_on_a_ordered,
        )
        .unwrap();
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
        .store_connection(
            &mut deps.storage,
            chan_end_on_a_ordered.connection_hops()[0].clone(),
            conn_end_on_a,
        )
        .unwrap();
    let packet_commitment =
        compute_packet_commitment(&packet.data, &packet_timeout_height, &packet_timestamp);

    contract
        .store_packet_commitment(
            &mut deps.storage,
            &src_port,
            &src_channel,
            packet_sequence,
            packet_commitment,
        )
        .unwrap();

    let client_state: ClientState = get_dummy_client_state();

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
    let _client_type = IbcClientType::new("iconclient".to_string());

    let light_client = LightClient::new("lightclient".to_string());

    contract
        .bind_port(&mut deps.storage, &src_port, "moduleaddress".to_string())
        .unwrap();

    contract
        .store_client_implementations(&mut deps.storage, IbcClientId::default(), light_client)
        .unwrap();
    mock_lightclient_reply(&mut deps);
    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: vec![1, 2, 3, 4],
        next_proof_context_hash: vec![1, 2, 3],
    }
    .try_into()
    .unwrap();
    //let height = msg.proof_height_on_b;
    let consenus_state_any = consenus_state.to_any().encode_to_vec();
    contract
        .store_consensus_state(
            &mut deps.storage,
            &IbcClientId::default(),
            proof_height,
            consenus_state_any,
            consenus_state.get_keccak_hash().to_vec(),
        )
        .unwrap();
    let env = get_mock_env();
    contract
        .ibc_store()
        .expected_time_per_block()
        .save(deps.as_mut().storage, &(env.block.time.seconds()))
        .unwrap();

    let res =
        contract.timeout_on_close_packet_validate_to_light_client(deps.as_mut(), info, env, msg);
    print!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_TIMEOUT_ON_MODULE
    )
}
