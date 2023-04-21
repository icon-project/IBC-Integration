use std::borrow::Cow;

use super::*;

#[cw_serde]
pub struct VerifyChannelState {
    pub proof_height: String,
    pub counterparty_prefix: Vec<u8>,
    pub proof: Vec<u8>,
    pub root: Vec<u8>,
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
#[derive(Debug, Serialize, Deserialize)]
pub struct PacketData {
    pub packet: Packet,
    pub signer: Signer,
    pub acknowledgement: Option<Acknowledgement>,
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
    // commitment key
    pub ack_path: Vec<u8>,
    // commitment byte
    pub ack: Vec<u8>,
}

pub enum TimeoutMsgType {
    Timeout(MsgTimeout),
    TimeoutOnClose(MsgTimeoutOnClose),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientId(IbcClientId);

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

    pub fn new(client_type: ClientType, counter: u64) -> Result<Self, CwErrors> {
        match IbcClientId::new(client_type.client_type(), counter) {
            Ok(result) => Ok(Self(result)),
            Err(err) => Err(CwErrors::FailedToCreateClientId {
                client_type: client_type,
                counter,
                validation_error: err,
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
        let data: Vec<&str> = value.as_str().split('-').collect();
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

#[derive(Debug, Clone, Serialize, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct PortId(IbcPortId);

impl FromStr for PortId {
    type Err = CwErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let port_id = IbcPortId::from_str(s).map_err(|error| CwErrors::DecodeError {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConnectionId(IbcConnectionId);

impl FromStr for ConnectionId {
    type Err = CwErrors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let conn_id = IbcConnectionId::from_str(s).map_err(|error| CwErrors::DecodeError {
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

use hex_buffer_serde::Hex;
use schemars::JsonSchema;

#[derive(JsonSchema)]
struct StringHex(());

impl Hex<Vec<u8>> for StringHex {
    type Error = hex::FromHexError;

    fn create_bytes(value: &Vec<u8>) -> Cow<'_, [u8]> {
       Cow::Borrowed(&value[..])
    }

    fn from_bytes(bytes: &[u8]) -> Result<Vec<u8>, Self::Error> {
        Ok(bytes.to_vec())
    }
}



#[cw_serde]
pub struct TestHex {
  #[serde(with = "StringHex")]
   pub  bytes: Vec<u8>,
}

#[cfg(test)]
mod tests{
    use super::TestHex;


    #[test]
    fn test_hex_serialize_deserialize(){
        let test= TestHex{
            bytes:hex::decode("deadbeef").unwrap()
        };
        let serialized = serde_json::to_value(&test).unwrap();
        assert_eq!("{\"bytes\":\"deadbeef\"}",serialized.to_string());
        let deserialized= serde_json::from_str::<TestHex>("{\"bytes\":\"deadbeef\"}").unwrap();
        assert_eq!(test,deserialized);
    }

}
