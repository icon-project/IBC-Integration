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
            None => Err(ContractError::IbcContextError {
                error: ContextError::ChannelError(ChannelError::ChannelNotFound {
                    port_id: port_id.ibc_port_id().clone(),
                    channel_id: channel_id.ibc_channel_id().clone(),
                }),
            }),
        }
    }

    // Add new channel to the store
    pub fn store_channel_end(
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

    // Increment the sequence number for channel
    pub fn increase_channel_sequence(&self, store: &mut dyn Storage) -> Result<u64, ContractError> {
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
    pub fn init_channel_counter(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
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
    pub fn get_next_sequence_send(
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

    pub fn increase_next_sequence_send(
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
                    None => Err(ContractError::IbcPackketError {
                        error: PacketError::MissingNextSendSeq {
                            port_id: port_id.ibc_port_id().clone(),
                            channel_id: channel_id.ibc_channel_id().clone(),
                        },
                    }),
                }
            },
        )?;
        Ok(sequence)
    }

    // Query the sequence recieve number
    pub fn get_next_sequence_recv(
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

    pub fn increase_next_sequence_recv(
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
                    None => Err(ContractError::IbcPackketError {
                        error: PacketError::MissingNextRecvSeq {
                            port_id: port_id.ibc_port_id().clone(),
                            channel_id: channel_id.ibc_channel_id().clone(),
                        },
                    }),
                }
            },
        )?;
        Ok(sequence)
    }

    // Query the sequence acknowledgement number
    pub fn get_next_sequence_ack(
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

    pub fn increase_next_sequence_ack(
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
                    None => Err(ContractError::IbcPackketError {
                        error: PacketError::MissingNextAckSeq {
                            port_id: port_id.ibc_port_id().clone(),
                            channel_id: channel_id.ibc_channel_id().clone(),
                        },
                    }),
                }
            },
        )?;
        Ok(sequence)
    }
    // Get the channel sequence number
    pub fn channel_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_channel_sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}

//TODO : Implement Methods
#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    fn store_channel(
        &mut self,
        channel_end_path: &ibc::core::ics24_host::path::ChannelEndPath,
        channel_end: ibc::core::ics04_channel::channel::ChannelEnd,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn increase_channel_counter(&mut self) {
        todo!()
    }

    fn emit_ibc_event(&mut self, event: ibc::events::IbcEvent) {
        todo!()
    }

    fn log_message(&mut self, message: String) {
        todo!()
    }

    fn store_packet_commitment(
        &mut self,
        commitment_path: &ibc::core::ics24_host::path::CommitmentPath,
        commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn delete_packet_commitment(
        &mut self,
        commitment_path: &ibc::core::ics24_host::path::CommitmentPath,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn store_packet_receipt(
        &mut self,
        receipt_path: &ibc::core::ics24_host::path::ReceiptPath,
        receipt: ibc::core::ics04_channel::packet::Receipt,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn store_packet_acknowledgement(
        &mut self,
        ack_path: &ibc::core::ics24_host::path::AckPath,
        ack_commitment: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn delete_packet_acknowledgement(
        &mut self,
        ack_path: &ibc::core::ics24_host::path::AckPath,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    fn channel_end(
        &self,
        channel_end_path: &ibc::core::ics24_host::path::ChannelEndPath,
    ) -> Result<ibc::core::ics04_channel::channel::ChannelEnd, ibc::core::ContextError> {
        todo!()
    }

    fn get_packet_commitment(
        &self,
        commitment_path: &ibc::core::ics24_host::path::CommitmentPath,
    ) -> Result<ibc::core::ics04_channel::commitment::PacketCommitment, ibc::core::ContextError>
    {
        todo!()
    }

    fn get_packet_receipt(
        &self,
        receipt_path: &ibc::core::ics24_host::path::ReceiptPath,
    ) -> Result<ibc::core::ics04_channel::packet::Receipt, ibc::core::ContextError> {
        todo!()
    }

    fn get_packet_acknowledgement(
        &self,
        ack_path: &ibc::core::ics24_host::path::AckPath,
    ) -> Result<
        ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
        ibc::core::ContextError,
    > {
        todo!()
    }
}
