use cw_storage_plus::KeyDeserialize;

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
        let light_client_address = light_client.to_string();

        self.check_client_registered(deps.as_ref().storage, client_type.clone())
            .unwrap();

        self.store_client_into_registry(deps.storage, client_type, light_client_address)
            .unwrap();
    }

    fn generate_client_identifier(
        &self,
        deps: DepsMut,
        client_type: ClientType,
    ) -> Result<ClientId, ContractError> {
        let client_seqence = self.client_counter(deps.as_ref().storage)?;
        let client_identifer = ClientId::new(client_type, client_seqence.try_into().unwrap())?;
        self.increase_client_counter(deps.storage)?;
        Ok(client_identifer)
    }
}
