use cosmwasm_std::Reply;

use super::*;

pub trait IbcClient {
    fn create_client(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        message: MsgCreateClient,
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
        message: MsgUpdateClient,
    ) -> Result<Response, ContractError>;
    fn execute_update_client_reply(
        &self,
        deps: DepsMut,
        message: Reply,
    ) -> Result<Response, ContractError>;
    fn upgrade_client(&self, deps: DepsMut, message: MsgUpgradeClient);
    fn register_client(&self, deps: DepsMut, client_type: ClientType, light_client: Addr);
    fn generate_client_identifier(
        &self,
        deps: DepsMut,
        client_type: ClientType,
    ) -> Result<ClientId, ContractError>;
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
        message: &MsgChannelOpenAck,
    ) -> Result<Response, ContractError>;

    // channel_open_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_open_confirm(
        &self,
        deps: DepsMut,
        message: &MsgChannelOpenConfirm,
    ) -> Result<Response, ContractError>;

    // channel_close_init is called by either module to close their end of the
    // channel. Once closed, channels cannot be reopened.
    fn validate_channel_close_init(
        &self,
        deps: DepsMut,
        message: &MsgChannelCloseInit,
    ) -> Result<Response, ContractError>;

    // channel_close_confirm is called by the counterparty module to close their
    // end of the channel, since the other end has been closed.
    fn validate_channel_close_confirm(
        &self,
        deps: DepsMut,
        message: &MsgChannelCloseConfirm,
    ) -> Result<Response, ContractError>;
}

pub trait ExecuteChannel {
    fn execute_channel_open_init(
        &self,
        deps: DepsMut,
        message: Reply,
        // message: &MsgChannelOpenInit,
    ) -> Result<Response, ContractError>;

    fn execute_channel_open_try(
        &self,
        deps: DepsMut,
        message: Reply,
        // message: &MsgChannelOpenInit,
    ) -> Result<Response, ContractError>;
}
