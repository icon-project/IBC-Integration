use cosmwasm_std::Addr;

use super::*;

impl<'a> IbcClient for CwIbcCoreContext<'a> {
    fn create_client(&self, deps: DepsMut, message: MsgCreateClient) -> IbcClientId {
        todo!()
    }

    fn update_client(
        &self,
        deps: DepsMut,
        message: ibc::core::ics02_client::msgs::update_client::MsgUpdateClient,
    ) {
        todo!()
    }

    fn upgrade_client(
        &self,
        deps: DepsMut,
        message: ibc::core::ics02_client::msgs::upgrade_client::MsgUpgradeClient,
    ) {
        todo!()
    }

    fn register_client(&self, deps: DepsMut, client_type: ClientType, light_client: Addr) {
        todo!()
    }

    fn generate_client_identifier(&self, deps: Deps, client_type: ClientType) -> String {
        todo!()
    }
}
