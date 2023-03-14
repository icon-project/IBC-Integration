//! ICS 04: Channel implementation that facilitates communication between
use self::state::CwIbcStore;
pub use super::*;
use cosmwasm_std::Storage;

impl<'a> CwIbcStore<'a> {
    // get the channel from the store
    pub fn get_channel_end(
        &self,
        store: &dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<ChannelEnd, ContractError> {
        match self
            .channels()
            .may_load(store, (port_id.clone(), channel_id.clone()))?
        {
            Some(request) => Ok(request),
            None => Err(ContractError::ChannelNotFound {
                port_id,
                channel_id,
            }),
        }
    }

    // add new channel to the store
    pub fn add_channel_end(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
        channel_end: ChannelEnd,
    ) -> Result<(), ContractError> {
        match self
            .channels()
            .save(store, (port_id, channel_id), &channel_end)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn query_channel_sequence(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        match self.next_channel_sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increment_channel_sequence(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence = self.next_channel_sequence().update(
            store,
            |mut req_id| -> Result<_, ContractError> {
                req_id += 1;

                Ok(req_id)
            },
        )?;
        Ok(sequence)
    }

    pub fn query_next_sequence_send(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self.next_sequence_send().load(store, (port_id, channel_id)) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_next_sequence_send(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<(), ContractError> {
        match self
            .next_sequence_send()
            .save(store, (port_id, channel_id), &sequence)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::MockStorage;
    use ibc::core::ics04_channel::{
        channel::{Counterparty, Order, State},
        Version,
    };

    use super::*;

    #[test]
    fn test_add_channe() {
        let store = CwIbcStore::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let channel_end = ChannelEnd::new(
            State::Init,
            Order::None,
            Counterparty::default(),
            Vec::default(),
            Version::from("ics-20".to_string()),
        );
        let mut storage = MockStorage::default();

        let _storing = store.add_channel_end(
            &mut storage,
            port_id.clone(),
            channel_id.clone(),
            channel_end.clone(),
        );

        let retrived_channel_end = store.get_channel_end(&mut storage, port_id, channel_id);

        assert_eq!(channel_end, retrived_channel_end.unwrap())
    }
}
