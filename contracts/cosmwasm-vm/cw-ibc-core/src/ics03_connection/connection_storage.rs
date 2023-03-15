use crate::types::ConnectionId;
use crate::{state::CwIbcStore, ContractError};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Response, Storage};
use ibc::core::ics03_connection::connection::ConnectionEnd;

impl<'a> CwIbcStore<'a> {
    pub fn set_connection(
        &self,
        deps: DepsMut,
        conn_end: ConnectionEnd,
        conn_id: ConnectionId,
    ) -> Result<Response, ContractError> {
        self.add_connection(deps.storage, conn_end, conn_id)?;
        Ok(Response::new().add_attribute("method", "set_connection"))
    }

    pub fn get_connection(&self, deps: Deps, conn_id: ConnectionId) -> Binary {
        self.query_connection(deps.storage, conn_id).unwrap()
    }

    pub fn get_next_connection_sequence(&self, store: &mut dyn Storage, sequence: u128) -> u128 {
        self.connection_next_sequence_init(store, sequence).unwrap();
        self.query_next_sequence(store).unwrap();
        self.increase_connection_sequence(store).unwrap()
    }
}

impl<'a> CwIbcStore<'a> {
    pub fn add_connection(
        &self,
        store: &mut dyn Storage,
        conn_end: ConnectionEnd,
        conn_id: ConnectionId,
    ) -> Result<(), ContractError> {
        let data = to_binary(&conn_end).unwrap();
        match self.connections().save(store, conn_id, &data) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn query_connection(
        &self,
        store: &dyn Storage,
        conn_id: ConnectionId,
    ) -> Result<Binary, ContractError> {
        match self.connections().load(store, conn_id) {
            Ok(conn_end) => Ok(conn_end),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn connection_next_sequence_init(
        &self,
        store: &mut dyn Storage,
        sequence: u128,
    ) -> Result<(), ContractError> {
        match self.next_connection_sequence().save(store, &sequence) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn query_next_sequence(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        match self.next_connection_sequence().load(store) {
            Ok(u128) => Ok(u128),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increase_connection_sequence(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence_no = self.next_connection_sequence().update(
            store,
            |mut seq| -> Result<_, ContractError> {
                seq += 1;

                Ok(seq)
            },
        )?;

        Ok(sequence_no)
    }
}

#[cfg(test)]

mod tests {
    use std::time::Duration;

    use super::*;
    use crate::state::CwIbcStore;
    use crate::IbcConnectionId;
    use cosmwasm_std::testing::mock_dependencies;
    use cosmwasm_std::testing::MockStorage;
    use cosmwasm_std::Response;
    use ibc::core::ics03_connection::connection::Counterparty;
    use ibc::core::ics03_connection::connection::State;
    use ibc::core::ics03_connection::version::Version;

    #[test]
    fn test_set_connection() {
        let mut deps = mock_dependencies();
        let conn_end = ConnectionEnd::default();
        let conn_id = ConnectionId(IbcConnectionId::default());
        let contract = CwIbcStore::new();
        let actual_response = contract
            .set_connection(deps.as_mut(), conn_end, conn_id)
            .unwrap();
        let expected_response = Response::new().add_attribute("method", "set_connection");
        assert_eq!(actual_response, expected_response);
    }

    #[test]
    fn test_get_connection() {
        let mut s = MockStorage::default();
        let shhs = ibc::core::ics23_commitment::commitment::CommitmentPrefix::try_from(
            "hello".to_string().as_bytes().to_vec(),
        );
        let shanii = Counterparty::new(IbcClientId::default(), None, shhs.unwrap());
        let conn_end = ConnectionEnd::new(
            State::Open,
            IbcClientId::default(),
            shanii,
            vec![Version::default()],
            Duration::default(),
        );
        let conn_id = ConnectionId(IbcConnectionId::new(5));
        println!("{:?}", conn_id);
        let mut contract = CwIbcStore::new();
        let ss = contract.add_connection(&mut s, conn_end, conn_id.clone());
        let response = contract.query_connection(&mut s, conn_id);
        println!("{:?}", response);
    }

    use crate::IbcClientId;
    #[test]
    fn test_connection_sequence() {
        let mut store = MockStorage::default();
        let contract = CwIbcStore::new();
        contract
            .connection_next_sequence_init(&mut store, u128::default())
            .unwrap();
        let result = contract.get_next_connection_sequence(&mut store, 1);
        assert_eq!(0, result);
        let increment_sequence = contract.increase_connection_sequence(&mut store);
        assert_eq!(1, increment_sequence.unwrap());
    }
}
