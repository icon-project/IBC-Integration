use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_ibc_packet_recv};
use cosmwasm_std::{
    to_binary, CosmosMsg, Deps, Empty, Env, Event, IbcMsg, IbcPacketReceiveMsg, Response,
};
use cosmwasm_std::{IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock};

use crate::error::ContractError;
use crate::state::{CwCallservice, IbcConfig};
use crate::types::message::CallServiceMessage;
use crate::types::message::CallServiceMessageType;

impl<'a> CwCallservice<'a> {
    pub fn send_call_message(
        &self,
        env: Env,
        deps: Deps,
        time_out_height: u64,
        data: &[u8],
    ) -> Result<Response, ContractError> {
        let ibc_config = self.ibc_config().may_load(deps.storage).unwrap().unwrap();
        let packet = self.send_packet(
            env,
            ibc_config.src_endpoint().clone(),
            ibc_config.next_sequence().unwrap().try_into().unwrap(),
            time_out_height,
            data,
        );

        // let event = event_xcall_message_sent(from, sequence_no, req_id, data);

        Ok(Response::new()
            .add_attribute("key", "valu")
            .add_message(packet))
    }

    fn send_packet(
        &self,
        env: Env,
        dst: IbcEndpoint,
        sequence: u64,
        time_out_height: u64,
        data: &[u8],
    ) -> IbcMsg {
        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));

        let data =
            CallServiceMessage::new(CallServiceMessageType::CallServiceRequest, data.to_vec());

        IbcMsg::SendPacket {
            channel_id: dst.channel_id,
            data: to_binary(&data).unwrap(),
            timeout,
        }
    }
}

pub fn event_xcall_message_sent(
    from: String,
    sequence_no: u128,
    req_id: u128,
    data: Vec<u8>,
) -> Event {
    let event = Event::new("xcall_message_sent");

    event
        .add_attribute("from", from)
        .add_attribute("sequence_no", sequence_no.to_string())
        .add_attribute("req_id", req_id.to_string())
        .add_attribute("data", to_binary(&data).unwrap().to_string())
}

#[test]
fn test() {
    let mut mock_deps = mock_dependencies();

    let contrat = CwCallservice::default();
    let env = mock_env();

    let src = IbcEndpoint {
        port_id: "our-port".to_string(),
        channel_id: "channel-1".to_string(),
    };

    let dst = IbcEndpoint {
        port_id: "their-port".to_string(),
        channel_id: "channel-3".to_string(),
    };

    let ibc_config = IbcConfig::new(src, dst);

    contrat
        .ibc_config()
        .save(mock_deps.as_mut().storage, &ibc_config)
        .unwrap();

    let ibc_msg = contrat
        .send_call_message(env.clone(), mock_deps.as_ref(), 10, &vec![1, 2, 3])
        .unwrap();

    println!("{:?}", ibc_msg);

    let msgs = mock_ibc_packet_recv("channel-1", &vec![1, 2, 3]).unwrap();

    let r = ibc_packet_receive(mock_deps.as_mut(), env, msgs);
}
