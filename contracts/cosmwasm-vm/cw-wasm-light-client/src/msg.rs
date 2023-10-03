use cosmwasm_schema::cw_serde;
use ics07_tendermint_cw::ics23::FakeInner;
use ics08_wasm::{
    client_message::Header as WasmHeader, client_state::ClientState as WasmClientState,
    consensus_state::ConsensusState as WasmConsensusState,
};

use serde::{Deserializer, Serializer};

#[cw_serde]
pub struct HeightRaw {
    pub revision_number: u64,

    pub revision_height: u64,
}

struct Base64;

impl Base64 {
    pub fn serialize<S: Serializer>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        base64::serialize(v, serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        base64::deserialize(deserializer)
    }
}

pub mod base64 {
    use alloc::{string::String, vec::Vec};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        let mut buf = String::new();
        base64::encode_config_buf(v, base64::STANDARD, &mut buf);

        String::serialize(&buf, serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(deserializer)?;

        let mut buf = Vec::new();
        base64::decode_config_buf(base64.as_bytes(), base64::STANDARD, &mut buf)
            .map_err(serde::de::Error::custom)?;

        Ok(buf)
    }
}

#[cw_serde]
pub struct GenesisMetadata {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

#[cw_serde]
pub struct QueryResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genesis_metadata: Option<Vec<GenesisMetadata>>,
}

impl QueryResponse {
    pub fn status(status: String) -> Self {
        Self {
            status,
            genesis_metadata: None,
        }
    }

    pub fn genesis_metadata(genesis_metadata: Option<Vec<GenesisMetadata>>) -> Self {
        Self {
            status: "".to_string(),
            genesis_metadata,
        }
    }
}

#[cw_serde]
pub struct ContractResult {
    pub is_valid: bool,
    pub error_msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    pub found_misbehaviour: bool,
}
#[allow(dead_code)]
impl ContractResult {
    pub fn success() -> Self {
        Self {
            is_valid: true,
            error_msg: "".to_string(),
            data: None,
            found_misbehaviour: false,
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            is_valid: false,
            error_msg: msg,
            data: None,
            found_misbehaviour: false,
        }
    }

    pub fn misbehaviour(mut self, found: bool) -> Self {
        self.found_misbehaviour = found;
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(data);
        self
    }
}

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    VerifyMembership(VerifyMembershipMsgRaw),
    VerifyNonMembership(VerifyNonMembershipMsgRaw),
    VerifyClientMessage(VerifyClientMessageRaw),
    CheckForMisbehaviour(CheckForMisbehaviourMsgRaw),
    UpdateStateOnMisbehaviour(UpdateStateOnMisbehaviourMsgRaw),
    UpdateState(UpdateStateMsgRaw),
    CheckSubstituteAndUpdateState(CheckSubstituteAndUpdateStateMsg),
    VerifyUpgradeAndUpdateState(VerifyUpgradeAndUpdateStateMsgRaw),
}

#[cw_serde]
pub enum QueryMsg {
    ClientTypeMsg(ClientTypeMsg),
    GetLatestHeightsMsg(GetLatestHeightsMsg),
    ExportMetadata(ExportMetadataMsg),
    Status(StatusMsg),
    GetClientState {},
}

#[cw_serde]
pub struct ClientTypeMsg {}

#[cw_serde]
pub struct GetLatestHeightsMsg {}

#[cw_serde]
pub struct StatusMsg {}

#[cw_serde]
pub struct ExportMetadataMsg {}

#[cw_serde]
pub struct MerklePath {
    pub key_path: Vec<String>,
}

#[cw_serde]
pub struct VerifyMembershipMsgRaw {
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub proof: Vec<u8>,
    pub path: MerklePath,
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub value: Vec<u8>,
    pub height: HeightRaw,
    pub delay_block_period: u64,
    pub delay_time_period: u64,
}

#[cw_serde]
pub struct VerifyNonMembershipMsgRaw {
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub proof: Vec<u8>,
    pub path: MerklePath,
    pub height: HeightRaw,
    pub delay_block_period: u64,
    pub delay_time_period: u64,
}

#[cw_serde]
pub struct WasmMisbehaviour {
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub data: Vec<u8>,
}

#[cw_serde]
pub enum ClientMessageRaw {
    Header(WasmHeader<FakeInner>),
    Misbehaviour(WasmMisbehaviour),
}

#[cw_serde]
pub struct VerifyClientMessageRaw {
    pub client_message: ClientMessageRaw,
}

#[cw_serde]
pub struct CheckForMisbehaviourMsgRaw {
    pub client_message: ClientMessageRaw,
}

#[cw_serde]
pub struct UpdateStateOnMisbehaviourMsgRaw {
    pub client_message: ClientMessageRaw,
}

#[cw_serde]
pub struct UpdateStateMsgRaw {
    pub client_message: ClientMessageRaw,
}

#[cw_serde]
pub struct CheckSubstituteAndUpdateStateMsg {}

#[cw_serde]
pub struct VerifyUpgradeAndUpdateStateMsgRaw {
    pub upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
    pub upgrade_consensus_state: WasmConsensusState<FakeInner>,
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub proof_upgrade_client: Vec<u8>,
    #[schemars(with = "String")]
    #[serde(with = "Base64", default)]
    pub proof_upgrade_consensus_state: Vec<u8>,
}
