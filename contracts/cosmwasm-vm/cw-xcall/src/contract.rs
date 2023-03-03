use crate::{
    error::ContractError,
    events::{event_call_message, event_response_message, event_rollback_message},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{CwCallService, EXECUTE_CALL, EXECUTE_ROLLBACK},
    types::message::CallServiceMessage,
    types::response::CallServiceResponseType,
    types::{
        address::Address,
        message::CallServiceMessageType,
        request::CallServiceMessageRequest,
        response::{to_int, CallServiceMessageReponse},
    },
};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcPacket, MessageInfo, Reply, Response, StdResult,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-xcall";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

impl<'a> CwCallService<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        _msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        self.init(deps, info)
    }

    pub fn execute(
        &mut self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::SetAdmin { address } => self.add_admin(deps.storage, info, address),
            ExecuteMsg::SetProtocol { value } => {
                todo!()
            }
            ExecuteMsg::SetProtocolFeeHandler { address } => {
                self.set_protocol_feehandler(deps, env, info, address)
            }
            ExecuteMsg::SendCallMessage { to, data, rollback } => {
                self.send_packet(env, deps, info, to, data, rollback, 0)
            }
            ExecuteMsg::ExecuteCall { request_id } => self.execute_call(deps, info, request_id),
            ExecuteMsg::ExecuteRollback { sequence_no } => {
                self.execute_rollback(deps, info, sequence_no)
            }
        }
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::GetAdmin {} => to_binary(&self.query_admin(deps.storage).unwrap()),
            QueryMsg::GetProtocolFee {} => todo!(),
            QueryMsg::GetProtocolFeeHandler {} => {
                to_binary(&self.query_feehandler(deps.storage).unwrap())
            }
        }
    }

    pub fn reply(&self, deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
        match msg.id {
            EXECUTE_CALL => self.reply_message_sent(deps.as_ref(), env, msg),
            EXECUTE_ROLLBACK => self.reply_rollback(deps.as_ref(), msg),
            _ => Err(ContractError::Unauthorized {}),
        }
    }
}

impl<'a> CwCallService<'a> {
    pub fn receive_packet_data(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: IbcPacket,
    ) -> Result<Response, ContractError> {
        self.ensure_owner(deps.storage, &info).unwrap();

        let call_service_message: CallServiceMessage = message.data.try_into()?;

        match call_service_message.message_type() {
            CallServiceMessageType::CallServiceRequest => self.hanadle_request(
                deps,
                info.sender.to_string(),
                call_service_message.payload(),
            ),
            CallServiceMessageType::CallServiceResponse => {
                self.handle_response(deps, call_service_message.payload())
            }
        }
    }

    fn hanadle_request(
        &self,
        deps: DepsMut,
        from: String,
        data: &[u8],
    ) -> Result<Response, ContractError> {
        let request_id = self.increment_last_request_id(deps.storage)?;
        let message_request: CallServiceMessageRequest = data.try_into()?;

        let from = Address::from(&from);
        let to = message_request.to();
        
        let request = CallServiceMessageRequest::new(
            from.clone(),
            to.to_string(),
            message_request.sequence_no(),
            message_request.rollback().into(),
            message_request.data().into(),
        );

        self.insert_request(deps.storage, request_id, request)?;

        let event = event_call_message(
            from.to_string(),
            to.to_string(),
            message_request.sequence_no(),
            request_id,
        );

        Ok(Response::new()
            .add_attribute("action", "call_service")
            .add_attribute("method", "handle_response")
            .add_event(event))
    }

    fn handle_response(&self, deps: DepsMut, data: &[u8]) -> Result<Response, ContractError> {
        let message: CallServiceMessageReponse = data.try_into()?;
        let response_sequence_no = message.sequence_no();

        let mut call_request = self.query_request(deps.storage, response_sequence_no)?;

        if call_request.is_null() {
            return Ok(Response::new()
                .add_attribute("action", "call_service")
                .add_attribute("method", "handle_response")
                .add_attribute(
                    "message",
                    format!("handle_resposne: no request for {}", response_sequence_no),
                ));
        }

        match message.response_code() {
            CallServiceResponseType::CallServiceResponseSucess => {
                let event = match message.message().is_empty() {
                    true => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        "",
                    ),
                    false => event_response_message(
                        response_sequence_no,
                        to_int(message.response_code()),
                        message.message(),
                    ),
                };
                self.cleanup_request(deps, response_sequence_no);
                Ok(Response::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .add_event(event))
            }
            _ => {
                self.ensure_rollback_length(call_request.rollback())
                    .unwrap();
                call_request.set_enabled();
                self.set_call_request(deps.storage, response_sequence_no, call_request)?;

                let event = event_rollback_message(response_sequence_no);

                Ok(Response::new()
                    .add_attribute("action", "call_service")
                    .add_attribute("method", "handle_response")
                    .add_event(event))
            }
        }
    }

    fn cleanup_request(&self, deps: DepsMut, sequence_no: u128) {
        self.remove_call_request(deps.storage, sequence_no);
    }

    fn init(&self, deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        let last_sequence_no = u128::default();
        let last_request_id = u128::default();
        let owner = Address::from(info.sender.as_str());

        self.add_owner(deps.storage, owner)?;
        self.init_last_sequnce_no(deps.storage, last_sequence_no)?;
        self.init_last_request_id(deps.storage, last_request_id)?;

        Ok(Response::new())
    }
}
