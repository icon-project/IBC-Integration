// Copyright (C) 2022 ComposableFi.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// use crate::{contract::HostFunctions, ics23::FakeInner, Bytes, ContractError};
use common::ibc::{
    core::{
        ics23_commitment::commitment::{CommitmentPrefix, CommitmentProofBytes},
        ics24_host::Path,
    },
    Height,
};
use core::{str::FromStr, time::Duration};
use cosmwasm_schema::cw_serde;
use cw_light_client_common::ContractError;
use ibc_proto::google::protobuf::Any;
use std::convert::Infallible;
// use ics07_tendermint::{
// 	client_message::{ClientMessage, Header, Misbehaviour},
// 	client_state::ClientState,
// };
use crate::ics08_wasm::{
    client_message::Header as WasmHeader, client_state::ClientState as WasmClientState,
    consensus_state::ConsensusState as WasmConsensusState,
};
use prost::Message;
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

#[derive(Eq, Default)]
#[cw_serde]
pub struct FakeInner;

impl TryFrom<Any> for FakeInner {
    type Error = Infallible;

    fn try_from(_: Any) -> Result<Self, Self::Error> {
        Ok(FakeInner)
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

// pub struct VerifyMembershipMsg {
// 	pub prefix: CommitmentPrefix,
// 	pub proof: CommitmentProofBytes,
// 	pub path: Path,
// 	pub value: Vec<u8>,
// 	pub height: Height,
// 	pub delay_block_period: u64,
// 	pub delay_time_period: u64,
// }

// impl TryFrom<VerifyMembershipMsgRaw> for VerifyMembershipMsg {
// 	type Error = ContractError;

// 	fn try_from(mut raw: VerifyMembershipMsgRaw) -> Result<Self, Self::Error> {
// 		let proof = CommitmentProofBytes::try_from(raw.proof)?;
// 		let prefix = raw.path.key_path.remove(0).into_bytes();
// 		let path_str = raw.path.key_path.join("");
// 		let path = Path::from_str(&path_str)?;
// 		let height = Height::from(raw.height);
// 		Ok(Self {
// 			proof,
// 			path,
// 			value: raw.value,
// 			height,
// 			prefix: CommitmentPrefix::try_from(prefix)?,
// 			delay_block_period: raw.delay_block_period,
// 			delay_time_period: raw.delay_time_period,
// 		})
// 	}
// }

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

// pub struct VerifyNonMembershipMsg {
// 	pub prefix: CommitmentPrefix,
// 	pub proof: CommitmentProofBytes,
// 	pub path: Path,
// 	pub height: Height,
// 	pub delay_block_period: u64,
// 	pub delay_time_period: u64,
// }

// impl TryFrom<VerifyNonMembershipMsgRaw> for VerifyNonMembershipMsg {
// 	type Error = ContractError;

// 	fn try_from(mut raw: VerifyNonMembershipMsgRaw) -> Result<Self, Self::Error> {
// 		let proof = CommitmentProofBytes::try_from(raw.proof)?;
// 		let prefix = raw.path.key_path.remove(0).into_bytes();
// 		let path_str = raw.path.key_path.join("");
// 		let path = Path::from_str(&path_str)?;
// 		let height = Height::from(raw.height);
// 		Ok(Self {
// 			proof,
// 			path,
// 			height,
// 			prefix: CommitmentPrefix::try_from(prefix)?,
// 			delay_block_period: raw.delay_block_period,
// 			delay_time_period: raw.delay_time_period,
// 		})
// 	}
// }

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

// pub struct VerifyUpgradeAndUpdateStateMsg {
// 	pub upgrade_client_state: WasmClientState<FakeInner, FakeInner, FakeInner>,
// 	pub upgrade_consensus_state: WasmConsensusState<FakeInner>,
// 	pub proof_upgrade_client: CommitmentProofBytes,
// 	pub proof_upgrade_consensus_state: CommitmentProofBytes,
// }
