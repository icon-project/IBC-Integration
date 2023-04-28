pub mod ack;
pub mod admin;
pub mod assertion;
pub mod call_message;
pub mod check;
pub mod contract;
pub mod error;
pub mod events;
pub mod fee;
pub mod fee_handler;
pub mod handle_call_message;
pub mod helpers;
pub mod ibc;
pub mod msg;
pub mod owner;
pub mod requests;
pub mod state;
pub mod types;
use crate::ack::{on_ack_failure, on_ack_sucess};
use crate::{
    ack::{make_ack_fail, make_ack_success},
    check::{check_order, check_version},
    error::ContractError,
    events::{
        event_call_executed, event_call_message, event_response_message, event_rollback_executed,
        event_rollback_message, event_xcall_message_sent,
    },
    ibc::{APP_ORDER, IBC_VERSION},
    msg::{InstantiateMsg, QueryMsg},
    state::{
        CwCallService, IbcConfig, ACK_FAILURE_ID, EXECUTE_CALL_ID, EXECUTE_ROLLBACK_ID,
        SEND_CALL_MESSAGE_REPLY_ID,
    },
    types::{
        call_request::CallRequest,
        message::{CallServiceMessage, CallServiceMessageType},
        request::CallServiceMessageRequest,
        response::{to_int, CallServiceMessageResponse, CallServiceResponseType},
        storage_keys::StorageKey,
    },
};
use common::types::message::CrossContractMessage::XCallMessage;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    attr, ensure, ensure_eq, entry_point, from_binary, to_binary, Addr, Api, Binary, Coin,
    CosmosMsg, Deps, DepsMut, Empty, Env, Event, Ibc3ChannelOpenResponse, IbcBasicResponse,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse,
    IbcEndpoint, IbcMsg, IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, MessageInfo, Never, QuerierWrapper, Reply,
    Response, StdError, StdResult, Storage, SubMsg, SubMsgResult, WasmMsg,
};
use cosmwasm_std::{to_vec, QueryRequest};
use cw2::set_contract_version;
use cw_common::types::{Ack, Address};
use cw_common::xcall_msg::ExecuteMsg;
use cw_common::Height;
use cw_common::ProstMessage;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use schemars::_serde_json::to_string;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let call_service = CwCallService::default();

    call_service.instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let mut call_service = CwCallService::default();

    call_service.execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let call_service = CwCallService::default();

    call_service.query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let call_service = CwCallService::default();

    call_service.reply(deps, env, msg)
}
