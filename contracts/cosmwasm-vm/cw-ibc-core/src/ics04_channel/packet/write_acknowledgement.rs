use std::str::FromStr;

use crate::ChannelError::InvalidChannelState;
use common::{
    ibc::core::ics04_channel::{channel::State, commitment::AcknowledgementCommitment},
    utils::keccak256,
};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw_common::{
    commitment::acknowledgement_commitment_key,
    ibc_types::{IbcChannelId, IbcPortId, Sequence},
    raw_types::channel::RawPacket,
};

use crate::{context::CwIbcCoreContext, ics04_channel::write_acknowledgement_event, ContractError};

impl<'a> CwIbcCoreContext<'a> {
    pub fn write_acknowledgement(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        packet: RawPacket,
        ack: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let dest_port = IbcPortId::from_str(&packet.destination_port)
            .map_err(|e| ContractError::IbcValidationError { error: e })?;
        let dest_channel = IbcChannelId::from_str(&packet.destination_channel)
            .map_err(|e| ContractError::IbcValidationError { error: e })?;
        let seq = packet.sequence;

        let authenticated = self.authenticate_capability(
            deps.as_ref().storage,
            info,
            dest_port.as_bytes().to_vec(),
        );
        if !authenticated {
            return Err(ContractError::Unauthorized {});
        }
        if !ack.len() > 0 {
            return Err(ContractError::IbcPacketError {
                error: cw_common::ibc_types::PacketError::InvalidAcknowledgement,
            });
        }

        let channel = self.get_channel_end(
            deps.as_ref().storage,
            dest_port.clone(),
            dest_channel.clone(),
        )?;
        if channel.state != State::Open {
            return Err(ContractError::IbcChannelError {
                error: InvalidChannelState {
                    channel_id: dest_channel.clone(),
                    state: channel.state,
                },
            });
        }

        let ack_commitment = keccak256(&ack).to_vec();
        self.store_packet_acknowledgement(
            deps.storage,
            &dest_port,
            &dest_channel,
            Sequence::from(seq),
            AcknowledgementCommitment::from(ack_commitment),
        )?;

        let event = write_acknowledgement_event(
            packet.destination_port.clone(),
            packet.destination_channel.clone(),
            packet.sequence,
            ack,
        );

        Ok(Response::new().add_event(event))
    }
}
