use super::*;

impl<'a> CwIbcCoreContext<'a> {
    // write method to increment next client sequence
    pub fn increase_client_counter(&self, store: &mut dyn Storage) -> Result<u64, ContractError> {
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

    // query to get next client sequence
    pub fn client_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_client_sequence().may_load(store) {
            Ok(result) => match result {
                Some(sequence) => Ok(sequence),
                None => Err(ContractError::InvalidNextClientSequence {}),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn init_client_counter(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
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

    // query to get implementation for client id
    pub fn get_client_implementations(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<String, ContractError> {
        match self
            .ibc_store()
            .client_implementations()
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

    pub fn store_client_implementations(
        &self,
        store: &mut dyn Storage,
        client_id: ClientId,
        client: String,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_implementations()
            .save(store, client_id, &client)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn check_client_registered(
        &self,
        store: &dyn Storage,
        client_type: ClientType,
    ) -> Result<(), ContractError> {
        match self
            .ibc_store()
            .client_registry()
            .may_load(store, client_type)
        {
            Ok(result) => match result {
                Some(_) => Err(ContractError::IbcClientError {
                    error: ClientError::Other {
                        description: "Client Implementation Already Exist".to_string(),
                    },
                }),
                None => Ok(()),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn get_client(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<String, ContractError> {
        let client = self.get_client_implementations(store, client_id.clone())?;

        if client.is_empty() {
            return Err(ContractError::IbcClientError {
                error: ClientError::ClientNotFound {
                    client_id: client_id.ibc_client_id().clone(),
                },
            });
        }
        Ok(client)
    }

    pub fn get_client_state(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<Vec<u8>, ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id.ibc_client_id());

        let client_state = self
            .ibc_store()
            .commitments()
            .load(store, client_key)
            .map_err(|_| ContractError::IbcDecodeError {
                error: format!("NotFound ClientId({})", client_id.ibc_client_id().as_str()),
            })?;

        Ok(client_state)
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    pub fn client_state(
        &self,
        store: &dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
    ) -> Result<Box<dyn ibc::core::ics02_client::client_state::ClientState>, ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);

        let client_state_data = self.ibc_store().commitments().load(store, client_key)?;

        let client_state: ClientState = client_state_data.as_slice().try_into().unwrap();

        Ok(Box::new(client_state))
    }

    pub fn decode_client_state(
        &self,
        client_state: ibc_proto::google::protobuf::Any,
    ) -> Result<Box<dyn IbcClientState>, ContractError> {
        let client_state: ClientState = ClientState::try_from(client_state).unwrap();

        Ok(Box::new(client_state))
    }

    pub fn consensus_state(
        &self,
        store: &dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<Box<dyn IbcConsensusState>, ContractError> {
        let consensus_state_key = commitment::consensus_state_commitment_key(
            client_id,
            height.revision_number(),
            height.revision_height(),
        );

        let consensus_state_data = self
            .ibc_store()
            .commitments()
            .load(store, consensus_state_key)?;

        let consensus_state: ConsensusState =
            ConsensusState::try_from(consensus_state_data).unwrap();

        Ok(Box::new(consensus_state))
    }

    fn next_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ContractError,
    > {
        todo!()
    }

    fn prev_consensus_state(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<
        Option<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>>,
        ContractError,
    > {
        todo!()
    }

    pub fn host_height(&self) -> Result<ibc::Height, ContractError> {
        Ok(ibc::Height::new(10, 10).unwrap())
    }

    pub fn host_timestamp(
        &self,
        store: &dyn Storage,
    ) -> Result<ibc::timestamp::Timestamp, ContractError> {
        //TODO Update timestamp logic
        let duration = self.ibc_store().expected_time_per_block().load(store)?;
        let block_time = Duration::from_secs(duration);
        Ok(IbcTimestamp::from_nanoseconds(block_time.as_nanos() as u64).unwrap())
    }

    pub fn host_consensus_state(
        &self,
        height: &ibc::Height,
    ) -> Result<Box<dyn ibc::core::ics02_client::consensus_state::ConsensusState>, ContractError>
    {
        todo!()
    }

    pub fn validate_self_client(
        &self,
        client_state_of_host_on_counterparty: ibc_proto::google::protobuf::Any,
    ) -> Result<(), ContractError> {
        Ok(())
    }

    pub fn client_update_time(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::timestamp::Timestamp, ContractError> {
        Ok(IbcTimestamp::none())
    }

    pub fn client_update_height(
        &self,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: &ibc::Height,
    ) -> Result<ibc::Height, ContractError> {
        Ok(ibc::Height::new(10, 10).unwrap())
    }

    pub fn max_expected_time_per_block(&self) -> std::time::Duration {
        Duration::from_secs(60)
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_client_state(
        &self,
        store: &mut dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        client_state: Vec<u8>,
    ) -> Result<(), ContractError> {
        let client_key = commitment::client_state_commitment_key(client_id);

        self.ibc_store()
            .commitments()
            .save(store, client_key, &client_state)?;

        Ok(())
    }

    pub fn store_consensus_state(
        &self,
        store: &mut dyn Storage,
        client_id: &ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        consensus_state: Vec<u8>,
    ) -> Result<(), ContractError> {
        let consensus_key = commitment::consensus_state_commitment_key(
            client_id,
            height.revision_number(),
            height.revision_height(),
        );

        self.ibc_store()
            .commitments()
            .save(store, consensus_key, &consensus_state)?;

        Ok(())
    }

    //TODO : Implement Methods
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn store_update_time(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        timestamp: ibc::timestamp::Timestamp,
    ) -> Result<(), ContractError> {
        todo!()
    }

    //TODO : Implement Methods
    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn store_update_height(
        &mut self,
        client_id: ibc::core::ics24_host::identifier::ClientId,
        height: ibc::Height,
        host_height: ibc::Height,
    ) -> Result<(), ContractError> {
        todo!()
    }
}
