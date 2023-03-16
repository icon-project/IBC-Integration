use crate::context::CwIbcCoreContext;

use super::*;

impl<'a> CwIbcCoreContext<'a> {
    // Get the channel from the store
    pub fn get_channel_end(
        &self,
        store: &dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<ChannelEnd, ContractError> {
        match self
            .ibc_store()
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
            .ibc_store()
            .channels()
            .save(store, (port_id, channel_id), &channel_end)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Get the channel sequence number
    pub fn query_channel_sequence(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        match self.ibc_store().next_channel_sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Increment the sequence number for channel
    pub fn increment_channel_sequence(
        &self,
        store: &mut dyn Storage,
    ) -> Result<u128, ContractError> {
        let sequence = self.ibc_store().next_channel_sequence().update(
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
        match self
            .ibc_store()
            .next_channel_sequence()
            .save(store, &sequence_no)
        {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    // Query the sequence send number
    pub fn query_next_sequence_send(
        &self,
        store: &dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self
            .ibc_store()
            .next_sequence_send()
            .load(store, (port_id, channel_id))
        {
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
            .ibc_store()
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
        let sequence = self.ibc_store().next_sequence_send().update(
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
        store: &dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self
            .ibc_store()
            .next_sequence_recv()
            .load(store, (port_id, channel_id))
        {
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
            .ibc_store()
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
        let sequence = self.ibc_store().next_sequence_recv().update(
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
        store: &dyn Storage,
        port_id: PortId,
        channel_id: ChannelId,
    ) -> Result<Sequence, ContractError> {
        match self
            .ibc_store()
            .next_sequence_ack()
            .load(store, (port_id, channel_id))
        {
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
            .ibc_store()
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
        let sequence = self.ibc_store().next_sequence_ack().update(
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
