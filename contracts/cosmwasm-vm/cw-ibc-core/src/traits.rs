use ibc::core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour;

use super::*;

pub trait IbcClient {
    fn create_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: IbcMsgCreateClient,
    ) -> Result<Response, ContractError>;

    fn execute_create_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn update_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: IbcMsgUpdateClient,
    ) -> Result<Response, ContractError>;
    fn execute_update_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn upgrade_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgUpgradeClient,
    ) -> Result<Response, ContractError>;
    fn execute_upgrade_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn register_client(
        &self,
        deps: DepsMut,
        client_type: ClientType,
        light_client: Addr,
    ) -> Result<Response, ContractError>;
    fn generate_client_identifier(
        &self,
        store: &mut dyn Storage,
        client_type: ClientType,
    ) -> Result<ClientId, ContractError>;

    fn misbehaviour(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgSubmitMisbehaviour,
    ) -> Result<Response, ContractError>;

    fn execute_misbehaviour_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
}

pub trait ValidateChannel {
    // channel_open_init is called by a module to initiate a channel opening handshake with a module on another chain.
    fn validate_channel_open_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenInit,
    ) -> Result<Response, ContractError>;

    // channel_open_try is called by a module to accept the first step of a channel opening handshake initiated by a module on another chain.
    fn validate_channel_open_try(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenTry,
    ) -> Result<Response, ContractError>;

    // channel_open_ack is called by the handshake-originating module to
    // acknowledge the acceptance of the initial request by the counterparty module on the other chain.
    fn validate_channel_open_ack(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenAck,
    ) -> Result<Response, ContractError>;

    // channel_open_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_open_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelOpenConfirm,
    ) -> Result<Response, ContractError>;

    // channel_close_init is called by either module to close their end of the
    // channel. Once closed, channels cannot be reopened.
    fn validate_channel_close_init(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelCloseInit,
    ) -> Result<Response, ContractError>;

    // channel_close_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_close_confirm(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: &MsgChannelCloseConfirm,
    ) -> Result<Response, ContractError>;
}

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
