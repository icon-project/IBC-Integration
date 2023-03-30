use crate::ics03_connection::event::create_open_try_event;
use ibc::core::ics03_connection::msgs::conn_open_try::MsgConnectionOpenTry;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    //initialise a connection for connection openinit
    pub fn connection_open_init(
        &self,
        message: MsgConnectionOpenInit,
        deps: DepsMut,
        client_id: ClientId,
    ) -> Result<Response, ContractError> {
        // validate
        // self.client_state(&message.client_id_on_a);
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
            conn_id.clone(),
            crate::ClientId::from(message.client_id_on_a.clone()),
            client_id_on_b,
        );
        let counter = match self.increase_connection_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        self.store_connection_to_client(deps.storage, client_id, conn_id.clone())?;
        self.store_connection(deps.storage, conn_id.clone(), conn_end)
            .unwrap();
        return Ok(Response::new().add_event(event));
    }

    //initialise a connection for connection opentry
    fn connection_open_try(
        &self,
        message: MsgConnectionOpenTry,
        deps: DepsMut,
        client_id: ClientId,
        counterparty_client_id: ClientId,
        counterparty_connection_id: ConnectionId,
    ) -> Result<Response, ContractError> {
        //TODO validate
        // self.validate_self_client(message.client_state_of_b_on_a.clone())?;
        // let host_height = self
        //     .host_height()
        //     .map_err(|_| ContractError::IbcConnectionError {
        //         error: ConnectionError::Other {
        //             description: "failed to get host height".to_string(),
        //         },
        //     })?;
        // if message.consensus_height_of_b_on_a > host_height {
        //     return Err(ContractError::IbcConnectionError {
        //         error: ConnectionError::InvalidConsensusHeight {
        //             target_height: message.consensus_height_of_b_on_a,
        //             current_height: host_height,
        //         }
        //         .into(),
        //     });
        // }
        // let client_id_on_a = message.counterparty.client_id();

        //TODO verify proofs

        //new connection
        let conn_id = ConnectionId::new(self.connection_counter(deps.storage)?.try_into().unwrap());
        let conn_end = ConnectionEnd::new(
            State::TryOpen,
            message.client_id_on_b.clone(),
            message.counterparty.clone(),
            message.versions_on_a.clone(),
            message.delay_period,
        );
        let event = create_open_try_event(
            conn_id.clone(),
            client_id.clone(),
            counterparty_connection_id,
            counterparty_client_id,
        );
        let counter = match self.increase_connection_counter(deps.storage) {
            Ok(counter) => counter,
            Err(error) => return Err(error),
        };
        self.store_connection_to_client(deps.storage, client_id, conn_id.clone())?;
        self.store_connection(deps.storage, conn_id.clone(), conn_end)
            .unwrap();
        return Ok(Response::new().add_event(event));
    }
}

pub fn get_compatible_versions() -> Vec<Version> {
    vec![Version::default()]
}
