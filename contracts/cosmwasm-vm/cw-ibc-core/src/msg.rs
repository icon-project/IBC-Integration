use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cw_serde]
pub enum LightClientMessage {
    CreateClient {
        client_id: String,
        client_state: Vec<u8>,
        consensus_state: Vec<u8>,
    },
    UpdateClient {
        client_id: String,
        header: Vec<u8>,
    },
    UpgradeClient {
        upgraded_client_state: Vec<u8>,
        upgraded_consensus_state: Vec<u8>,
        proof_upgrade_client: Vec<u8>,
        proof_upgrade_consensus_state: Vec<u8>,
    },
    VerifyChannel {
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        counterparty_chan_end_path: Vec<u8>,
        expected_counterparty_channel_end: Vec<u8>,
    },
    Misbehaviour {
        client_id: String,
        misbehaviour: Vec<u8>,
    },
}

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
    pub fn client_type(&self) -> ClientType {
        ClientType::new(self.client_type.to_owned())
    }

    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
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
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consensus_state_commitment(&self) -> &[u8] {
        &self.consensus_state_commitment
    }
    pub fn client_id(&self) -> Result<ClientId, ContractError> {
        ClientId::from_str(&self.client_id).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })
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

    pub fn client_id(&self) -> Result<ClientId, ContractError> {
        ClientId::from_str(&self.client_id).map_err(|error| ContractError::IbcClientError {
            error: ClientError::InvalidClientIdentifier(error),
        })
    }

    pub fn client_state_commitment(&self) -> &[u8] {
        &self.client_state_commitment
    }
    pub fn consesnus_state_commitment(&self) -> &[u8] {
        &self.consesnus_state_commitment
    }
    pub fn height(&self) -> Height {
        Height::from_str(&self.height).unwrap()
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
    pub fn client_id(&self) -> Result<ClientId, ContractError> {
        ClientId::from_str(&self.client_id).map_err(|error| ContractError::IbcClientError {
            error: ClientError::InvalidClientIdentifier(error),
        })
    }
}
