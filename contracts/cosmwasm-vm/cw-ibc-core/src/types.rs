use ibc::{
    core::ics04_channel::{
        msgs::{timeout::MsgTimeout, timeout_on_close::MsgTimeoutOnClose},
        packet::Packet,
        timeout::TimeoutHeight,
    },
    signer::Signer,
    timestamp::Timestamp,
};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(IbcClientId);

impl Default for ClientId {
    fn default() -> Self {
        Self(IbcClientId::default())
    }
}
impl From<IbcClientId> for ClientId {
    fn from(value: IbcClientId) -> Self {
        Self(value)
    }
}
impl FromStr for ClientId {
    type Err = ibc::core::ics24_host::error::ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = IbcClientId::from_str(s)?;
        Ok(Self(r))
    }
}
impl ClientId {
    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Get this identifier as a borrowed byte slice
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    pub fn new(client_type: ClientType, counter: u64) -> Result<Self, ContractError> {
        match IbcClientId::new(client_type.client_type(), counter) {
            Ok(result) => Ok(Self(result)),
            Err(error) => Err(ContractError::IbcClientError {
                error: ClientError::ClientIdentifierConstructor {
                    client_type: client_type.client_type(),
                    counter,
                    validation_error: error,
                },
            }),
        }
    }

    pub fn ibc_client_id(&self) -> &IbcClientId {
        &self.0
    }

    pub fn from(client_id: IbcClientId) -> Self {
        Self(client_id)
    }
}

impl<'a> PrimaryKey<'a> for ClientId {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_bytes())]
    }
}

impl KeyDeserialize for ClientId {
    type Output = ClientId;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_id = IbcClientId::from_str(&result).unwrap();
        Ok(ClientId(client_id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientType(IbcClientType);

impl ClientType {
    pub fn new(cleint_type: String) -> ClientType {
        ClientType(IbcClientType::new(cleint_type))
    }
    pub fn client_type(&self) -> IbcClientType {
        self.0.clone()
    }

    /// Get this identifier as a borrowed `&str`
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<IbcClientType> for ClientType {
    fn from(value: IbcClientType) -> Self {
        Self(value)
    }
}

impl From<ClientId> for ClientType {
    fn from(value: ClientId) -> Self {
        let data: Vec<&str> = value.as_str().split("-").collect();
        ClientType::new(data[0].to_string())
    }
}

impl<'a> PrimaryKey<'a> for ClientType {
    type Prefix = ();

    type SubPrefix = ();

    type Suffix = Self;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.0.as_str().as_bytes())]
    }
}

impl KeyDeserialize for ClientType {
    type Output = ClientType;

    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let client_type = IbcClientType::new(result);
        Ok(ClientType(client_type))
    }
}

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

    pub fn default() -> Self {
        Self(IbcConnectionId::default())
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChannelId(IbcChannelId);

impl Default for ChannelId {
    fn default() -> Self {
        Self(IbcChannelId::default())
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortId(IbcPortId);

impl Default for PortId {
    fn default() -> Self {
        Self(IbcPortId::default())
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

#[cw_serde]
pub struct VerifyConnectionState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    proof: Vec<u8>,
    root: Vec<u8>,
    counterparty_conn_end_path: Vec<u8>,
    expected_counterparty_connection_end: Vec<u8>,
}
impl VerifyConnectionState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        counterparty_conn_end_path: Vec<u8>,
        expected_counterparty_connection_end: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            proof,
            root,
            counterparty_conn_end_path,
            expected_counterparty_connection_end,
        }
    }
}

#[cw_serde]
pub struct VerifyClientFullState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    client_state_proof: Vec<u8>,
    root: Vec<u8>,
    client_state_path: Vec<u8>,
    expected_client_state: Vec<u8>,
}
impl VerifyClientFullState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        client_state_proof: Vec<u8>,
        root: Vec<u8>,
        client_state_path: Vec<u8>,
        expected_client_state: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            client_state_proof,
            root,
            client_state_path,
            expected_client_state,
        }
    }
}

#[cw_serde]
pub struct VerifyClientConsesnusState {
    proof_height: String,
    counterparty_prefix: Vec<u8>,
    consensus_state_proof: Vec<u8>,
    root: Vec<u8>,
    conesenus_state_path: Vec<u8>,
    expected_conesenus_state: Vec<u8>,
}

impl VerifyClientConsesnusState {
    pub fn new(
        proof_height: String,
        counterparty_prefix: Vec<u8>,
        consensus_state_proof: Vec<u8>,
        root: Vec<u8>,
        conesenus_state_path: Vec<u8>,
        expected_conesenus_state: Vec<u8>,
    ) -> Self {
        Self {
            proof_height,
            counterparty_prefix,
            consensus_state_proof,
            root,
            conesenus_state_path,
            expected_conesenus_state,
        }
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
#[cw_serde]
pub struct VerifyChannelState {
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
    pub counterparty_chan_end_path: Vec<u8>,
    pub expected_counterparty_channel_end: Vec<u8>,
}

#[cw_serde]
pub enum LightClientPacketMessage {
    VerifyPacketReceiptAbsence {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        receipt_path: Vec<u8>,
        packet_data: Vec<u8>,
    },

    VerifyNextSequenceRecv {
        height: String,
        prefix: Vec<u8>,
        proof: Vec<u8>,
        root: Vec<u8>,
        seq_recv_path: Vec<u8>,
        sequence: u64,
        packet_data: Vec<u8>,
    },
}

pub enum TimeoutMsgType {
    Timeout(MsgTimeout),
    TimeoutOnClose(MsgTimeoutOnClose),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PacketData {
    pub packet: Packet,
    pub signer: Signer,
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
            data: data,
            timeout_height_on_b: packet.timeout_height_on_b,
            timeout_timestamp_on_b: packet.timeout_timestamp_on_b,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PacketDataResponse {
    pub packet: PacketResponse,
    pub signer: Signer,
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
    pub fn new(packet: Packet, signer: Signer) -> Self {
        Self { packet, signer }
    }
}
