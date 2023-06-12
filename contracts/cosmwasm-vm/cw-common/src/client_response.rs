use crate::cw_types::{CwEndPoint, CwPacket};
use crate::ibc_types::{IbcChannelId, IbcClientId, IbcClientType, IbcPortId};
use crate::{
    errors::CwErrors,
    ibc_types::IbcHeight,
    types::{MessageInfo, PacketData},
};
pub use common::ibc::core::ics04_channel::packet::Packet;
use common::ibc::core::ics04_channel::timeout::TimeoutHeight;
use common::ibc::timestamp::Timestamp;
use common::ibc::{
    core::ics04_channel::{msgs::acknowledgement::Acknowledgement, packet::Sequence},
    signer::Signer,
};
use cosmwasm_schema::cw_serde;
use cosmwasm_schema::serde::{Deserialize, Serialize};
use std::str::FromStr;

#[cw_serde]
pub struct CreateClientResponse {
    client_type: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
    // any bytes
    consensus_state_bytes: Vec<u8>,
    // any bytes
    client_state_bytes: Vec<u8>,
}

impl Default for CreateClientResponse {
    fn default() -> Self {
        Self {
            client_type: "iconlightclient".to_string(),
            height: Default::default(),
            client_state_commitment: Default::default(),
            consensus_state_commitment: Default::default(),
            consensus_state_bytes: Default::default(),
            client_state_bytes: Default::default(),
        }
    }
}

impl CreateClientResponse {
    pub fn new(
        client_type: String,
        height: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
        client_state_bytes: Vec<u8>,
        consensus_state_bytes: Vec<u8>,
    ) -> Self {
        Self {
            client_type,
            height,
            client_state_commitment,
            consensus_state_commitment,
            client_state_bytes,
            consensus_state_bytes,
        }
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }

    pub fn client_state_bytes(&self) -> &[u8] {
        &self.client_state_bytes
    }
    pub fn consensus_state_bytes(&self) -> &[u8] {
        &self.consensus_state_bytes
    }

    pub fn get_height(&self) -> &str {
        &self.height
    }

    pub fn get_client_type(&self) -> &str {
        &self.client_type
    }
    pub fn height(&self) -> IbcHeight {
        IbcHeight::from_str(&self.height).unwrap()
    }
    pub fn client_type(&self) -> IbcClientType {
        IbcClientType::new(self.client_type.to_owned())
    }
}

#[cw_serde]
pub struct UpdateClientResponse {
    pub height: String,
    pub client_id: String,
    pub client_state_commitment: Vec<u8>,
    pub consensus_state_commitment: Vec<u8>,
    // any bytes
    pub client_state_bytes: Vec<u8>,
    // any bytes
    pub consensus_state_bytes: Vec<u8>,
}

impl UpdateClientResponse {
    pub fn new(
        height: String,
        client_id: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
        client_state_bytes: Vec<u8>,
        consensus_state_bytes: Vec<u8>,
    ) -> Self {
        Self {
            height,
            client_id,
            client_state_commitment,
            consensus_state_commitment,
            client_state_bytes,
            consensus_state_bytes,
        }
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }

    pub fn client_state_bytes(&self) -> &[u8] {
        &self.client_state_bytes
    }
    pub fn consensus_state_bytes(&self) -> &[u8] {
        &self.consensus_state_bytes
    }

    pub fn get_height(&self) -> &str {
        &self.height
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
    pub fn height(&self) -> IbcHeight {
        IbcHeight::from_str(&self.height).unwrap()
    }
    pub fn client_id(&self) -> Result<IbcClientId, CwErrors> {
        IbcClientId::from_str(&self.client_id)
            .map_err(|e| CwErrors::InvalidClientId(self.client_id.to_string(), e))
    }
}

#[cw_serde]
pub struct UpgradeClientResponse {
    pub client_id: String,
    pub height: String,
    pub client_state_commitment: Vec<u8>,
    pub consensus_state_commitment: Vec<u8>,
    pub client_state_bytes: Vec<u8>,
    pub consensus_state_bytes: Vec<u8>,
}

impl UpgradeClientResponse {
    pub fn new(
        client_state_commitment: Vec<u8>,
        client_state_bytes: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
        consensus_state_bytes: Vec<u8>,
        client_id: String,
        height: String,
    ) -> Self {
        Self {
            height,
            client_id,
            client_state_commitment,
            consensus_state_commitment,
            client_state_bytes,
            consensus_state_bytes,
        }
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }
    pub fn get_height(&self) -> &str {
        &self.height
    }

    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
    pub fn height(&self) -> IbcHeight {
        IbcHeight::from_str(&self.height).unwrap()
    }

