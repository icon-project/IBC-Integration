use super::*;

impl<'a> CwIbcStore<'a> {
    // Get the channel from the store
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

    // Add new channel to the store
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

    // Get the channel sequence number
    pub fn query_channel_sequence(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        match self.next_channel_sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Increment the sequence number for channel
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

    // Initialize the next sequence storage
    pub fn init_next_channel_sequence(
        &self,
        store: &mut dyn Storage,
        sequence_no: u128,
    ) -> Result<(), ContractError> {
        match self.next_channel_sequence().save(store, &sequence_no) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Query the sequence send number
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

    // Storing the send sequene in the storage
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

    pub fn increment_next_sequence_send(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        let sequence = self.next_sequence_send().update(
            store,
            (port_id.clone(), channel_id.clone()),
            |req_id| -> Result<_, ContractError> {
                match req_id {
                    Some(seq) => Ok(seq.increment()),
                    None => Err(ContractError::MissingNextSendSeq {
                        port_id,
                        channel_id,
                    }),
                }
            },
        )?;
        Ok(sequence)
    }

    // Query the sequence recieve number
    pub fn query_next_sequence_recv(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self.next_sequence_recv().load(store, (port_id, channel_id)) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Storing the recieve sequene in the storage
    pub fn store_next_sequence_recv(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<(), ContractError> {
        match self
            .next_sequence_recv()
            .save(store, (port_id, channel_id), &sequence)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increment_next_sequence_recv(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        let sequence = self.next_sequence_recv().update(
            store,
            (port_id.clone(), channel_id.clone()),
            |req_id| -> Result<_, ContractError> {
                match req_id {
                    Some(seq) => Ok(seq.increment()),
                    None => Err(ContractError::MissingNextRecvSeq {
                        port_id,
                        channel_id,
                    }),
                }
            },
        )?;
        Ok(sequence)
    }

    // Query the sequence acknowledgement number
    pub fn query_next_sequence_ack(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self.next_sequence_ack().load(store, (port_id, channel_id)) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Storing the acknowledgement sequene in the storage
    pub fn store_next_sequence_ack(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
        sequence: Sequence,
    ) -> Result<(), ContractError> {
        match self
            .next_sequence_ack()
            .save(store, (port_id, channel_id), &sequence)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increment_next_sequence_ack(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        let sequence = self.next_sequence_ack().update(
            store,
            (port_id.clone(), channel_id.clone()),
            |req_id| -> Result<_, ContractError> {
                match req_id {
                    Some(seq) => Ok(seq.increment()),
                    None => Err(ContractError::MissingNextAckSeq {
                        port_id,
                        channel_id,
                    }),
                }
            },
        )?;
        Ok(sequence)
    }
}

#[cfg(test)]
mod tests {

    use cosmwasm_std::{testing::MockStorage, StdError};
    use ibc::core::ics04_channel::{
        channel::{Counterparty, Order, State},
        Version,
    };

    use super::*;

    #[test]
    fn test_add_channel() {
        let ctx = CwIbcStore::default();
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

        let _storing = ctx.add_channel_end(
            &mut storage,
            port_id.clone(),
            channel_id.clone(),
            channel_end.clone(),
        );

        let retrived_channel_end = ctx.get_channel_end(&mut storage, port_id, channel_id);

        assert_eq!(channel_end, retrived_channel_end.unwrap())
    }

    #[test]
    fn test_channel_sequence_initialisation() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let _store = ctx.init_next_channel_sequence(&mut store, u128::default());
        let result = ctx.query_channel_sequence(&mut store);

        assert_eq!(0, result.unwrap());

        let incremented_result = ctx.increment_channel_sequence(&mut store);
        assert_eq!(1, incremented_result.unwrap());
    }

    #[test]
    fn test_channel_sequence_fail() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let result = ctx.increment_channel_sequence(&mut store);

        assert_eq!(
            result,
            Err(ContractError::from(StdError::NotFound {
                kind: "u128".to_string()
            }))
        )
    }

    #[test]
    fn test_channel_sequence_send() {
        let ctx = CwIbcStore::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let sequene = Sequence::from(6);
        let mut store = MockStorage::default();

        let _store =
            ctx.store_next_sequence_send(&mut store, port_id.clone(), channel_id.clone(), sequene);
        let result = ctx.query_next_sequence_send(&mut store, port_id, channel_id);

        assert_eq!(sequene, result.unwrap())
    }

    #[test]
    fn test_channel_sequence_send_increment() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let sequence = Sequence::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let _store =
            ctx.store_next_sequence_send(&mut store, port_id.clone(), channel_id.clone(), sequence);
        let result = ctx.query_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(sequence, result.unwrap());

        let incremented_result =
            ctx.increment_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());
        assert_eq!(Sequence::from(1), incremented_result.unwrap());
    }

    #[test]
    fn test_channel_sequence_recv_increment() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let sequence = Sequence::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let _store =
            ctx.store_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone(), sequence);
        let result = ctx.query_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(sequence, result.unwrap());

        let incremented_result =
            ctx.increment_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());
        assert_eq!(Sequence::from(1), incremented_result.unwrap());
    }

    #[test]
    fn test_channel_sequence_ack_increment() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let sequence = Sequence::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let _store =
            ctx.store_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone(), sequence);
        let result = ctx.query_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(sequence, result.unwrap());

        let incremented_result =
            ctx.increment_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());
        assert_eq!(Sequence::from(1), incremented_result.unwrap());
    }

    #[test]
    fn test_channel_sequence_ack_fail() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let result =
            ctx.increment_next_sequence_ack(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(
            result,
            Err(ContractError::MissingNextAckSeq {
                port_id,
                channel_id
            })
        )
    }

    #[test]
    fn test_channel_sequence_send_fail() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let result =
            ctx.increment_next_sequence_send(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(
            result,
            Err(ContractError::MissingNextSendSeq {
                port_id,
                channel_id
            })
        )
    }

    #[test]
    fn test_channel_sequence_recv_fail() {
        let ctx = CwIbcStore::default();
        let mut store = MockStorage::default();
        let port_id = PortId::dafault();
        let channel_id = ChannelId::default();
        let result =
            ctx.increment_next_sequence_recv(&mut store, port_id.clone(), channel_id.clone());

        assert_eq!(
            result,
            Err(ContractError::MissingNextRecvSeq {
                port_id,
                channel_id
            })
        )
    }
}
