use ibc::core::ics03_connection::msgs::conn_open_ack::MsgConnectionOpenAck;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn connection_open_init(
        &self,
        deps: DepsMut,
        message: MsgConnectionOpenInit,
    ) -> Result<Response, ContractError> {
        let connection_identifier = self.generate_connection_idenfier(deps.storage)?;

        self.client_state(deps.storage, &message.client_id_on_a.clone())?;

        let client_id = ClientId::from(message.client_id_on_a.clone());

        self.check_for_connection(deps.as_ref().storage, client_id.clone())?;

        let versions = match message.version {
            Some(version) => {
                if self.get_compatible_versions().contains(&version) {
                    vec![version]
                } else {
                    return Err(ContractError::IbcConnectionError {
                        error: (ConnectionError::EmptyVersions),
                    });
                }
            }
            None => self.get_compatible_versions(),
        };

        let connection_end = ConnectionEnd::new(
            State::Init,
            message.client_id_on_a,
            message.counterparty.clone(),
            versions,
            message.delay_period,
        );

        self.update_connection_commitment(
            deps.storage,
            connection_identifier.clone(),
            connection_end.clone(),
        )?;
        self.store_connection_to_client(
            deps.storage,
            client_id.clone(),
            connection_identifier.clone(),
        )?;
        self.store_connection(deps.storage, connection_identifier.clone(), connection_end)
            .unwrap();

        let event = create_open_init_event(
            connection_identifier.connection_id().as_str(),
            client_id.as_str(),
            message.counterparty.client_id().as_str(),
        );

        return Ok(Response::new()
            .add_attribute("method", "connection_open_init")
            .add_attribute("connection_id", connection_identifier.as_str())
            .add_event(event));
    }

    pub fn generate_connection_idenfier(
        &self,
        store: &mut dyn Storage,
    ) -> Result<ConnectionId, ContractError> {
        let counter = self.connection_counter(store)?;

        let connection_id = ConnectionId::new(counter);

        self.increase_connection_counter(store)?;

        Ok(connection_id)
    }
    pub fn get_compatible_versions(&self) -> Vec<Version> {
        vec![Version::default()]
    }

    pub fn connection_open_ack(&self,deps:DepsMut, msg : MsgConnectionOpenAck){

    }
}
