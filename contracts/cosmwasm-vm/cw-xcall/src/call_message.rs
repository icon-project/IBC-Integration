use cosmwasm_std::{
    to_binary, DepsMut, Env, IbcMsg, IbcTimeout, IbcTimeoutBlock, MessageInfo, Response,
};

use crate::{
    error::ContractError,
    events::event_xcall_message_sent,
    state::CwCallService,
    types::{
        address::Address, call_request::CallRequest, message::CallServiceMessage,
        request::CallServiceMessageRequest,
    },
};

impl<'a> CwCallService<'a> {
    pub fn send_packet(
        &self,
        env: Env,
        deps: DepsMut,
        info: MessageInfo,
        to: String,
        data: Vec<u8>,
        rollback: Vec<u8>,
        time_out_height: u64,
    ) -> Result<Response, ContractError> {
        let from_address = info.sender.to_string();
        self.ensure_caller_is_contract_and_rollback_is_null(
            deps.as_ref(),
            info.sender.clone(),
            &rollback,
        )?;

        self.ensure_data_length(data.len())?;
        self.ensure_rollback_length(&rollback)?;

        // TODO : ADD fee logic

        let need_response = !rollback.is_empty();
        let sequence_no = self.increment_last_sequence_no(deps.storage)?;

        if need_response {
            let request = CallRequest::new(
                Address::from(&from_address),
                to.clone(),
                rollback.clone(),
                need_response,
            );

            self.set_call_request(deps.storage, sequence_no, request)?;
        }

        let call_request = CallServiceMessageRequest::new(
            Address::from(info.sender.as_str()),
            to,
            sequence_no,
            rollback.to_vec(),
            data.to_vec(),
        );

        let message: CallServiceMessage = call_request.into();
        let packet =
            self.create_packet_and_event_for_request(deps, env, time_out_height, message.clone())?;

        let event = event_xcall_message_sent(sequence_no, info.sender.to_string(), 0, &message);

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
        time_out_height: u64,
        message: CallServiceMessage,
    ) -> Result<IbcMsg, ContractError> {
        let ibc_config = self
            .ibc_config()
            .may_load(deps.as_ref().storage)
            .unwrap()
            .unwrap();

        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));

        Ok(IbcMsg::SendPacket {
            channel_id: ibc_config.dst_endpoint().channel_id.clone(),
            data: to_binary(&message).unwrap(),
            timeout,
        })
    }
}
