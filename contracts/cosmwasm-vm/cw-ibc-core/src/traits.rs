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
    ) -> Result<IbcClientId, ContractError>;
    fn update_client(&self, deps: DepsMut, message: MsgUpdateClient);
    fn upgrade_client(&self, deps: DepsMut, message: MsgUpgradeClient);
    fn register_client(&self, deps: DepsMut, client_type: ClientType, light_client: Addr);
    fn generate_client_identifier(
        &self,
        deps: DepsMut,
        client_type: ClientType,
    ) -> Result<ClientId, ContractError>;
}