    pub fn client_id(&self) -> Result<IbcClientId, CwErrors> {
        IbcClientId::from_str(&self.client_id)
            .map_err(|e| CwErrors::InvalidClientId(self.client_id.to_string(), e))
    }
}

#[cw_serde]
pub struct MisbehaviourResponse {
    client_id: String,
    pub client_state_commitment: Vec<u8>,
    pub client_state_bytes: Vec<u8>,
}

impl MisbehaviourResponse {
    pub fn new(
        client_id: String,
        client_state_commitment: Vec<u8>,
        client_state_bytes: Vec<u8>,
    ) -> Self {
        Self {
            client_id,
            client_state_commitment,
            client_state_bytes,
        }
    }
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
    pub fn client_id(&self) -> Result<IbcClientId, CwErrors> {
        IbcClientId::from_str(&self.client_id)
            .map_err(|e| CwErrors::InvalidClientId(self.client_id.to_string(), e))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketResponse {
    pub seq_on_a: Sequence,
    pub port_id_on_a: IbcPortId,
    pub chan_id_on_a: IbcChannelId,
    pub port_id_on_b: IbcPortId,
    pub chan_id_on_b: IbcChannelId,
    pub data: Vec<u8>,
    pub timeout_height_on_b: TimeoutHeight,
    pub timeout_timestamp_on_b: Timestamp,
}

impl From<PacketResponse> for Packet {
    fn from(packet: PacketResponse) -> Self {
        Packet {
            sequence: packet.seq_on_a,
            port_id_on_a: packet.port_id_on_a,
            chan_id_on_a: packet.chan_id_on_a,
            port_id_on_b: packet.port_id_on_b,
            chan_id_on_b: packet.chan_id_on_b,
            data: packet.data,
            timeout_height_on_b: packet.timeout_height_on_b,
            timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
        }
    }
}

impl From<Packet> for PacketResponse {
    fn from(packet: Packet) -> Self {
        // let data = hex::encode(packet.data);
        PacketResponse {
            seq_on_a: packet.sequence,
            port_id_on_a: packet.port_id_on_a,
            chan_id_on_a: packet.chan_id_on_a,
            port_id_on_b: packet.port_id_on_b,
            chan_id_on_b: packet.chan_id_on_b,
            data: packet.data,
            timeout_height_on_b: packet.timeout_height_on_b,
            timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
        }
    }
}

impl From<PacketData> for PacketDataResponse {
    fn from(value: PacketData) -> Self {
        PacketDataResponse {
            packet: PacketResponse::from(value.packet),
            acknowledgement: value.acknowledgement,
            signer: value.signer,
            message_info: value.message_info,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PacketDataResponse {
    pub message_info: MessageInfo,
    pub packet: PacketResponse,
    pub signer: Signer,
    pub acknowledgement: Option<Acknowledgement>,
}

#[cw_serde]
pub struct OpenConfirmResponse {
    pub conn_id: String,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
}

#[cw_serde]
pub struct OpenTryResponse {
    pub conn_id: String,
    pub client_id: String,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
    pub versions: Vec<u8>,
    pub delay_period: u64,
}

impl OpenTryResponse {
    pub fn new(
        conn_id: String,
        client_id: String,
        counterparty_client_id: String,
        counterparty_connection_id: String,
        counterparty_prefix: Vec<u8>,
        versions: Vec<u8>,
        delay_period: u64,
    ) -> Self {
        Self {
            conn_id,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_prefix,
            versions,
            delay_period,
        }
    }
}

#[cw_serde]
pub struct OpenAckResponse {
    pub conn_id: String,
    pub version: Vec<u8>,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
}

#[cw_serde]
pub struct XcallPacketResponseData {
    pub packet: CwPacket,
    pub acknowledgement: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LightClientResponse {
    pub message_info: MessageInfo,
    pub ibc_endpoint: CwEndPoint,
}

#[cw_serde]
pub struct XcallPacketAck {
    pub acknowledgement: Vec<u8>,
}
