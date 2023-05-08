use super::*;

/// These are constants defined in the `CwIbcConnection` struct that are used throughout the codebase.
pub const MAX_DATA_SIZE: u64 = 2048;
pub const MAX_ROLLBACK_SIZE: u64 = 1024;
pub const ACK_FAILURE_ID: u64 = 3;

pub const XCALL_FORWARD_REPLY_ID: u64 = 4;


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
    last_sequence_no: Item<'a, u128>,
    last_request_id: Item<'a, u128>,
    owner: Item<'a, String>,
    admin: Item<'a, String>,
    message_request: Map<'a, u128, Vec<u8>>,
    ibc_config: Item<'a, IbcConfig>,
    fee_handler: Item<'a, String>,
    fee: Item<'a, u128>,
    ibc_host: Item<'a, Addr>,
    xcall_host: Item<'a, Addr>,
    timeout_height: Item<'a, u64>,
}

impl<'a> Default for CwIbcConnection<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcConnection<'a> {
    pub fn new() -> Self {
        Self {
            last_sequence_no: Item::new(StorageKey::SequenceNo.as_str()),
            last_request_id: Item::new(StorageKey::RequestNo.as_str()),
            owner: Item::new(StorageKey::Owner.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            message_request: Map::new(StorageKey::MessageRequest.as_str()),
            ibc_config: Item::new(StorageKey::IbcConfig.as_str()),
            fee_handler: Item::new(StorageKey::FeeHandler.as_str()),
            fee: Item::new(StorageKey::Fee.as_str()),
            ibc_host: Item::new(StorageKey::IbcHost.as_str()),
            timeout_height: Item::new(StorageKey::TimeoutHeight.as_str()),
            xcall_host: Item::new(StorageKey::XCallHost.as_str()),
        }
    }

    pub fn last_sequence_no(&self) -> &Item<'a, u128> {
        &self.last_sequence_no
    }

    pub fn last_request_id(&self) -> &Item<'a, u128> {
        &self.last_request_id
    }

    pub fn owner(&self) -> &Item<'a, String> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, String> {
        &self.admin
    }

    pub fn message_request(&self) -> &Map<'a, u128, Vec<u8>> {
        &self.message_request
    }

    pub fn ibc_config(&self) -> &Item<'a, IbcConfig> {
        &self.ibc_config
    }

    pub fn fee_handler(&self) -> &Item<'a, String> {
        &self.fee_handler
    }
    pub fn fee(&self) -> &Item<'a, u128> {
        &self.fee
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
    pub fn set_timeout_height(
        &self,
        store: &mut dyn Storage,
        timeout_height: u64,
    ) -> Result<(), ContractError> {
        self.timeout_height
            .save(store, &timeout_height)
            .map_err(ContractError::Std)
    }
    pub fn get_timeout_height(&self, store: &dyn Storage) -> u64 {
        self.timeout_height.load(store).unwrap_or(0)
    }
}
