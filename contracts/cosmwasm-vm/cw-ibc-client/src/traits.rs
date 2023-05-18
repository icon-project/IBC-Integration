use ibc::core::ics02_client::msgs::misbehaviour::MsgSubmitMisbehaviour;

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
