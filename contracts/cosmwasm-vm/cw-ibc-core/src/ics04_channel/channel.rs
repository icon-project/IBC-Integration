use super::*;
use cw_common::commitment;
use prost::DecodeError;
impl<'a> CwIbcCoreContext<'a> {
    /// This function retrieves the channel_end of a specified channel from storage and returns it as a result.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to access
    /// the storage of the smart contract. The `get_channel_end` function uses this parameter to load
    /// the channel end from the storage.
    /// * `port_id`: The identifier of the IBC port associated with the channel being queried.
    /// * `channel_id`: The `channel_id` parameter is of type `ChannelId` and represents the unique
    /// identifier of an IBC channel within a given port. It is used to retrieve the corresponding
    /// `ChannelEnd` from the storage.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `ChannelEnd` or a `ContractError`.
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
            None => Err(ChannelError::ChannelNotFound {
                port_id: port_id.ibc_port_id().clone(),
                channel_id: channel_id.ibc_channel_id().clone(),
            })
            .map_err(|e| Into::<ContractError>::into(e))?,
        }
    }

    /// This function stores a channel end in the IBC store for a given port and channel ID.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `Storage`. It is used to
    /// store the `channel_end` data in the IBC store. The `Storage` trait provides an interface for
    /// reading and writing data to a persistent storage.
    /// * `port_id`: The `port_id` parameter is a unique identifier for a port on a blockchain network.
    /// It is used to identify the source or destination of an inter-blockchain communication (IBC)
    /// message.
    /// * `channel_id`: `channel_id` is a unique identifier for a channel within a given port. It is
    /// used to distinguish between different channels that may exist within the same port.
    /// * `channel_end`: `channel_end` is an object of type `ChannelEnd` which contains information
    /// about the end of a channel, including its state, ordering, and connection information. It is
    /// being passed as an argument to the `store_channel_end` function, which stores this information
    /// in the storage provided as a parameter
    ///
    /// Returns:
    ///
    /// This function returns a `Result<(), ContractError>` where `()` indicates that the function
    /// returns no meaningful value on success, and `ContractError` is an error type that can be
    /// returned if there is an error while storing the channel end in the storage.
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

    /// The function increases the channel sequence number and updates it in the storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `increase_channel_sequence` function
    /// updates the channel sequence number stored in the IBC store by calling the
    /// `next_channel_sequence` function and passing the
    ///
    /// Returns:
    ///
    /// a `Result` that contains an `u64` value if the operation is successful, or a `ContractError` if
    /// there is an error.
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

    /// This function initializes a channel counter by saving a sequence number to the storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data on the blockchain. The
    /// `init_channel_counter` function takes a reference to this object as an argument so that it can
    /// save the `sequence_no`
    /// * `sequence_no`: `sequence_no` is an unsigned 64-bit integer that represents the sequence number
    /// of the channel counter. It is used to initialize the channel counter in the storage.
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` where `()` indicates that the function returns nothing on success
    /// and `ContractError` is an error type that can be returned in case of an error.
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

    /// This function retrieves the next sequence number for sending a message over a specified IBC
    /// channel.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `Storage` trait defines methods for reading
    /// and writing data to the contract's storage.
    /// * `port_id`: The `port_id` parameter is an identifier for a port in the IBC (Inter-Blockchain
    /// Communication) protocol. It is used to uniquely identify a port and its associated channels on a
    /// particular blockchain.
    /// * `channel_id`: `channel_id` is a unique identifier for a channel in the IBC (Inter-Blockchain
    /// Communication) protocol. It is used to distinguish between different channels that may exist
    /// between two blockchain networks.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `sequence` or a `ContractError`. The
    /// `sequence` represents the next sequence number for sending messages on a specific channel, while
    /// the `ContractError` represents any error that may occur while loading the sequence number from
    /// storage.
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

    /// This function stores the next sequence to be sent in a given port and channel in a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data.
    /// * `port_id`: The `port_id` parameter is an identifier for a specific port on a blockchain
    /// network. In the context of the Inter-Blockchain Communication (IBC) protocol, a port is a module
    /// that allows communication between two different blockchains.
    /// * `channel_id`: The `channel_id` parameter is an identifier for a specific channel within a
    /// given port. It is used to uniquely identify a channel and is typically a string or integer
    /// value.
    /// * `sequence`: The `sequence` parameter is a value of type `sequence` that represents the next
    /// sequence number to be used for sending messages on a particular channel. It is being saved in
    /// the storage of the contract using the `store_next_sequence_send` function.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` with either an empty `Ok(())` value if the operation is
    /// successful or a `ContractError::Std` if there is an error.
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

    /// This function increases the next sequence number for sending IBC packets on a specific channel.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and persist data.
    /// * `port_id`: The identifier of the IBC port associated with the channel for which the sequence
    /// number is being increased.
    /// * `channel_id`: The `channel_id` parameter is an identifier for a specific channel within an IBC
    /// (Inter-Blockchain Communication) connection. It is used in the `increase_next_sequence_send`
    /// function to update the sequence number for the next outgoing packet on that channel.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `sequence` or a `ContractError`.
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
                    None => Err(PacketError::MissingNextSendSeq {
                        port_id: port_id.ibc_port_id().clone(),
                        channel_id: channel_id.ibc_channel_id().clone(),
                    })
                    .map_err(|e| Into::<ContractError>::into(e))?,
                }
            },
        )?;
        Ok(sequence)
    }

    /// This function retrieves the next sequence number for a receiving channel from the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `Storage` trait defines methods for reading
    /// and writing data to the contract's storage.
    /// * `port_id`: `PortId` is a unique identifier for a port on a blockchain network. It is used in
    /// the Inter-Blockchain Communication (IBC) protocol to identify the source or destination of a
    /// packet being sent between two blockchains.
    /// * `channel_id`: `channel_id` is an identifier for a specific channel between two IBC-connected
    /// blockchains. It is used to uniquely identify the channel and its associated data, such as the
    /// sequence number of the next message to be received on the channel.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `sequence` or a `ContractError`. The
    /// `sequence` represents the next expected sequence number for a receive operation on a given
    /// channel, while the `ContractError` represents an error that occurred while attempting to load
    /// the sequence number from storage.
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

    /// This function stores the next sequence to be received in a given port and channel in a storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `store` object provides methods to read and
    /// write data to the storage.
    /// * `port_id`: The `port_id` parameter is an identifier for a specific port on a blockchain
    /// network. In the context of the Inter-Blockchain Communication (IBC) protocol, a port is a
    /// logical endpoint that allows communication between two different blockchains.
    /// * `channel_id`: The `channel_id` parameter is an identifier for a specific channel within a
    /// given port. It is used to uniquely identify a channel and is typically a string or integer
    /// value.
    /// * `sequence`: The `sequence` parameter is a value representing the next expected sequence number
    /// for a packet to be received on a particular channel. It is being saved in the storage of the
    /// contract using the `store_next_sequence_recv` function.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` type with either an `Ok(())` value indicating success or an
    /// `Err` value containing a `ContractError` if an error occurred.
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

    /// This function increases the next sequence number for a receiving channel in an IBC protocol
    /// implementation.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and update the state of the contract.
    /// * `port_id`: The identifier of the IBC port associated with the channel for which the next
    /// receive sequence should be increased.
    /// * `channel_id`: The `channel_id` parameter is an identifier for an IBC channel. It is used in
    /// conjunction with the `port_id` parameter to uniquely identify a channel in the IBC protocol.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `sequence` or a `ContractError`.
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
                    None => Err(PacketError::MissingNextRecvSeq {
                        port_id: port_id.ibc_port_id().clone(),
                        channel_id: channel_id.ibc_channel_id().clone(),
                    })
                    .map_err(|e| Into::<ContractError>::into(e))?,
                }
            },
        )?;
        Ok(sequence)
    }

    /// This function retrieves the next sequence acknowledgement from the IBC store for a given port
    /// and channel.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `Storage` trait defines a set of methods
    /// that allow reading and writing data to the contract's storage. The `store` parameter is passed
    /// to the
    /// * `port_id`: The `port_id` parameter is an identifier for a port in the IBC (Inter-Blockchain
    /// Communication) protocol. Ports are used to establish connections between different blockchains
    /// and enable the transfer of data and assets between them. In this function, `port_id` is used to
    /// identify the specific port
    /// * `channel_id`: `channel_id` is a unique identifier for a channel in the IBC (Inter-Blockchain
    /// Communication) protocol. It is used to distinguish between different channels that may exist
    /// between two blockchain networks.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either a `sequence` or a `ContractError`. The
    /// `sequence` represents the next sequence acknowledgement expected by the IBC module for a given
    /// `port_id` and `channel_id`. If there is an error while loading the sequence from the storage, a
    /// `ContractError` is returned.
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

    /// This function stores the next sequence acknowledgement in the IBC store for a given port and
    /// channel.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the storage of the smart contract. The `store` object provides methods to read
    /// and write data to the storage.
    /// * `port_id`: The `port_id` parameter is an identifier for a specific port on a blockchain
    /// network. In the context of the function `store_next_sequence_ack`, it is used to identify the
    /// port for which the next sequence acknowledgement needs to be stored.
    /// * `channel_id`: The `channel_id` parameter is an identifier for a specific channel within a
    /// port. It is used to uniquely identify a channel and is typically a string or integer value.
    /// * `sequence`: The `sequence` parameter is a value representing the next sequence number to be
    /// acknowledged in a channel of the Inter-Blockchain Communication (IBC) protocol. It is stored in
    /// the IBC store associated with the given `port_id` and `channel_id` using the provided `store`
    /// object implementing the
    ///
    /// Returns:
    ///
    /// a `Result` type with either an `Ok(())` value indicating success or an `Err` value containing a
    /// `ContractError` if an error occurred.
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

    /// This function increases the next sequence acknowledgement for a given port and channel in an IBC
    /// store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and persist data.
    /// * `port_id`: The identifier of the IBC port associated with the channel for which the next
    /// sequence acknowledgement is being increased.
    /// * `channel_id`: The `channel_id` parameter is an identifier for an IBC channel. It is used in
    /// the `increase_next_sequence_ack` function to update the next sequence acknowledgement number for
    /// a specific channel.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing a `sequence` or a `ContractError`.
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
                    None => Err(PacketError::MissingNextAckSeq {
                        port_id: port_id.ibc_port_id().clone(),
                        channel_id: channel_id.ibc_channel_id().clone(),
                    })
                    .map_err(|e| Into::<ContractError>::into(e))?,
                }
            },
        )?;
        Ok(sequence)
    }

    /// This function retrieves the next channel sequence number from the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the smart contract. The `channel_counter` function takes a
    /// reference to this object as an argument so that it can read the value of the
    /// `next_channel_sequence` key from
    ///
    /// Returns:
    ///
    /// The function `channel_counter` returns a `Result<u64, ContractError>`. If the
    /// `next_channel_sequence()` method call on the `ibc_store()` returns an `Ok` value, then the
    /// function returns the `u64` value inside the `Ok` variant. If the method call returns an `Err`
    /// value, then the function returns a `ContractError` wrapped inside an
    pub fn channel_counter(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().next_channel_sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<'a> CwIbcCoreContext<'a> {
    /// This function stores a channel end in the IBC store commitment.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a trait object of type `Storage`, which is used to store the
    /// channel information.
    /// * `port_id`: The identifier of the IBC port associated with the channel being stored.
    /// * `channel_id`: The unique identifier of the IBC channel being stored.
    /// * `channel_end`: The `channel_end` parameter is an instance of the `ChannelEnd` struct from the
    /// `common::ibc::core::ics04_channel::channel` module. It represents the end of a channel and contains
    /// information such as the channel state, the counterparty channel identifier, the connection hops,
    /// and the
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` which indicates that it either returns an empty `Ok` value or a
    /// `ContractError` if an error occurs during the execution of the function.
    pub fn store_channel(
        &self,
        store: &mut dyn Storage,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
        channel_end: common::ibc::core::ics04_channel::channel::ChannelEnd,
    ) -> Result<(), ContractError> {
        let channel_commitment_key = commitment::channel_commitment_key(port_id, channel_id);

        let channel_end_bytes = to_vec(&channel_end).map_err(ContractError::Std)?;

        self.ibc_store()
            .commitments()
            .save(store, channel_commitment_key, &channel_end_bytes)?;

        Ok(())
    }

    /// This function stores a packet commitment in the IBC store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and save the packet commitment.
    /// * `port_id`: The identifier of the IBC port associated with the channel.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for a channel in the IBC
    /// (Inter-Blockchain Communication) protocol. It is used to distinguish between different channels
    /// that may exist between two interconnected blockchains.
    /// * `sequence`: The sequence parameter is a unique identifier for a packet within a channel. It is
    /// used to ensure that packets are processed in the correct order and to prevent replay attacks.
    /// * `commitment`: The `commitment` parameter is of type
    /// `common::ibc::core::ics04_channel::commitment::PacketCommitment`, which represents the commitment to a
    /// packet sent over a channel in the IBC protocol. It contains information such as the hash of the
    /// packet data, the sequence number of the
    ///
    /// Returns:
    ///
    /// a `Result` with an empty tuple `()` as the success value and a `ContractError` as the error
    /// value.
    pub fn store_packet_commitment(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        commitment: common::ibc::core::ics04_channel::commitment::PacketCommitment,
    ) -> Result<(), ContractError> {
        let commitment_path = commitment::packet_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_bytes = to_vec(&commitment).map_err(ContractError::Std)?;
        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    /// This function deletes a packet commitment from the store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the storage of the contract. The `dyn` keyword indicates that the type of the
    /// object implementing the `Storage` trait is not known at compile time and will be determined at
    /// runtime.
    /// * `port_id`: The identifier of the IBC port associated with the channel whose packet commitment
    /// is being deleted.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for an IBC channel. It is used
    /// to specify which channel the packet commitment should be deleted from.
    /// * `sequence`: The `sequence` parameter is a unique identifier for a packet within a channel. It
    /// is used to track the order of packets sent and received on a channel. When a packet is sent on a
    /// channel, it is assigned a sequence number, which is incremented for each subsequent packet sent
    /// on the same channel
    ///
    /// Returns:
    ///
    /// a `Result` object with the `Ok` variant containing an empty tuple `()` and the `Err` variant
    /// containing a `ContractError` object.
    pub fn delete_packet_commitment(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<(), ContractError> {
        let commitment_path = commitment::packet_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        self.ibc_store()
            .commitments()
            .remove(store, commitment_path);

        Ok(())
    }

    /// This function stores a packet receipt commitment in the IBC store with a commitment path and a boolean
    /// value indicating whether the receipt is ok or not.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and save the packet receipt commitment.
    /// * `port_id`: The identifier of the IBC port associated with the packet being acknowledged.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for an IBC channel. It is of
    /// type `ChannelId`, which is a struct that contains the `port_id` and `channel_ordering` fields.
    /// The `port_id` field identifies the IBC port associated with the channel, while
    /// * `sequence`: sequence is a unique identifier for a packet within a channel. It is used to
    /// ensure that packets are processed in the correct order and to prevent replay attacks.
    /// * `receipt`: The `receipt` parameter is of type `common::ibc::core::ics04_channel::packet::Receipt`,
    /// which represents the acknowledgement of a packet being received and processed by the
    /// counterparty chain in the IBC protocol. It can have two possible values: `Ok` or `Err`. If the
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` which indicates that it either returns an empty value or an error
    /// of type `ContractError`.
    pub fn store_packet_receipt(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        receipt: common::ibc::core::ics04_channel::packet::Receipt,
    ) -> Result<(), ContractError> {
        let commitment_path = commitment::receipt_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let ok = match receipt {
            common::ibc::core::ics04_channel::packet::Receipt::Ok => true,
        };
        let commitment_bytes = to_vec(&ok).map_err(ContractError::Std)?;
        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    /// This function stores an acknowledgement commitment for a given port, channel, and sequence in a
    /// storage system.
    ///
    /// Arguments:
    ///
    /// * `store`: A mutable reference to a trait object of type `Storage`, which is used to store the
    /// acknowledgement commitment.
    /// * `port_id`: The identifier of the IBC port associated with the channel for which the
    /// acknowledgement is being stored.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for a channel in the IBC
    /// (Inter-Blockchain Communication) protocol. It is used to specify which channel the packet
    /// acknowledgement is being stored for.
    /// * `sequence`: The sequence number of the packet for which the acknowledgement is being stored.
    /// This is used to uniquely identify the packet within the channel.
    /// * `ack_commitment`: The acknowledgement commitment is a data structure that represents the
    /// commitment to an acknowledgement message sent over an IBC channel. It is used to ensure that the
    /// acknowledgement message is not tampered with during transmission. In this function, the
    /// acknowledgement commitment is passed as a parameter and stored in the IBC store using the
    ///
    /// Returns:
    ///
    /// a `Result<(), ContractError>` which indicates that it either returns an empty value or an error
    /// of type `ContractError`.
    pub fn store_packet_acknowledgement(
        &self,
        store: &mut dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
        ack_commitment: common::ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
    ) -> Result<(), ContractError> {
        let commitment_path = commitment::acknowledgement_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_bytes = ack_commitment.into_vec();

        self.ibc_store()
            .commitments()
            .save(store, commitment_path, &commitment_bytes)?;

        Ok(())
    }

    /// The function `delete_packet_acknowledgement` deletes a packet acknowledgement in a Rust program.
    ///
    /// Arguments:
    ///
    /// * `ack_path`: `ack_path` is a parameter of type `common::ibc::core::ics24_host::path::AckPath` which
    /// represents the path to the acknowledgement packet. It is used in the
    /// `delete_packet_acknowledgement` function to delete the acknowledgement packet associated with
    /// the given path.
    ///
    /// Returns:
    ///
    /// a `Result` type with an empty tuple `()` as the success value and a `ContractError` as the error
    /// value.
    pub fn delete_packet_acknowledgement(
        &mut self,
        ack_path: &common::ibc::core::ics24_host::path::AckPath,
    ) -> Result<(), ContractError> {
        todo!()
    }

    /// This function retrieves the channel end commitment information for a given port and channel ID from the IBC
    /// store.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to interact with the contract's storage and retrieve data related to the IBC channel.
    /// * `port_id`: The identifier of the IBC port associated with the channel.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for an IBC channel. It is used
    /// to retrieve the channel end from the store.
    ///
    /// Returns:
    ///
    /// a `Result` containing either a `ChannelEnd` object or a `ContractError`.
    pub fn channel_end(
        &self,
        store: &dyn Storage,
        port_id: &IbcPortId,
        channel_id: &IbcChannelId,
    ) -> Result<common::ibc::core::ics04_channel::channel::ChannelEnd, ContractError> {
        let channel_commitment_key = commitment::channel_commitment_key(port_id, channel_id);

        let channel_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, channel_commitment_key)
            .map_err(|_| ContractError::IbcDecodeError {
                error: DecodeError::new("ChannelNotFound".to_string()),
            })?;

        let channel_end: ChannelEnd =
            serde_json_wasm::from_slice(&channel_end_bytes).map_err(|error| {
                ContractError::IbcDecodeError {
                    error: DecodeError::new(error.to_string()),
                }
            })?;
        Ok(channel_end)
    }

    /// This function retrieves the packet commitment for a given port, channel, and sequence number
    /// from storage.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// interact with the contract's storage and load the packet commitment from the IBC store.
    /// * `port_id`: The identifier of the port associated with the channel for which the packet
    /// commitment is being retrieved.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for a channel within a given
    /// port. It is used to retrieve the packet commitment associated with a specific channel and
    /// sequence number.
    /// * `sequence`: `sequence` is a unique identifier for a packet within a channel. It is used to
    /// keep track of the order in which packets are sent and received. When a packet is sent, it is
    /// assigned a sequence number by the sending chain. This sequence number is included in the packet
    /// commitment and acknowledgement messages
    ///
    /// Returns:
    ///
    /// a `Result` that contains either a `PacketCommitment` or a `ContractError`.
    pub fn get_packet_commitment(
        &self,
        store: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<common::ibc::core::ics04_channel::commitment::PacketCommitment, ContractError> {
        let commitment_path = commitment::packet_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: DecodeError::new("PacketCommitmentNotFound".to_string()),
            })?;
        let commitment: PacketCommitment = serde_json_wasm::from_slice(&commitment_end_bytes)
            .map_err(|error| ContractError::IbcDecodeError {
                error: DecodeError::new(error.to_string()),
            })?;

        Ok(commitment)
    }

    /// The function retrieves a packet receipt from storage and returns an error if it is not found.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. It is used to
    /// access the contract's storage and load data from it.
    /// * `port_id`: The identifier of the port associated with the channel that the packet was sent
    /// over.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for a channel in the IBC
    /// protocol. It is used to specify which channel the packet was sent over and to retrieve
    /// information about the channel, such as its state and associated counterparty.
    /// * `sequence`: The `sequence` parameter is a unique identifier for a packet sent over a channel
    /// in the IBC protocol. It is used to track the order of packets and ensure that they are processed
    /// in the correct order.
    ///
    /// Returns:
    ///
    /// This function returns a `Result` containing either an
    /// `common::ibc::core::ics04_channel::packet::Receipt::Ok` if the packet receipt is found, or a
    /// `ContractError::IbcPacketError` with a `PacketError::PacketReceiptNotFound` if the packet
    /// receipt is not found.
    pub fn get_packet_receipt(
        &self,
        store: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<common::ibc::core::ics04_channel::packet::Receipt, ContractError> {
        let commitment_path = commitment::receipt_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: DecodeError::new("PacketCommitmentNotFound".to_string()),
            })?;
        let commitment: bool =
            serde_json_wasm::from_slice(&commitment_end_bytes).map_err(|error| {
                ContractError::IbcDecodeError {
                    error: DecodeError::new(error.to_string()),
                }
            })?;
        match commitment {
            true => Ok(common::ibc::core::ics04_channel::packet::Receipt::Ok),
            false => Err(ContractError::IbcPacketError {
                error: PacketError::PacketReceiptNotFound { sequence: sequence },
            }),
        }
    }

    /// This function retrieves the acknowledgement commitment for a given packet sequence in a channel.
    ///
    /// Arguments:
    ///
    /// * `store`: `store` is a mutable reference to a trait object of type `dyn Storage`. This is used
    /// to access the storage of the contract.
    /// * `port_id`: The identifier of the IBC port associated with the channel.
    /// * `channel_id`: The `channel_id` parameter is a unique identifier for an IBC channel. It is used
    /// to specify which channel the packet acknowledgement commitment is associated with.
    /// * `sequence`: The `sequence` parameter is a unique identifier for a packet sent over a channel
    /// in the IBC protocol. It is used to ensure that packets are processed in the correct order and to
    /// prevent replay attacks.
    ///
    /// Returns:
    ///
    /// a Result containing an AcknowledgementCommitment or a ContractError.
    pub fn get_packet_acknowledgement(
        &self,
        store: &dyn Storage,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: Sequence,
    ) -> Result<
        common::ibc::core::ics04_channel::commitment::AcknowledgementCommitment,
        ContractError,
    > {
        let commitment_path = commitment::acknowledgement_commitment_path(
            port_id.ibc_port_id(),
            channel_id.ibc_channel_id(),
            sequence,
        );
        let commitment_end_bytes = self
            .ibc_store()
            .commitments()
            .load(store, commitment_path)
            .map_err(|_| ContractError::IbcDecodeError {
                error: DecodeError::new("PacketCommitmentNotFound".to_string()),
            })?;
        let commitment =
            common::ibc::core::ics04_channel::commitment::AcknowledgementCommitment::from(
                commitment_end_bytes,
            );

        Ok(commitment)
    }
}
