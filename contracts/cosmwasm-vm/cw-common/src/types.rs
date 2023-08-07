use common::rlp::{self, Decodable, Encodable};

use cosmwasm_schema::cw_serde;
use cosmwasm_schema::serde::{Deserialize, Serialize};
use cosmwasm_std::Binary;
use cw_storage_plus::KeyDeserialize;
use std::fmt::Display;

#[cw_serde]
pub struct VerifyChannelState {
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub client_id: String,
    // commitment key
    pub counterparty_chan_end_path: Vec<u8>,
    // commitment bytes
    pub expected_counterparty_channel_end: Vec<u8>,
}

#[cw_serde]
pub struct VerifyPacketData {
    pub height: String,
    pub prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    // commitment key
    pub commitment_path: Vec<u8>,
    // commitment bytes
    pub commitment: Vec<u8>,
}

#[cw_serde]
pub struct VerifyPacketAcknowledgement {
    pub height: String,
    pub prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    // commitment key
    pub ack_path: Vec<u8>,
    // commitment byte
    pub ack: Vec<u8>,
}
use crate::raw_types::channel::RawMessageTimeout;
use crate::raw_types::channel::RawMessageTimeoutOnclose;
pub enum TimeoutMsgType {
    Timeout(RawMessageTimeout),
    TimeoutOnClose(RawMessageTimeoutOnclose),
}

#[cw_serde]
pub struct Address(String);

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address(value.to_string())
    }
}

impl From<&String> for Address {
    fn from(value: &String) -> Self {
        Address(value.to_string())
    }
}

impl From<&[u8]> for Address {
    fn from(value: &[u8]) -> Self {
        let address = String::from_vec(value.to_vec()).unwrap();
        Address(address)
    }
}
impl Encodable for Address {
    fn rlp_append(&self, stream: &mut rlp::RlpStream) {
        stream.begin_list(1).append(&self.0);
    }
}

impl Decodable for Address {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        Ok(Self(rlp.val_at(0)?))
    }
}

impl Address {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// IBC ACK. See:
/// https://github.com/cosmos/cosmos-sdk/blob/f999b1ff05a4db4a338a855713864497bedd4396/proto/ibc/core/channel/v1/channel.proto#L141-L147
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ack {
    Result(Binary),
    Error(String),
}
