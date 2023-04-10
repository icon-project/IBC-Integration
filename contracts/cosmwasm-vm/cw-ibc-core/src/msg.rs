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
    VerifyConection {
        verify_connection_state: VerifyConnectionState,
        verify_client_full_satate: VerifyClientFullState,
        vefiry_client_consensus_state: VerifyClientConsesnusState,
    },
}
