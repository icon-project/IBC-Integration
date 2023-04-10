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
    pub fn store_channel(
        &self,
        store: &mut dyn Storage,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
        channel_end: ibc::core::ics04_channel::channel::ChannelEnd,
    ) -> Result<(), ContractError> {
        let channel_commitemtn_key = self.channel_commitment_key(port_id, channel_id);

        let channel_end_bytes = to_vec(&channel_end).map_err(|error| ContractError::Std(error))?;

        self.ibc_store()
            .commitments()
            .save(store, channel_commitemtn_key, &channel_end_bytes)?;

        Ok(())
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

    pub fn store_packet_commitment(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        commitment: ibc::core::ics04_channel::commitment::PacketCommitment,
    ) -> Result<(), ContractError> {
        let commitment_path = self.packet_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_bytes = to_vec(&commitment).map_err(|error| ContractError::Std(error))?;
        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    pub fn delete_packet_commitment(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), ContractError> {
        let commitment_path = self.packet_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        self.ibc_store()
            .commitments()
            .remove(store, commitment_path);

        Ok(())
    }

    pub fn store_packet_receipt(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        receipt: ibc::core::ics04_channel::packet::Receipt,
    ) -> Result<(), ContractError> {
        let commitment_path = self.packet_receipt_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let ok = match receipt {
            ibc::core::ics04_channel::packet::Receipt::Ok => true,
        };
        let commitment_bytes = to_vec(&ok).map_err(|error| ContractError::Std(error))?;
        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    pub fn store_packet_acknowledgement(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        ack_commitment: ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
    ) -> Result<(), ContractError> {
        let commitment_path = self.packet_acknowledgement_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_bytes = ack_commitment.into_vec();

        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    pub fn delete_packet_acknowledgement(
        &mut self,
        ack_path: &ibc::core::ics24_host::path::AckPath,
    ) -> Result<(), ibc::core::ContextError> {
        todo!()
    }

    pub fn channel_end(
        &self,
        store: &mut dyn Storage,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
    ) -> Result<ibc::core::ics04_channel::channel::ChannelEnd, ContractError> {
        let channel_commitemtn_key = self.channel_commitment_key(port_id, channel_id);

        let channel_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, channel_commitemtn_key)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "ChannelNotFound".to_string(),
            })?;

        let channel_end: ChannelEnd =
            serde_json_wasm::from_slice(&channel_end_bytes).map_err(|error| {
                ContractError::IbcDecodeError {
                    error: error.to_string(),
                }
            })?;
        Ok(channel_end)
    }

    pub fn get_packet_commitment(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<ibc::core::ics04_channel::commitment::PacketCommitment, ContractError> {
        let commitment_path = self.packet_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "PacketCommitmentNotFound".to_string(),
            })?;
        let commitment: PacketCommitment = serde_json_wasm::from_slice(&commitment_end_bytes)
            .map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;

        Ok(commitment)
    }

    pub fn get_packet_receipt(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<ibc::core::ics04_channel::packet::Receipt, ContractError> {
        let commitment_path = self.packet_receipt_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "PacketCommitmentNotFound".to_string(),
            })?;
        let commitment: bool =
            serde_json_wasm::from_slice(&commitment_end_bytes).map_err(|error| {
                ContractError::IbcDecodeError {
                    error: error.to_string(),
                }
            })?;
        match commitment {
            true => Ok(ibc::core::ics04_channel::packet::Receipt::Ok),
            false => Err(ContractError::IbcPackketError {
                error: PacketError::PacketReceiptNotFound { sequence },
            }),
        }
    }

    pub fn get_packet_acknowledgement(
        &self,
        store: &mut dyn Storage,
        poirt_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<ibc::core::ics04_channel::commitment::AcknowledgementCommitment, ContractError>
    {
        let commitment_path = self.packet_acknowledgement_commitment_path(
            poirt_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "PacketCommitmentNotFound".to_string(),
            })?;
        let commitment = ibc::core::ics04_channel::commitment::AcknowledgementCommitment::from(
            commitment_end_bytes,
        );

        Ok(commitment)
    }
}
