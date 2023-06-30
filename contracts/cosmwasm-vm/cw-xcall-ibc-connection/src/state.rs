use cw_common::xcall_types::network_address::NetId;
use cw_storage_plus::Map;

use crate::types::{
    channel_config::ChannelConfig, config::Config, connection_config::ConnectionConfig,
    network_fees::NetworkFees,
};

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
///
/// * `last_sequence_no`: A field of type `Item` that stores the last sequence number used in a call
/// service message.
/// * `last_request_id`: This is a field of type `Item<'a, u128>` which stores the last request ID used
/// by the `CwIbcConnection` struct. The `Item` type is a wrapper around a value that can be stored in a
/// persistent storage, and the `'a` lifetime parameter indicates that
/// * `owner`: The `owner` property is a `String` type field that holds the address of the owner of the
/// `CwIbcConnection` struct.
/// * `admin`: `admin` is a field of type `Item<'a, String>` in the `CwIbcConnection` struct. It is a
/// reference to a string that represents the address of the admin of the call service. The `Item` type
/// is a wrapper around a reference to a value of a
/// * `message_request`: A map that stores CallServiceMessageRequest structs with a u128 key.
/// * `requests`: `requests` is a `Map` that stores `CallRequest` objects with a `u128` key. This map is
/// used to keep track of all the call requests made by the users of the `CwIbcConnection` struct. The
/// `u128` key is used to uniquely identify
/// * `ibc_config`: This property is of type `Item<'a, IbcConfig>` and represents the IBC configuration
/// for the call service. It is likely used to define the parameters and settings for inter-blockchain
/// communication.
/// * `fee_handler`: The `fee_handler` property is an `Item` that holds a `String` value. It likely
/// represents the address or identifier of the entity responsible for handling fees associated with the
/// `CwIbcConnection` struct.
/// * `fee`: `fee` is a field of type `Item<'a, u128>` in the `CwIbcConnection` struct. It is a reference
/// to an item of type `u128` that represents the fee for the call service. The `Item` type is a wrapper
/// around a reference to
/// * `ibc_host`: `ibc_host` is a field of type `Item<'a, Addr>` in a struct called `CwIbcConnection`. It
/// is likely used to store the address of the IBC host that the `CwIbcConnection` interacts with. The
/// `Addr` type likely represents a network
/// * `timeout_height`: The `timeout_height` property is an `Item` that stores the timeout height for
/// the Call Service. This is the block height at which the Call Service will stop processing requests
/// if they have not been completed.
pub struct CwIbcConnection<'a> {
    owner: Item<'a, String>,
    config: Item<'a, Config>,
    admin: Item<'a, String>,
    ibc_config: Map<'a, NetId, IbcConfig>,
    ibc_host: Item<'a, Addr>,
    xcall_host: Item<'a, Addr>,
    configured_networks: Map<'a, (String, String), NetId>,
    connection_configs: Map<'a, String, ConnectionConfig>,
    channel_configs: Map<'a, String, ChannelConfig>,
    network_fees: Map<'a, NetId, NetworkFees>,
    unclaimed_packet_fees: Map<'a, (String, String), u128>,
    unclaimed_ack_fees: Map<'a, (String, u64), u128>,
    incoming_packets: Map<'a, (String, i64), u64>,
    outgoing_packets: Map<'a, (String, u64), i64>,
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
            ibc_config: Map::new(StorageKey::IbcConfig.as_str()),
            ibc_host: Item::new(StorageKey::IbcHost.as_str()),
            xcall_host: Item::new(StorageKey::XCallHost.as_str()),
            configured_networks: Map::new(StorageKey::ConfiguredNetworks.as_str()),
            channel_configs: Map::new(StorageKey::ChannelConfigs.as_str()),
            connection_configs: Map::new(StorageKey::ConnectionConfigs.as_str()),
            network_fees: Map::new(StorageKey::NetworkFees.as_str()),
            unclaimed_packet_fees: Map::new(StorageKey::UnclaimedPacketFees.as_str()),
            unclaimed_ack_fees: Map::new(StorageKey::UnClaimedAckFees.as_str()),
            incoming_packets: Map::new(StorageKey::IncomingPackets.as_str()),
            outgoing_packets: Map::new(StorageKey::IncomingPackets.as_str()),
        }
    }

    pub fn owner(&self) -> &Item<'a, String> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, String> {
        &self.admin
    }

    pub fn get_config(&self, store: &dyn Storage) -> Result<Config, ContractError> {
        self.config.load(store).map_err(ContractError::Std)
    }

    pub fn store_config(
        &self,
        store: &mut dyn Storage,

        config: &Config,
    ) -> Result<(), ContractError> {
        self.config.save(store, config).map_err(ContractError::Std)
    }

    pub fn get_ibc_config(
        &self,
        store: &dyn Storage,
        nid: &NetId,
    ) -> Result<IbcConfig, ContractError> {
        self.ibc_config
            .load(store, nid.to_owned())
            .map_err(ContractError::Std)
    }

    pub fn store_ibc_config(
        &self,
        store: &mut dyn Storage,
        nid: &NetId,
        config: &IbcConfig,
    ) -> Result<(), ContractError> {
        self.ibc_config
            .save(store, nid.to_owned(), config)
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

    pub fn set_xcall_host(
        &self,
        store: &mut dyn Storage,
        address: Addr,
    ) -> Result<(), ContractError> {
        self.xcall_host
            .save(store, &address)
            .map_err(ContractError::Std)
    }
    pub fn get_xcall_host(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        self.xcall_host.load(store).map_err(ContractError::Std)
    }

    pub fn get_channel_config(
        &self,
        store: &dyn Storage,
        channel: &str,
    ) -> Result<ChannelConfig, ContractError> {
        self.channel_configs
            .load(store, channel.to_owned())
            .map_err(ContractError::Std)
    }

    pub fn store_channel_config(
        &self,
        store: &mut dyn Storage,
        channel: &str,
        channel_config: &ChannelConfig,
    ) -> Result<(), ContractError> {
        self.channel_configs
            .save(store, channel.to_owned(), channel_config)
            .map_err(ContractError::Std)
    }

    pub fn get_connection_config(
        &self,
        store: &dyn Storage,
        connection_id: &str,
    ) -> Result<ConnectionConfig, ContractError> {
        self.connection_configs
            .load(store, connection_id.to_owned())
            .map_err(ContractError::Std)
    }

    pub fn store_connection_config(
        &self,
        store: &mut dyn Storage,
        connection_id: &str,
        config: &ConnectionConfig,
    ) -> Result<(), ContractError> {
        self.connection_configs
            .save(store, connection_id.to_string(), config)
            .map_err(ContractError::Std)
    }

    pub fn get_counterparty_nid(
        &self,
        store: &dyn Storage,
        connection_id: &str,
        port_id: &str,
    ) -> Result<NetId, ContractError> {
        self.configured_networks
            .load(store, (connection_id.to_string(), port_id.to_string()))
            .map_err(ContractError::Std)
    }

    pub fn store_counterparty_nid(
        &self,
        store: &mut dyn Storage,
        connection_id: &str,
        port_id: &str,
        nid: &NetId,
    ) -> Result<(), ContractError> {
        self.configured_networks
            .save(store, (connection_id.to_owned(), port_id.to_owned()), nid)
            .map_err(ContractError::Std)
    }

    pub fn get_network_fees(
        &self,
        store: &dyn Storage,
        nid: NetId,
    ) -> Result<NetworkFees, ContractError> {
        self.network_fees
            .load(store, nid)
            .map_err(ContractError::Std)
    }

    pub fn store_network_fees(
        &self,
        store: &mut dyn Storage,
        nid: NetId,
        network_fees: &NetworkFees,
    ) -> Result<(), ContractError> {
        self.network_fees
            .save(store, nid, network_fees)
            .map_err(ContractError::Std)
    }

    pub fn add_unclaimed_packet_fees(
        &self,
        store: &mut dyn Storage,
        nid: &NetId,
        address: &str,
        value: u128,
    ) -> Result<(), ContractError> {
        let mut acc = self
            .unclaimed_packet_fees
            .load(store, (nid.to_string(), address.to_string()))
            .unwrap_or(0);
        acc += value;
        self.unclaimed_packet_fees
            .save(store, (nid.to_string(), address.to_owned()), &value)
            .map_err(ContractError::Std)
    }

    pub fn get_unclaimed_packet_fee(
        &self,
        store: &dyn Storage,
        nid: &NetId,
        address: &str,
    ) -> Result<u128, ContractError> {
        self.unclaimed_packet_fees
            .load(store, (nid.to_string(), address.to_owned()))
            .map_err(ContractError::Std)
    }

    pub fn reset_unclaimed_packet_fees(
        &self,
        store: &mut dyn Storage,
        nid: &NetId,
        address: &str,
    ) -> Result<(), ContractError> {
        self.unclaimed_packet_fees
            .save(store, (nid.to_string(), address.to_owned()), &0_u128)
            .map_err(ContractError::Std)
    }

    pub fn add_unclaimed_ack_fees(
        &self,
        store: &mut dyn Storage,
        nid: &NetId,
        sequence: u64,
        value: u128,
    ) -> Result<(), ContractError> {
        let mut acc = self
            .unclaimed_ack_fees
            .load(store, (nid.to_string(), sequence))
            .unwrap_or(0);
        acc += value;
        self.unclaimed_ack_fees
            .save(store, (nid.to_string(), sequence), &value)
            .map_err(ContractError::Std)
    }

    pub fn get_unclaimed_ack_fee(
        &self,
        store: &dyn Storage,
        nid: &str,
        sequence: u64,
    ) -> Result<u128, ContractError> {
        self.unclaimed_ack_fees
            .load(store, (nid.to_owned(), sequence))
            .map_err(ContractError::Std)
    }

    pub fn get_denom(&self, store: &dyn Storage) -> Result<String, ContractError> {
        let config = self.get_config(store)?;
        Ok(config.denom)
    }

    pub fn get_port(&self, store: &dyn Storage) -> Result<String, ContractError> {
        let config = self.get_config(store)?;
        Ok(config.port_id)
    }

    pub fn reset_unclaimed_ack_fees(
        &self,
        store: &mut dyn Storage,
        nid: &str,
        sequence: u64,
    ) -> Result<(), ContractError> {
        self.unclaimed_ack_fees
            .save(store, (nid.to_owned(), sequence), &0_u128)
            .map_err(ContractError::Std)
    }

    pub fn get_incoming_packet_sequence(
        &self,
        store: &dyn Storage,
        channel_id: &str,
        sn: i64,
    ) -> Result<u64, ContractError> {
        self.incoming_packets
            .load(store, (channel_id.to_owned(), sn))
            .map_err(ContractError::Std)
    }
    pub fn remove_incoming_packet_sequence(
        &self,
        store: &mut dyn Storage,
        channel_id: &str,
        sequence: i64,
    ) {
        self.incoming_packets
            .remove(store, (channel_id.to_owned(), sequence))
    }

    pub fn store_incoming_packet_sequence(
        &self,
        store: &mut dyn Storage,
        channel_id: &str,
        sn: i64,
        seq: u64,
    ) -> Result<(), ContractError> {
        self.incoming_packets
            .save(store, (channel_id.to_owned(), sn), &seq)
            .map_err(ContractError::Std)
    }

    pub fn get_outgoing_packet_sn(
        &self,
        store: &dyn Storage,
        channel_id: &str,
        sequence: u64,
    ) -> Result<i64, ContractError> {
        self.outgoing_packets
            .load(store, (channel_id.to_owned(), sequence))
            .map_err(ContractError::Std)
    }

    pub fn remove_outgoing_packet_sn(
        &self,
        store: &mut dyn Storage,
        channel_id: &str,
        sequence: u64,
    ) {
        self.outgoing_packets
            .remove(store, (channel_id.to_owned(), sequence))
    }

    pub fn store_outgoing_packet_sn(
        &self,
        store: &mut dyn Storage,
        channel_id: &str,
        sequence: u64,
        sn: i64,
    ) -> Result<(), ContractError> {
        self.outgoing_packets
            .save(store, (channel_id.to_owned(), sequence), &sn)
            .map_err(ContractError::Std)
    }
}
