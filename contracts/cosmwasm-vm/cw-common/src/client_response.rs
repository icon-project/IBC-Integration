use crate::errors::CwErrors;
use crate::types::ClientId;
use crate::types::ClientType;
use cosmwasm_schema::cw_serde;
use ibc::core::ics02_client::height::Height;
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
        ClientId::from_str(&self.client_id).map_err(|error| CwErrors::InvalidClientId(error))
    }
}

#[cw_serde]
pub struct UpgradeClientResponse {
    client_id: String,
    height: String,
    client_state_commitment: Vec<u8>,
    consesnus_state_commitment: Vec<u8>,
}

impl UpgradeClientResponse {
    pub fn new(
        client_state_commitment: Vec<u8>,
        consesnus_state_commitment: Vec<u8>,
        client_id: String,
        height: String,
    ) -> Self {
        {
            Self {
                height,
                client_id,
                client_state_commitment,
                consesnus_state_commitment,
            }
        }
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consesnus_state_commitment(&self) -> &[u8] {
        &self.consesnus_state_commitment
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
        ClientId::from_str(&self.client_id).map_err(|error| CwErrors::InvalidClientId(error))
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
        ClientId::from_str(&self.client_id).map_err(|error| CwErrors::InvalidClientId(error))
    }
}
