use super::*;

impl<'a> CwIbcCoreContext<'a> {
    // query to get client type using client id
    pub fn get_client_type(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<IbcClientType, ContractError> {
        match self
            .ibc_store()
            .client_types()
            .may_load(store, client_id.clone())
        {
            Ok(result) => match result {
                Some(client_type) => Ok(client_type.client_type()),
                None => Err(ContractError::InvalidClientId {
                    client_id: client_id.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    // query to get client form registry using client id
    pub fn get_client_from_registry(
        &self,
        store: &dyn Storage,
        client_type: ClientType,
    ) -> Result<String, ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .may_load(store, client_type.clone())
        {
            Ok(result) => match result {
                Some(client) => Ok(client),
                None => Err(ContractError::InvalidClientType {
                    client_type: client_type.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // query to get impls for client id
    pub fn get_client_impls(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<String, ContractError> {
        match self
            .ibc_store()
            .client_impls()
            .may_load(store, client_id.clone())
        {
            Ok(result) => match result {
                Some(client) => Ok(client),
                None => Err(ContractError::InvalidClientId {
                    client_id: client_id.as_str().to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    // query to get next client sequence
    pub fn get_next_client_sequence(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        match self.ibc_store().next_client_sequence().may_load(store) {
            Ok(result) => match result {
                Some(sequence) => Ok(sequence),
                None => Err(ContractError::InvalidNextClientSequence {}),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // write method to increment next client sequence
    pub fn increment_next_client_sequence(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        match self.ibc_store().next_client_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        ) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(error),
        }
    }

    pub fn init_next_client_sequence(
        &self,
        store: &mut dyn Storage,
        sequence_no: u128,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .next_client_sequence()
            .save(store, &sequence_no)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_client_type(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        client_type: ClientType,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_types()
            .save(store, client_id, &client_type)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_client_into_registry(
        &self,
        store: &mut dyn Storage,
        client_type: ClientType,
        client: String,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .save(store, client_type, &client)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_client_impl(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        client: String,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_impls()
            .save(store, client_id, &client)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}

impl<'a> CwIbcCoreContext<'a> {
    fn client_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Box<dyn ibc::core::ics02_client::client_state::ClientState>, ibc::core::ContextError>
    {
        todo!()
    }

    fn decode_client_state(
        &self,
        client_state: ibc_proto::google::protobuf::Any,
    ) -> Result<Box<dyn ibc::core::ics02_client::client_state::ClientState>, ibc::core::ContextError>
    {
        todo!()
    }

    fn consensus_state(
        &self,
        client_cons_state_path: &ibc::core::ics24_host::path::ClientConsensusStatePath,
    ) -> Result<
        Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>,
        ibc::core::ContextError,
    > {
        todo!()
    }

    fn next_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ibc::core::ContextError,
    > {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ibc::core::ContextError,
    > {
        todo!()
    }

    fn host_height(&self) -> Result<ibc::Height, ibc::core::ContextError> {
        todo!()
    }

    fn host_timestamp(&self) -> Result<ibc::timestamp::Timestamp, ibc::core::ContextError> {
        todo!()
    }

    fn host_consensus_state(
        &self,
        height: &ibc::Height,
    ) -> Result<
        Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>,
        ibc::core::ContextError,
    > {
        todo!()
    }

    fn client_counter(&self) -> Result<u64, ibc::core::ContextError> {
        todo!()
    }

    fn validate_self_client(
        &self,
        client_state_of_host_on_counterparty: ibc_proto::google::protobuf::Any,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn commitment_prefix(&self) -> ibc::core::ics23_commitment::commitment::CommitmentPrefix {
        todo!()
    }

    fn client_update_time(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::timestamp::Timestamp, ibc::core::ContextError> {
        todo!()
    }

    fn client_update_height(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::Height, ibc::core::ContextError> {
        todo!()
    }

    fn max_expected_time_per_block(&self) -> std::time::Duration {
        todo!()
    }
}

impl<'a> CwIbcCoreContext<'a> {
    fn store_client_state(
        &mut self,
        client_state_path: ibc::core::ics24_host::path::ClientStatePath,
        client_state: Box<dyn ibc::core::ics02_client::client_state::ClientState>,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn store_consensus_state(
        &mut self,
        consensus_state_path: ibc::core::ics24_host::path::ClientConsensusStatePath,
        consensus_state: Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn increase_client_counter(&mut self) {
        todo!()
    }

    fn store_update_time(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        timestamp: ibc::timestamp::Timestamp,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn store_update_height(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        host_height: ibc::Height,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }
}
