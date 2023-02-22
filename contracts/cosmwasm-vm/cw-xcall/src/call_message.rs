use cosmwasm_std::{
    ensure, to_binary, Addr, Binary, Deps, DepsMut, Env, Event, IbcMsg, MessageInfo, Response,
};
use cosmwasm_std::{IbcTimeout, IbcTimeoutBlock};

use crate::error::ContractError;
use crate::state::CwCallservice;
use crate::types::address::Address;
use crate::types::message::CallServiceMessage;
use crate::types::message::CallServiceMessageType;
use crate::types::request::CallServiceMessageRequest;

impl<'a> CwCallservice<'a> {
    pub fn send_packet(
        &self,
        env: Env,
        deps: DepsMut,
        info: MessageInfo,
        to: String,
        data: Binary,
        rollback: Binary,
        time_out_height: u64,
    ) -> Result<Response, ContractError> {
        self.ensure_caller_is_contract_and_rollback_is_null(
            deps.as_ref(),
            info.sender.clone(),
            &rollback.0,
        )?;

        self.ensure_data_length(data.len())?;

        self.ensure_rollback_length(&rollback.0)?;

        let (packet, event) = self.create_packet_and_event_for_request(
            deps,
            env,
            info.clone(),
            to,
            rollback,
            time_out_height,
            data.clone(),
        )?;

        Ok(Response::new()
            .add_message(packet)
            .add_attribute("action", "xcall-service")
            .add_attribute("method", "send_packt")
            .add_event(event))
    }

    pub fn create_packet_and_event_for_request(
        &self,
        deps: DepsMut,
        env: Env,

        info: MessageInfo,
        to: String,
        rollback: Binary,
        time_out_height: u64,
        data: Binary,
    ) -> Result<(IbcMsg, Event), ContractError> {
        let ibc_config = self
            .ibc_config()
            .may_load(deps.as_ref().storage)
            .unwrap()
            .unwrap();
        let seq_no = self.increment_last_sequence_no(deps).unwrap();

        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));

        let call_request = CallServiceMessageRequest::new(
            Address::from(info.sender.as_str()),
            to,
            seq_no,
            rollback,
            data,
        );

        let message = CallServiceMessage::new(
            CallServiceMessageType::CallServiceRequest,
            to_binary(&call_request).unwrap(),
        );

        let event = event_xcall_message_sent(seq_no, info.sender.to_string(), 0, &message);

        Ok((
            IbcMsg::SendPacket {
                channel_id: ibc_config.dst_endpoint().channel_id.clone(),
                data: to_binary(&message).unwrap(),
                timeout,
            },
            event,
        ))
    }
}

fn event_xcall_message_sent(
    sequence_no: u128,
    from: String,
    req_id: u128,
    data: &CallServiceMessage,
) -> Event {
    let event = Event::new("xcall_message_sent");

    event
        .add_attribute("from", from)
        .add_attribute("sequence_no", sequence_no.to_string())
        .add_attribute("req_id", req_id.to_string())
        .add_attribute("data", to_binary(data).unwrap().to_string())
}

// #[test]
// fn test() {
//     let mut mock_deps = mock_dependencies();

//     let contrat = CwCallservice::default();
//     let env = mock_env();

//     let src = IbcEndpoint {
//         port_id: "our-port".to_string(),
//         channel_id: "channel-1".to_string(),
//     };

//     let dst = IbcEndpoint {
//         port_id: "their-port".to_string(),
//         channel_id: "channel-3".to_string(),
//     };

//     let ibc_config = IbcConfig::new(src, dst);

//     contrat
//         .ibc_config()
//         .save(mock_deps.as_mut().storage, &ibc_config)
//         .unwrap();

//     let ibc_msg = contrat
//         .send_call_message(env.clone(), mock_deps.as_ref(), 10, &vec![1, 2, 3])
//         .unwrap();

//     println!("{:?}", ibc_msg);

//     let msgs = mock_ibc_packet_recv("channel-1", &vec![1, 2, 3]).unwrap();

//     let r = ibc_packet_receive(mock_deps.as_mut(), env, msgs);
// }
