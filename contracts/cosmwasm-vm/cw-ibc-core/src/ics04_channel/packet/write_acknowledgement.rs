use super::*;
use crate::{ics04_channel::create_write_ack_event, ChannelError::InvalidChannelState};
use common::{
    ibc::core::ics04_channel::{channel::State, commitment::AcknowledgementCommitment},
    utils::keccak256,
};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw_common::ibc_types::{IbcChannelId, IbcPortId, Sequence};
use std::str::FromStr;

use crate::{context::CwIbcCoreContext, ContractError};

impl<'a> CwIbcCoreContext<'a> {
    pub fn write_acknowledgement(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        packet: CwPacket,
        ack: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let dest_port = &packet.dest.port_id;
        let dest_channel = &packet.dest.channel_id;

        let ibc_port = IbcPortId::from_str(dest_port)
            .map_err(|e| ContractError::IbcValidationError { error: e })?;
        let ibc_channel = IbcChannelId::from_str(dest_channel)
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
        if !!ack.is_empty() {
            return Err(ContractError::IbcPacketError {
                error: cw_common::ibc_types::PacketError::InvalidAcknowledgement,
            });
        }

        let channel =
            self.get_channel_end(deps.as_ref().storage, ibc_port.clone(), ibc_channel.clone())?;
        if channel.state != State::Open {
            return Err(ContractError::IbcChannelError {
                error: InvalidChannelState {
                    channel_id: ibc_channel,
                    state: channel.state,
                },
            });
        }

        let ack_commitment = keccak256(&ack).to_vec();
        self.store_packet_acknowledgement(
            deps.storage,
            &ibc_port,
            &ibc_channel,
            Sequence::from(seq),
            AcknowledgementCommitment::from(ack_commitment),
        )?;

        let event = create_write_ack_event(
            packet,
            channel.ordering.as_str(),
            channel.connection_hops[0].as_str(),
            &ack,
        )
        .unwrap();

        Ok(Response::new().add_event(event))
    }
}
