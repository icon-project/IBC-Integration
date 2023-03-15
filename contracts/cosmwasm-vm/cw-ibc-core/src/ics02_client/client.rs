use super::*;

impl<'a> CwIbcStore<'a> {
    // query to get client type using client id
    pub fn get_client_type(
        &self,
        store: &dyn Storage,
        client_id: ClientId,
    ) -> Result<IbcClientType, ContractError> {
        match self.client_types().may_load(store, client_id.clone()) {
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
        match self.client_registry().may_load(store, client_type.clone()) {
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
        match self.client_impls().may_load(store, client_id.clone()) {
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
        match self.next_client_sequence().may_load(store) {
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
        match self
            .next_client_sequence()
            .update(store, |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            }) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(error),
        }
    }
}
