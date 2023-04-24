use super::*;
use crate::errors::CwErrors;
use crate::types::{ClientId, ClientType};
use cosmwasm_schema::cw_serde;
use ibc::core::ics04_channel::msgs::acknowledgement::Acknowledgement;
use ibc::core::ics04_channel::packet::Packet;
use ibc::core::ics04_channel::timeout::TimeoutHeight;
use ibc::signer::Signer;
use ibc::timestamp::Timestamp;
use serde::Deserialize;
use serde::Serialize;
use std::str::FromStr;
#[cw_serde]
pub struct CreateClientResponse {
    client_type: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
}

impl CreateClientResponse {
    pub fn new(
        client_type: String,
        height: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
    ) -> Self {
        Self {
            client_type,
            height,
            client_state_commitment,
            consensus_state_commitment,
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

    pub fn get_client_type(&self) -> &str {
        &self.client_type
    }
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }
    pub fn client_type(&self) -> ClientType {
        ClientType::new(self.client_type.to_owned())
    }
}

#[cw_serde]
pub struct UpdateClientResponse {
    height: String,
    client_id: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
}

impl UpdateClientResponse {
    pub fn new(
        height: String,
        client_id: String,
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
    ) -> Self {
        Self {
            height,
            client_id,
            client_state_commitment,
            consensus_state_commitment,
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
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }
    pub fn client_id(&self) -> Result<ClientId, CwErrors> {
        ClientId::from_str(&self.client_id).map_err(CwErrors::InvalidClientId)
    }
}

#[cw_serde]
pub struct UpgradeClientResponse {
    client_id: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consensus_state_commitment: Vec<u8>,
}

impl UpgradeClientResponse {
    pub fn new(
        client_state_commitment: Vec<u8>,
        consensus_state_commitment: Vec<u8>,
        client_id: String,
        height: String,
    ) -> Self {
        {
            Self {
                height,
                client_id,
                client_state_commitment,
                consensus_state_commitment,
            }
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
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }

    pub fn client_id(&self) -> Result<ClientId, CwErrors> {
        ClientId::from_str(&self.client_id).map_err(CwErrors::InvalidClientId)
    }
}

#[cw_serde]
pub struct MisbehaviourResponse {
    client_id: String,
    pub client_state_commitment: Vec<u8>,
}

impl MisbehaviourResponse {
    pub fn new(client_id: String, client_state_commitment: Vec<u8>) -> Self {
        Self {
            client_id,
            client_state_commitment,
        }
    }
    pub fn get_client_id(&self) -> &str {
        &self.client_id
    }
    pub fn client_id(&self) -> Result<ClientId, CwErrors> {
        ClientId::from_str(&self.client_id).map_err(CwErrors::InvalidClientId)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PacketResponse {
    pub seq_on_a: Sequence,
    pub port_id_on_a: IbcPortId,
    pub chan_id_on_a: IbcChannelId,
    pub port_id_on_b: IbcPortId,
    pub chan_id_on_b: IbcChannelId,
    pub data: String,
    pub timeout_height_on_b: TimeoutHeight,
    pub timeout_timestamp_on_b: Timestamp,
}

impl From<PacketResponse> for Packet {
    fn from(packet: PacketResponse) -> Self {
        let data = hex::decode(packet.data).unwrap();
        Packet {
            seq_on_a: packet.seq_on_a,
            port_id_on_a: packet.port_id_on_a,
            chan_id_on_a: packet.chan_id_on_a,
            port_id_on_b: packet.port_id_on_b,
            chan_id_on_b: packet.chan_id_on_b,
            data,
            timeout_height_on_b: packet.timeout_height_on_b,
            timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
        }
    }
}

impl From<Packet> for PacketResponse {
    fn from(packet: Packet) -> Self {
        let data = hex::encode(packet.data);
        PacketResponse {
            seq_on_a: packet.seq_on_a,
            port_id_on_a: packet.port_id_on_a,
            chan_id_on_a: packet.chan_id_on_a,
            port_id_on_b: packet.port_id_on_b,
            chan_id_on_b: packet.chan_id_on_b,
            data,
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
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PacketDataResponse {
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
