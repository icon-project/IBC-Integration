use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn commitment_prefix(&self) -> CommitmentPrefix {
        CommitmentPrefix::try_from(b"Ibc".to_vec()).unwrap_or_default()
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_connection(
        &self,
        store: &mut dyn Storage,
        conn_id: ConnectionId,
        conn_end: ConnectionEnd,
    ) -> Result<(), ContractError> {
        let data = conn_end.encode_vec().unwrap();
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

        // match self.ibc_store().connections().load(store, conn_id.clone()) {
        //     Ok(conn_end) => {
        //         let data: ConnectionEnd = ConnectionEnd::decode(&*conn_end).unwrap();
        //         Ok(data)
        //     }
        //     Err(_) => Err(ContractError::IbcContextError {
        //         error: ContextError::ConnectionError(ConnectionError::ConnectionNotFound {
        //             connection_id: conn_id.connection_id().clone(),
        //         }),
        //     }),
        // }
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
    ) -> Result<u128, ContractError> {
        let sequence_no = self.ibc_store().next_connection_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        )?;

        Ok(sequence_no)
    }

    pub fn connection_counter(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        match self.ibc_store().next_connection_sequence().load(store) {
            Ok(u128) => Ok(u128),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn connection_next_sequence_init(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
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
}
