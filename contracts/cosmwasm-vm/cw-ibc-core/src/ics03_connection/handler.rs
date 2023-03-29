use cosmwasm_std::Response;

use super::{event::event_open_init, *};

impl<'a> CwIbcCoreContext<'a> {
    pub fn connection_open_init(
        &self,
        message: MsgConnectionOpenInit,
        deps: DepsMut,
        client_id: ClientId,
        counterparty_client_id: ClientId,
    ) -> Result<Response, ContractError> {
        let counter = match self.increase_connection_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        let conn_id = connection_id((counter as u128).try_into().unwrap());
        let _client_id_on_a = self.connection_end(deps.storage, conn_id.clone());
        let v = get_compatible_versions();

        let versions = match message.version {
            Some(version) => {
                if v.contains(&version) {
                    vec![version]
                } else {
                    return Err(ContractError::VersionNotSupported {});
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
        self.store_connection(deps.storage, conn_id.clone(), conn_end)
            .unwrap();
        let event = event_open_init(conn_id, client_id, counterparty_client_id);
        return Ok(Response::new().add_event(event));
    }
}

pub fn connection_id(counter: u64) -> ConnectionId {
    ConnectionId::new(counter)
}

pub fn get_compatible_versions() -> Vec<Version> {
    vec![Version::default()]
}
