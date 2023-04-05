use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_connection(
        &self,
        store: &mut dyn Storage,
        conn_id: ConnectionId,
        conn_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        let data = conn_end
            .encode_vec()
            .map_err(|error| ContractError::IbcConnectionError {
                error: ConnectionError::Other {
                    description: error.to_string(),
                },
            })?;
        match self.ibc_store().connections().save(store, conn_id, &data) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn connection_end(
        &self,
        store: &dyn Storage,
        conn_id: ConnectionId,
    ) -> Result<ConnectionEnd, ContractError> {
        let data = self
            .ibc_store()
            .connections()
            .load(store, conn_id)
            .map_err(|error| ContractError::Std(error))?;

        let connection_end =
            ConnectionEnd::decode(&*data).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;

        Ok(connection_end)
    }

    pub fn store_connection_to_client(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        conn_id: ConnectionId,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_connections()
            .save(store, client_id, &conn_id)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn client_connection(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<ConnectionId, ContractError> {
        Ok(self
            .ibc_store()
            .client_connections()
            .load(store, client_id)
            .map_err(|error| ContractError::Std(error))?)
    }
    pub fn increase_connection_counter(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u64, ContractError> {
        let sequence_no = self.ibc_store().next_connection_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        )?;

        Ok(sequence_no)
    }

    pub fn connection_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_connection_sequence().load(store) {
            Ok(result) => Ok(result),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn connection_next_sequence_init(
        &self,
        store: &mut dyn Storage,
        sequence: u64,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_connection_sequence()
            .save(store, &sequence)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn init_connection_counter(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_connection_sequence()
            .save(store, &sequence_no)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn check_for_connection(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_connections()
            .may_load(store, client_id)
        {
            Ok(result) => match result {
                Some(id) => Err(ContractError::IbcConnectionError {
                    error: ConnectionError::Other {
                        description: format!("Connection Already Exists {}", id.as_str()),
                    },
                }),
                None => Ok(()),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn update_connection_commitment(
        &self,
        store: &mut dyn Storage,
        connection_id: ConnectionId,
        connection_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        let connection_commit_key = self.connection_commitment_key(connection_id.connection_id());

        let connection_end_bytes =
            connection_end
                .encode_vec()
                .map_err(|error| ContractError::IbcConnectionError {
                    error: ConnectionError::Other {
                        description: error.to_string(),
                    },
                })?;

        self.ibc_store()
            .commitments()
            .save(store, connection_commit_key, &connection_end_bytes)?;

        Ok(())
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::try_from(b"Ibc".to_vec()).unwrap_or_default() //TODO
    }

    fn host_current_height(&self) -> Result<ibc::Height, ibc::core::ContextError> {
        todo!()
    }

    fn host_oldest_height(&self) -> Result<ibc::Height, ibc::core::ContextError> {
        todo!()
    }

    fn client_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ibc::core::ContextError,
    > {
        todo!()
    }
}
