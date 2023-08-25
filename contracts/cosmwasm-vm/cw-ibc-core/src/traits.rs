use cw_common::raw_types::{
    channel::{
        RawMsgChannelCloseConfirm, RawMsgChannelCloseInit, RawMsgChannelOpenAck,
        RawMsgChannelOpenConfirm, RawMsgChannelOpenInit, RawMsgChannelOpenTry,
    },
    client::{
        RawMsgCreateClient, RawMsgSubmitMisbehaviour, RawMsgUpdateClient, RawMsgUpgradeClient,
    },
};

use super::*;

/// The `IbcClient` trait defines a set of functions that can be implemented by a module to interact
/// with an IBC client. These functions include creating, updating, and upgrading a client, registering
/// a client with a light client, generating a client identifier, submitting misbehaviour evidence, and
/// handling replies to these messages. By implementing these functions, a module can customize how it
/// interacts with IBC clients and handles client-related messages.
pub trait IbcClient {
    fn create_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env:Env,
        message: RawMsgCreateClient,
    ) -> Result<Response, ContractError>;

    fn update_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: RawMsgUpdateClient,
    ) -> Result<Response, ContractError>;
    fn execute_update_client_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn upgrade_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        env: Env,
        message: RawMsgUpgradeClient,
    ) -> Result<Response, ContractError>;
    fn execute_upgrade_client_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn register_client(
        &self,
        deps: DepsMut,
        client_type: IbcClientType,
        light_client: Addr,
    ) -> Result<Response, ContractError>;
    fn generate_client_identifier(
        &self,
        store: &mut dyn Storage,
        client_type: IbcClientType,
    ) -> Result<ClientId, ContractError>;

    fn misbehaviour(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: RawMsgSubmitMisbehaviour,
    ) -> Result<Response, ContractError>;

    fn execute_misbehaviour_reply(
        &self,
        deps: DepsMut,
        env: Env,
        message: Reply,
    ) -> Result<Response, ContractError>;
}

/// The `ValidateChannel` trait defines a set of functions that can be implemented by a module to
/// validate channel-related messages. These functions take in the necessary dependencies, message
/// information, and the specific message being validated, and return a response or an error. The
/// specific functions defined in this trait correspond to different stages of the channel opening and
/// closing process, such as `validate_channel_open_init` for validating a `MsgChannelOpenInit` message.
/// By implementing these functions, a module can customize how it validates channel-related messages
/// and interactions with other modules on different chains.
pub trait ValidateChannel {
    // channel_open_init is called by a module to initiate a channel opening handshake with a module on another chain.
    fn validate_channel_open_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelOpenInit,
    ) -> Result<Response, ContractError>;

    // channel_open_try is called by a module to accept the first step of a channel opening handshake initiated by a module on another chain.
    fn validate_channel_open_try(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelOpenTry,
    ) -> Result<Response, ContractError>;

    // channel_open_ack is called by the handshake-originating module to
    // acknowledge the acceptance of the initial request by the counterparty module on the other chain.
    fn validate_channel_open_ack(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelOpenAck,
    ) -> Result<Response, ContractError>;

    // channel_open_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_open_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelOpenConfirm,
    ) -> Result<Response, ContractError>;

    // channel_close_init is called by either module to close their end of the
    // channel. Once closed, channels cannot be reopened.
    fn validate_channel_close_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelCloseInit,
    ) -> Result<Response, ContractError>;

    // channel_close_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_close_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &RawMsgChannelCloseConfirm,
    ) -> Result<Response, ContractError>;
}

/// The `ExecuteChannel` trait defines a set of functions that can be implemented by a module to handle
/// the execution of channel-related messages. These functions take in the necessary dependencies and
/// message information, and return a response or an error. The specific functions defined in this trait
/// correspond to different stages of the channel opening and closing process, such as
/// `execute_channel_open_init` for handling the execution of a `MsgChannelOpenInit` message. By
/// implementing these functions, a module can customize how it handles channel-related messages and
/// interactions with other modules on different chains.
pub trait ExecuteChannel {
    fn execute_channel_open_init(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;

    fn execute_channel_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;

    fn execute_channel_close_init(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn execute_channel_open_confirm(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;

    fn execute_channel_open_ack(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;

    fn execute_channel_close_confirm(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
}
