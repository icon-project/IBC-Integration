use common::ibc::core::ics03_connection::connection::Counterparty as ConnectionCounterparty;
use common::ibc::core::ics03_connection::connection::State as ConnectionState;

use common::ibc::timestamp::Timestamp;
use cw_common::raw_types::channel::RawMsgRecvPacket;
use cw_common::types::Ack;
use cw_ibc_core::conversions::to_ibc_channel_id;
use cw_ibc_core::conversions::to_ibc_height;
use cw_ibc_core::conversions::to_ibc_port_id;
use cw_ibc_core::light_client::light_client::LightClient;
use cw_ibc_core::VALIDATE_ON_PACKET_RECEIVE_ON_MODULE;

use super::*;

pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());

    to_binary(&res).unwrap()
}

pub fn get_dummy_raw_packet_recv(timeout_height: u64, timeout_timestamp: u64) -> RawPacket {
    let (src, dest) = get_dummy_endpoints();
    RawPacket {
        sequence: 1,
        source_port: src.port_id,
        source_channel: src.channel_id,
        destination_port: dest.port_id,
        destination_channel: dest.channel_id,
        data: vec![0],
        timeout_height: Some(RawHeight {
            revision_number: 12,
            revision_height: timeout_height,
        }),
        timeout_timestamp,
    }
}

pub fn get_dummy_raw_msg_recv_packet(height: u64) -> RawMsgRecvPacket {
    let timestamp = Timestamp::default();
    RawMsgRecvPacket {
        packet: Some(get_dummy_raw_packet_recv(height, timestamp.nanoseconds())),
        proof_commitment: get_dummy_proof(),
        proof_height: Some(RawHeight {
            revision_number: 0,
            revision_height: height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

#[test]
fn test_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let env = get_mock_env();
    let info = create_mock_info("channel-creater", "umlg", 2000000000);

    let msg = get_dummy_raw_msg_recv_packet(12);
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

    let proof_height = to_ibc_height(msg.proof_height.clone()).unwrap();
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
    let res = contract.validate_receive_packet(deps.as_mut(), info, env, &msg);
    let missing_receipts = contract
    .ibc_store()
    .get_missing_packet_receipts(
        deps.as_ref().storage,
        &IbcPortId::from_str(&packet.destination_port).unwrap(),
        &IbcChannelId::from_str(&packet.destination_channel).unwrap(),
        0,
        10,
    )
    .unwrap();
println!("{missing_receipts:?}");
assert!(!missing_receipts.contains(&packet.sequence));
    println!("{:?}", res);
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0].id,
        VALIDATE_ON_PACKET_RECEIVE_ON_MODULE
    );
}

#[test]
fn execute_receive_packet() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let (src, dst) = get_dummy_endpoints();

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 0, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &packet,
        )
        .unwrap();

    let result = SubMsgResponse {
        data: None,
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Unordered,
        Counterparty::new(
            IbcPortId::from_str(&packet.src.port_id).unwrap(),
            Some(IbcChannelId::from_str(&packet.src.channel_id).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &chan_end_on_b,
        )
        .unwrap();
   

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
   assert!(res.is_ok());

  
}

#[test]
fn execute_receive_packet_ordered() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 10,
    };
    let timeout = IbcTimeout::with_both(timeout_block, cosmwasm_std::Timestamp::from_nanos(100));
    let (src, dst) = get_dummy_endpoints();

    let packet = IbcPacket::new(vec![0, 1, 2, 3], src, dst, 1, timeout);
    contract
        .store_callback_data(
            deps.as_mut().storage,
            VALIDATE_ON_PACKET_RECEIVE_ON_MODULE,
            &packet,
        )
        .unwrap();

    let result = SubMsgResponse {
        data: None,
        events: vec![],
    };
    let result: SubMsgResult = SubMsgResult::Ok(result);
    let reply = Reply { id: 0, result };

    let chan_end_on_b = ChannelEnd::new(
        State::Open,
        Order::Ordered,
        Counterparty::new(
            IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            Some(IbcChannelId::from_str(&packet.dest.channel_id).unwrap()),
        ),
        vec![IbcConnectionId::default()],
        Version::new("ics20-1".to_string()),
    );
    contract
        .store_channel_end(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &chan_end_on_b,
        )
        .unwrap();
    contract
        .store_next_sequence_recv(
            &mut deps.storage,
            &IbcPortId::from_str(&packet.dest.port_id).unwrap(),
            &IbcChannelId::from_str(&packet.dest.channel_id).unwrap(),
            &Sequence::from(1),
        )
        .unwrap();

    let res = contract.execute_receive_packet(deps.as_mut(), reply);
    
    assert!(res.is_ok());
  
}

#[test]
#[should_panic(expected = "ChannelNotFound")]
fn test_receive_packet_fail_missing_channel() {
    let contract = CwIbcCoreContext::default();
    let mut deps = deps();
    let info = create_mock_info("channel-creater", "umlg", 2000);
    let msg = get_dummy_raw_msg_recv_packet(12);
    let env = get_mock_env();

    contract
        .validate_receive_packet(deps.as_mut(), info, env, &msg)
        .unwrap();
}

#[test]
fn test_lookup_module_packet() {
    let mut deps = deps();
    let ctx = CwIbcCoreContext::default();
    let module_id =
        common::ibc::core::ics26_routing::context::ModuleId::from_str("contractaddress").unwrap();
    let msg = get_dummy_raw_msg_recv_packet(12);
    let port_id = to_ibc_port_id(&msg.packet.unwrap().source_port).unwrap();
    ctx.claim_capability(
        &mut deps.storage,
        port_id.as_bytes().to_vec(),
        module_id.to_string(),
    )
    .unwrap();
    let res = ctx.lookup_modules(&mut deps.storage, port_id.to_string().as_bytes().to_vec());

    assert!(res.is_ok());
    assert_eq!("contractaddress", res.unwrap())
}
