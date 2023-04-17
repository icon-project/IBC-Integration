use ibc::{
    core::ics04_channel::{
        msgs::{
            acknowledgement::Acknowledgement, timeout::MsgTimeout,
            timeout_on_close::MsgTimeoutOnClose,
        },
        packet::Packet,
        timeout::TimeoutHeight,
    },
    signer::Signer,
    timestamp::Timestamp,
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionId(IbcConnectionId);

impl FromStr for ConnectionId {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conn_id =
            IbcConnectionId::from_str(s).map_err(|error| ContractError::IbcDecodeError {
                error: error.to_string(),
            })?;

        Ok(Self(conn_id))
    }
}

impl ConnectionId {
    pub fn new(identifier: u64) -> Self {
        Self(IbcConnectionId::new(identifier))
    }

    /// Returns the static prefix to be used across all connection identifiers.
    pub fn prefix() -> &'static str {
        IbcConnectionId::prefix()
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    pub fn connection_id(&self) -> &IbcConnectionId {
        &self.0
    }
}

impl<'a> PrimaryKey<'a> for ConnectionId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = ();

    type SuperSuffix = ();
    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_str().as_bytes())]
    }
}
impl<'a> Prefixer<'a> for ConnectionId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for ConnectionId {
    type Output = ConnectionId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let connection_id = IbcConnectionId::from_str(&result).unwrap();
        Ok(ConnectionId(connection_id))
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelId(IbcChannelId);

impl<'a> PrimaryKey<'a> for ChannelId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for ChannelId {
    type Output = ChannelId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let port_id = IbcChannelId::from_str(&result).unwrap();
        Ok(ChannelId(port_id))
    }
}

impl ChannelId {
    /// function for create new channel id
    pub fn new(identifier: u64) -> Self {
        Self(IbcChannelId::new(identifier))
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn ibc_channel_id(&self) -> &IbcChannelId {
        &self.0
    }
}

impl From<IbcChannelId> for ChannelId {
    fn from(channel_id: IbcChannelId) -> Self {
        Self(channel_id)
    }
}

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortId(IbcPortId);

impl FromStr for PortId {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let port_id = IbcPortId::from_str(s).map_err(|error| ContractError::IbcDecodeError {
            error: error.to_string(),
        })?;

        Ok(Self(port_id))
    }
}

impl<'a> PrimaryKey<'a> for PortId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl PortId {
    /// Infallible creation of the well-known transfer port
    pub fn transfer() -> Self {
        Self(IbcPortId::transfer())
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn ibc_port_id(&self) -> &IbcPortId {
        &self.0
    }
}

impl Display for PortId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "{}", self.0)
    }
}

impl<'a> Prefixer<'a> for PortId {
    fn prefix(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for PortId {
    type Output = PortId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let port_id = IbcPortId::from_str(&result).unwrap();
        Ok(PortId(port_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleId(String);

impl ModuleId {
    pub fn new(s: String) -> Self {
        let ibc_module_id = IbcModuleId::from_str(&s).unwrap();
        Self(ibc_module_id.to_string())
    }
    pub fn module_id(&self) -> IbcModuleId {
        IbcModuleId::from_str(&self.0).unwrap()
    }
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> PrimaryKey<'a> for ModuleId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for ModuleId {
    type Output = ModuleId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let module_id = IbcModuleId::from_str(&result).unwrap();
        Ok(ModuleId(module_id.to_string()))
    }
}

impl From<IbcConnectionId> for ConnectionId {
    fn from(conn: IbcConnectionId) -> Self {
        ConnectionId(conn)
    }
}

impl From<IbcPortId> for PortId {
    fn from(port_id: IbcPortId) -> Self {
        PortId(port_id)
    }
}

impl From<IbcModuleId> for ModuleId {
    fn from(module: IbcModuleId) -> Self {
        ModuleId(module.to_string())
    }
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
pub struct OpenConfirmResponse {
    pub conn_id: String,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_prefix: Vec<u8>,
}

pub enum TimeoutMsgType {
    Timeout(MsgTimeout),
    TimeoutOnClose(MsgTimeoutOnClose),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketData {
    pub packet: Packet,
    pub signer: Signer,
    pub acknowledgement: Option<Acknowledgement>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PacketDataResponse {
    pub packet: PacketResponse,
    pub signer: Signer,
    pub acknowledgement: Option<Acknowledgement>,
}

#[cw_serde]
pub struct VerifyPacketData {
    pub height: String,
    pub prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub commitment_path: Vec<u8>,
    pub commitment: Vec<u8>,
}

impl PacketData {
    pub fn new(packet: Packet, signer: Signer, acknowledgement: Option<Acknowledgement>) -> Self {
        Self {
            packet,
            signer,
            acknowledgement,
        }
    }
}

#[cw_serde]
pub struct VerifyPacketAcknowledgement {
    pub height: String,
    pub prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub ack_path: Vec<u8>,
    pub ack: Vec<u8>,
}
