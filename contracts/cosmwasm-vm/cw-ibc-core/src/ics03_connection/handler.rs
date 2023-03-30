use ibc::core::ics24_host::identifier::ClientId;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn connection_open_init(
        &self,
        message: MsgConnectionOpenInit,
        deps: DepsMut,
        client_id: ClientId,
    ) -> Result<Response, ContractError> {
        // validate
        self.client_state(&message.client_id_on_a);
        let v = get_compatible_versions();

        let versions = match message.version {
            Some(version) => {
                if v.contains(&version) {
                    vec![version]
                } else {
                    return Err(ContractError::IbcConnectionError {
                        error: (ConnectionError::EmptyVersions),
                    });
                }
            }
            None => v,
        };
        let conn_end = ConnectionEnd::new(
            State::Init,
            message.client_id_on_a.clone(),
            Counterparty::new(
                message.counterparty.client_id().clone(),
                None,
                message.counterparty.prefix().clone(),
            ),
            versions,
            message.delay_period,
        );
        let conn_id = ConnectionId::new(self.connection_counter(deps.storage)?.try_into().unwrap());
        let r = message.counterparty.client_id().clone();
        let client_id_on_b = crate::ClientId::from(r);
        let event = create_open_init_event(
            conn_id,
            crate::ClientId::from(message.client_id_on_a.clone()),
            client_id_on_b,
        );
        let counter = match self.increase_connection_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        self.store_connection_to_client(deps.storage, crate::ClientId::from(client_id), conn_id)?;
        self.store_connection(deps.storage, conn_id.clone(), conn_end)
            .unwrap();
        return Ok(Response::new().add_event(event));
    }
}

pub fn connection_id(counter: u64) -> ConnectionId {
    ConnectionId::new(counter)
}

pub fn get_compatible_versions() -> Vec<Version> {
    vec![Version::default()]
}
