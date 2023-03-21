use super::*;

pub trait IbcClient {
    fn create_client(&self, deps: DepsMut, message: MsgCreateClient) -> IbcClientId;
    fn update_client(&self, deps: DepsMut, message: MsgUpdateClient);
    fn upgrade_client(&self, deps: DepsMut, message: MsgUpgradeClient);
    fn register_client(&self, deps: DepsMut, client_type: ClientType, light_client: Addr);
    fn generate_client_identifier(&self, deps: Deps, client_type: ClientType) -> String;
}
