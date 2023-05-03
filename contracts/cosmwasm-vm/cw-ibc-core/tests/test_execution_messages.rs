pub mod setup;

use common::icon::icon::types::v1::BtpHeader as RawBtpHeader;
use common::icon::icon::types::v1::MerkleNode as RawMerkleNode;
use common::icon::icon::types::v1::SignedHeader as RawSignedHeader;
use cosmwasm_std::{testing::mock_env, to_binary, to_vec, Addr, Event, Reply, SubMsgResponse};
use cw_common::client_response::{CreateClientResponse, UpdateClientResponse};

use cw_common::hex_string::HexString;
use cw_ibc_core::ics02_client::types::SignedHeader;
use cw_ibc_core::{
    context::CwIbcCoreContext,
    ics02_client::types::{ClientState, ConsensusState},
    msg::InstantiateMsg,
};

use cw_common::core_msg::ExecuteMsg as CoreExecuteMsg;
use prost::Message;

use setup::*;
use common::icon::icon::lightclient::v1::ClientState as RawClientState;
use common::icon::icon::lightclient::v1::ConsensusState as RawConsensusState;

#[test]
fn test_for_create_client_execution_message() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();

    let client_state: RawClientState = RawClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: RawConsensusState = RawConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            CoreExecuteMsg::RegisterClient {
                client_type: "iconclient".to_string(),
                client_address: Addr::unchecked("lightclientaddress"),
            },
        )
        .unwrap();

    assert_eq!(response.attributes[0].value, "register_client");

    let create_client_message = CoreExecuteMsg::CreateClient {
        client_state: HexString::from_bytes(&client_state.clone().encode_to_vec()),
        consensus_state: HexString::from_bytes(&consenus_state.clone().encode_to_vec()),
        signer: HexString::from_bytes("raw_message".as_bytes()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, create_client_message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "create_client");

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
        to_vec(&client_state).unwrap(),
        consenus_state.encode_to_vec(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };
    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_create_client_reply")
}

#[test]
fn test_for_update_client_execution_messages() {
    let mut deps = deps();
    let info = create_mock_info("alice", "umlg", 2000);

    let mut contract = CwIbcCoreContext::default();
    let env = mock_env();
    let response = contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    assert_eq!(response.attributes[0].value, "instantiate");
    let response = contract
        .execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            CoreExecuteMsg::RegisterClient {
                client_type: "iconclient".to_string(),
                client_address: Addr::unchecked("lightclientaddress"),
            },
        )
        .unwrap();
    assert_eq!(response.attributes[0].value, "register_client");

    let client_state: ClientState = common::icon::icon::lightclient::v1::ClientState {
        trusting_period: 2,
        frozen_height: 0,
        max_clock_drift: 5,
        latest_height: 100,
        network_section_hash: vec![1, 2, 3],
        validators: vec!["hash".as_bytes().to_vec()],
    }
    .try_into()
    .unwrap();

    let consenus_state: ConsensusState = common::icon::icon::lightclient::v1::ConsensusState {
        message_root: "message_root".as_bytes().to_vec(),
    }
    .try_into()
    .unwrap();

    contract
        .instantiate(deps.as_mut(), env.clone(), info.clone(), InstantiateMsg {})
        .unwrap();

    let mock_reponse_data = CreateClientResponse::new(
        "iconclient".to_string(),
        "10-15".to_string(),
        to_vec(&client_state).unwrap(),
        consenus_state.clone().try_into().unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 21,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    contract
        .reply(deps.as_mut(), env.clone(), reply_message)
        .unwrap();

    let merkle_node = RawMerkleNode {
        dir: 0,
        value: vec![0, 1, 2],
    };

    let btp_header = RawBtpHeader {
        main_height: 27,
        round: 0,
        next_proof_context_hash: hex::decode(
            "d090304264eeee3c3562152f2dc355601b0b423a948824fd0a012c11c3fc2fb4",
        )
        .unwrap(),
        network_section_to_root: vec![merkle_node],
        network_id: 1,
        update_number: 0,
        prev_network_section_hash: hex::decode(
            "b791b4b069c561ca31093f825f083f6cc3c8e5ad5135625becd2ff77a8ccfa1e",
        )
        .unwrap(),
        message_count: 1,
        message_root: hex::decode(
            "7702db70e830e07b4ff46313456fc86d677c7eeca0c011d7e7dcdd48d5aacfe2",
        )
        .unwrap(),
        next_validators: vec![hex::decode("00b040bff300eee91f7665ac8dcf89eb0871015306").unwrap()],
    };

    let signed_header: RawSignedHeader = RawSignedHeader {
        header: Some(btp_header),
        signatures: vec![hex::decode("6c8b2bc2c3d31e34bd4ed9db6eff7d5dc647b13c58ae77d54e0b05141cb7a7995102587f1fa33fd56815463c6b78e100217c29ddca20fcace80510e3dab03a1600").unwrap()],
    }
    .try_into()
    .unwrap();

    let message = CoreExecuteMsg::UpdateClient {
        client_id: "iconclient-0".to_string(),
        header: HexString::from_bytes(&signed_header.encode_to_vec()),
        signer: HexString::from_bytes("signeraddress".to_string().as_bytes()),
    };

    let response = contract
        .execute(deps.as_mut(), env.clone(), info, message)
        .unwrap();

    assert_eq!(response.attributes[0].value, "update_client");

    let mock_reponse_data = UpdateClientResponse::new(
        "10-15".to_string(),
        "iconclient-0".to_string(),
        to_vec(&client_state).unwrap(),
        to_vec(&consenus_state).unwrap(),
    );

    let mock_data_binary = to_binary(&mock_reponse_data).unwrap();

    let event = Event::new("empty");

    let reply_message = Reply {
        id: 22,
        result: cosmwasm_std::SubMsgResult::Ok(SubMsgResponse {
            events: vec![event],
            data: Some(mock_data_binary),
        }),
    };

    let response = contract.reply(deps.as_mut(), env, reply_message).unwrap();

    assert_eq!(response.attributes[0].value, "execute_update_client_reply")
}
