use crate::ibc_types::{IbcClientId, IbcClientType};
use crate::{errors::CwErrors, ibc_types::IbcHeight};
pub use common::ibc::core::ics04_channel::packet::Packet;

use cosmwasm_schema::cw_serde;

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
