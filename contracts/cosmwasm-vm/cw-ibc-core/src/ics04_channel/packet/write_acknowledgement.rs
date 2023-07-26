use super::*;
use crate::{
    conversions::{to_ibc_channel_id, to_ibc_port_id},
    ChannelError::InvalidChannelState,
};
use common::{
    ibc::core::ics04_channel::{channel::State, commitment::AcknowledgementCommitment},
    utils::keccak256,
};
use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw_common::{ibc_types::Sequence, raw_types::to_raw_packet};

use crate::{context::CwIbcCoreContext, ContractError};

impl<'a> CwIbcCoreContext<'a> {
    pub fn write_acknowledgement(
        &self,
        deps: DepsMut,
        info: MessageInfo,
        packet: CwPacket,
        ack: Vec<u8>,
    ) -> Result<Response, ContractError> {
        let ibc_port = to_ibc_port_id(&packet.dest.port_id)?;
        let ibc_channel = to_ibc_channel_id(&packet.dest.channel_id)?;
        let seq = packet.sequence;

        let authenticated =
            self.authenticate_capability(deps.as_ref().storage, info, ibc_port.as_bytes().to_vec());
        if !authenticated {
            return Err(ContractError::Unauthorized {});
        }
        if ack.is_empty() {
            return Err(ContractError::IbcPacketError {
                error: cw_common::ibc_types::PacketError::InvalidAcknowledgement,
            });
        }

        let channel = self.get_channel_end(deps.as_ref().storage, &ibc_port, &ibc_channel)?;
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

        let event = create_packet_event(
            IbcEventType::WriteAck,
            to_raw_packet(packet),
            &channel.ordering,
            &channel.connection_hops[0],
            Some(ack),
        )?;

        Ok(Response::new().add_event(event))
    }
}
