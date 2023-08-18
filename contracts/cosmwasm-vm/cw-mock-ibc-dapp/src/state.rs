use cw_storage_plus::Map;

use crate::types::config::Config;

use super::*;

/// These are constants defined in the `CwIbcConnection` struct that are used throughout the codebase.
pub const MAX_DATA_SIZE: u64 = 2048;
pub const MAX_ROLLBACK_SIZE: u64 = 1024;

pub const ACK_FAILURE_ID: u64 = 0;

pub const XCALL_HANDLE_MESSAGE_REPLY_ID: u64 = 1;
pub const XCALL_HANDLE_ERROR_REPLY_ID: u64 = 2;

pub const HOST_WRITE_ACKNOWLEDGEMENT_REPLY_ID: u64 = 3;
pub const HOST_SEND_MESSAGE_REPLY_ID: u64 = 4;

/// The `IbcConfig` struct represents a configuration for inter-blockchain communication with a source
/// and destination endpoint, and a sequence number.
///
/// Properties:
///
/// * `sequence`: The `sequence` property is an unsigned 128-bit integer that represents the sequence
/// number of the IBC transaction. It is used to ensure that transactions are processed in the correct
/// order and to prevent replay attacks.
/// * `src`: `src` is a property of the `IbcConfig` struct that represents the source endpoint of an
/// Inter-Blockchain Communication (IBC) transaction. An IBC endpoint is a unique identifier for a
/// blockchain network that can send or receive IBC packets. It typically includes information such as
/// the chain ID
/// * `dst`: `dst` is a property of the `IbcConfig` struct and represents the destination endpoint of an
/// Inter-Blockchain Communication (IBC) transaction. It is of type `IbcEndpoint`, which likely contains
/// information such as the chain ID, address, and port of the destination chain.
#[cw_serde]
pub struct IbcConfig {
    sequence: u128,
    src: CwEndPoint,
    dst: CwEndPoint,
}

/// This is an implementation block for the `IbcConfig` struct, defining several methods that can be
/// called on instances of the struct.
impl IbcConfig {
    pub fn new(src: CwEndPoint, dst: CwEndPoint) -> Self {
        Self {
            src,
            dst,
            sequence: u128::default(),
        }
    }

    pub fn src_endpoint(&self) -> &CwEndPoint {
        &self.src
    }

    pub fn dst_endpoint(&self) -> &CwEndPoint {
        &self.dst
    }

    pub fn sequence(&self) -> u128 {
        self.sequence
    }

    pub fn next_sequence(&self) -> Option<u128> {
        self.sequence.checked_add(1)
    }
}

/// This is a Rust struct representing a Call Service with various fields such as last sequence number,
/// owner, admin, message request, requests, IBC configuration, fee handler, fee, IBC host, and timeout
/// height.
///
/// Properties:
/// /// * `owner`: The `owner` property is a `String` type field that holds the address of the owner of the
/// `CwIbcConnection` struct.
///
///  * `admin`: `admin` is a field of type `Item<'a, String>` in the `CwIbcConnection` struct. It is a
/// reference to a string that represents the address of the admin of the call service. The `Item` type
/// is a wrapper around a reference to a value of a
///
/// * `ibc_config`: This property is of type `Map<'a, NetId,IbcConfig>` and represents the IBC configuration
/// for the given network. It is likely used to define the parameters and settings for inter-blockchain
/// communication.
///
///  * `ibc_host`: `ibc_host` is a field of type `Item<'a, Addr>` in a struct called `CwIbcConnection`. It
/// is likely used to store the address of the IBC host that the `CwIbcConnection` interacts with. The
/// `Addr` type likely represents a network
///
/// * `xcall_host`: `xcall_host` is a field of type `Item<'a, Addr>` in a struct called `CwIbcConnection`. It
/// is likely used to store the address of the Xcall App that the `CwIbcConnection` interacts with.
///
///
/// * `configured_networks`: `configured_networks` stores relation between connectionId ,
///  counterpartyPortId and counterpartyNetId
///
/// * `connection_configs`: `connection_configs` stores ConnectionConfig for given connectionId.
///
/// * `channel_configs`: `channel_configs` stores ChannelConfig for given channel.
///
/// * `network_fees`: `network_fees` stores NetworkFeesInfo for given network id.
///
/// * `unclaimed_packet_fees`: `unclaimed_packet_fees` acculumulated packet fees for given network and relayer.
///
/// * `unclaimed_ack_fees`: `unclaimed_ack_fees` stores ack fee for given packet by networkId.
///
/// * `incoming_packets`: `incoming_packets` stores incoming packets for reference.
///
/// * `outgoing_packets`: `outgoing_packets` stores outgoing packets for reference.
///

pub struct CwIbcConnection<'a> {
    owner: Item<'a, String>,
    config: Item<'a, Config>,
    admin: Item<'a, String>,
    ibc_config: Item<'a, IbcConfig>,
    ibc_host: Item<'a, Addr>,
    received_packets: Map<'a, u64, CwPacket>,
}

impl<'a> Default for CwIbcConnection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcConnection<'a> {
    pub fn new() -> Self {
        Self {
            owner: Item::new(StorageKey::Owner.as_str()),
            config: Item::new(StorageKey::Config.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            ibc_config: Item::new(StorageKey::IbcConfig.as_str()),
            ibc_host: Item::new(StorageKey::IbcHost.as_str()),
            received_packets: Map::new(StorageKey::ReceivedPackets.as_str()),
        }
    }

    pub fn owner(&self) -> &Item<'a, String> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, String> {
        &self.admin
    }

    pub fn get_ibc_config(&self, store: &dyn Storage) -> Result<IbcConfig, ContractError> {
        self.ibc_config.load(store).map_err(ContractError::Std)
    }

    pub fn store_ibc_config(
        &self,
        store: &mut dyn Storage,
        config: &IbcConfig,
    ) -> Result<(), ContractError> {
        self.ibc_config
            .save(store, config)
            .map_err(ContractError::Std)
    }
    pub fn set_ibc_host(
        &self,
        store: &mut dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        self.ibc_host
            .save(store, &address)
            .map_err(ContractError::Std)
    }
    pub fn get_ibc_host(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        self.ibc_host.load(store).map_err(ContractError::Std)
    }

    pub fn store_received_packet(
        &self,
        store: &mut dyn Storage,
        seq: u64,
        packet: CwPacket,
    ) -> Result<(), ContractError> {
        self.received_packets
            .save(store, seq, &packet)
            .map_err(ContractError::Std)
    }
    pub fn get_received_packet(
        &self,
        store: &dyn Storage,
        seq: u64,
    ) -> Result<CwPacket, ContractError> {
        self.received_packets
            .load(store, seq)
            .map_err(ContractError::Std)
    }
}
