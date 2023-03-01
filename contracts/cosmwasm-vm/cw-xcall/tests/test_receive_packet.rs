mod account;
mod setup;

use account::*;
use cosmwasm_std::{
    testing::{mock_dependencies, mock_ibc_packet_recv, mock_info},
    IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock,
};
use cw_xcall::{
    state::CwCallservice,
    types::{address::Address, message::CallServiceMessage, request::CallServiceMessageRequest},
};
use setup::*;

#[test]
fn test_receive_packet_for_call_message_request() {
    let mut mock_deps = mock_dependencies();
    let mock_info = create_mock_info(&alice().to_string(), "umlg", 2000);

    let contract = CwCallservice::default();

    contract
        .add_owner(
            mock_deps.as_mut().storage,
            Address::from(&mock_info.sender.to_string()),
        )
        .unwrap();

    let data = CallServiceMessageRequest::new(
        Address::from(mock_info.sender.as_str()),
        alice().to_string(),
        1,
        vec![],
        vec![1, 2, 3],
    );

    let message: CallServiceMessage = data.try_into().unwrap();

    let timeout_block = IbcTimeoutBlock {
        revision: 0,
        height: 0,
    };
    let timeout = IbcTimeout::with_block(timeout_block);
    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };
    let packet = IbcPacket::new(message, src, dst, 0, timeout);

    let result = contract
        .receive_packet_data(mock_deps.as_mut(), mock_info, packet)
        .unwrap();

    println!("{:?}", result)
}
