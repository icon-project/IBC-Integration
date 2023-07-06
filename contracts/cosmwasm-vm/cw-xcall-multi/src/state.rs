use cosmwasm_std::Order;
use cw_common::xcall_types::network_address::NetId;
use cw_storage_plus::{KeyDeserialize, PrimaryKey};
use serde::de::DeserializeOwned;

use crate::types::config::Config;

use super::*;

/// These are constants defined in the `CwCallService` struct that are used throughout the codebase.
pub const MAX_DATA_SIZE: u64 = 2048;
pub const MAX_ROLLBACK_SIZE: u64 = 1024;
pub const EXECUTE_CALL_ID: u64 = 0;
pub const EXECUTE_ROLLBACK_ID: u64 = 1;
pub const ACK_FAILURE_ID: u64 = 3;
pub const SEND_CALL_MESSAGE_REPLY_ID: u64 = 2;

/// This is a Rust struct representing a Call Service with various fields such as last sequence number,
/// owner, admin, message request, requests, IBC configuration, fee handler, fee, IBC host, and timeout
/// height.
///
/// Properties:
///
/// * `last_sequence_no`: A field of type `Item` that stores the last sequence number used in a call
/// service message.
/// * `last_request_id`: This is a field of type `Item<'a, u128>` which stores the last request ID used
/// by the `CwCallService` struct. The `Item` type is a wrapper around a value that can be stored in a
/// persistent storage, and the `'a` lifetime parameter indicates that
/// * `owner`: The `owner` property is a `String` type field that holds the address of the owner of the
/// `CwCallService` struct.
/// * `admin`: `admin` is a field of type `Item<'a, String>` in the `CwCallService` struct. It is a
/// reference to a string that represents the address of the admin of the call service. The `Item` type
/// is a wrapper around a reference to a value of a
/// * `message_request`: A map that stores CallServiceMessageRequest structs with a u128 key.
/// * `requests`: `requests` is a `Map` that stores `CallRequest` objects with a `u128` key. This map is
/// used to keep track of all the call requests made by the users of the `CwCallService` struct. The
/// `u128` key is used to uniquely identify
/// * `ibc_config`: This property is of type `Item<'a, IbcConfig>` and represents the IBC configuration
/// for the call service. It is likely used to define the parameters and storetings for inter-blockchain
/// communication.
/// * `fee_handler`: The `fee_handler` property is an `Item` that holds a `String` value. It likely
/// represents the address or identifier of the entity responsible for handling fees associated with the
/// `CwCallService` struct.
/// * `fee`: `fee` is a field of type `Item<'a, u128>` in the `CwCallService` struct. It is a reference
/// to an item of type `u128` that represents the fee for the call service. The `Item` type is a wrapper
/// around a reference to
/// * `ibc_host`: `ibc_host` is a field of type `Item<'a, Addr>` in a struct called `CwCallService`. It
/// is likely used to store the address of the IBC host that the `CwCallService` interacts with. The
/// `Addr` type likely represents a network
/// * `timeout_height`: The `timeout_height` property is an `Item` that stores the timeout height for
/// the Call Service. This is the block height at which the Call Service will stop processing requests
/// if they have not been completed.
pub struct CwCallService<'a> {
    sn: Item<'a, u128>,
    config: Item<'a, Config>,
    last_request_id: Item<'a, u128>,
    owner: Item<'a, String>,
    admin: Item<'a, String>,
    proxy_request: Map<'a, u128, CallServiceMessageRequest>,
    call_requests: Map<'a, u128, CallRequest>,
    fee_handler: Item<'a, String>,
    protocol_fee: Item<'a, u128>,
    default_connections: Map<'a, NetId, Addr>,
    pending_requests: Map<'a, (Vec<u8>, String), bool>,
    pending_responses: Map<'a, (Vec<u8>, String), bool>,
    successful_responses: Map<'a, u128, bool>,
    execute_request_id: Item<'a, u128>,
    execute_rollback_id: Item<'a, u128>,
}

impl<'a> Default for CwCallService<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwCallService<'a> {
    pub fn new() -> Self {
        Self {
            sn: Item::new(StorageKey::Sn.as_str()),
            last_request_id: Item::new(StorageKey::RequestNo.as_str()),
            owner: Item::new(StorageKey::Owner.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            proxy_request: Map::new(StorageKey::MessageRequest.as_str()),
            call_requests: Map::new(StorageKey::Requests.as_str()),
            fee_handler: Item::new(StorageKey::FeeHandler.as_str()),
            protocol_fee: Item::new(StorageKey::ProtocolFee.as_str()),
            default_connections: Map::new(StorageKey::DefaultConnections.as_str()),
            pending_requests: Map::new(StorageKey::PendingRequests.as_str()),
            pending_responses: Map::new(StorageKey::PendingResponses.as_str()),
            successful_responses: Map::new(StorageKey::SuccessfulResponses.as_str()),
            config: Item::new(StorageKey::Config.as_str()),
            execute_request_id: Item::new(StorageKey::ExecuteReqId.as_str()),
            execute_rollback_id: Item::new(StorageKey::ExecuteRollbackId.as_str()),
        }
    }

    pub fn get_next_sn(&self, store: &mut dyn Storage) -> Result<u128, ContractError> {
        let mut sn = self.sn.load(store).unwrap_or(0);
        sn += 1;
        self.sn.save(store, &sn)?;
        Ok(sn)
    }

    pub fn get_current_sn(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        self.sn.load(store).map_err(ContractError::Std)
    }

    pub fn sn(&self) -> &Item<'a, u128> {
        &self.sn
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

    pub fn last_request_id(&self) -> &Item<'a, u128> {
        &self.last_request_id
    }

    pub fn store_execute_request_id(
        &self,
        store: &mut dyn Storage,
        req_id: u128,
    ) -> Result<(), ContractError> {
        self.execute_request_id
            .save(store, &req_id)
            .map_err(ContractError::Std)
    }

    pub fn get_execute_request_id(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        self.execute_request_id
            .load(store)
            .map_err(ContractError::Std)
    }

    pub fn store_execute_rollback_id(
        &self,
        store: &mut dyn Storage,
        req_id: u128,
    ) -> Result<(), ContractError> {
        self.execute_rollback_id
            .save(store, &req_id)
            .map_err(ContractError::Std)
    }

    pub fn get_execute_rollback_id(&self, store: &dyn Storage) -> Result<u128, ContractError> {
        self.execute_rollback_id
            .load(store)
            .map_err(ContractError::Std)
    }

    pub fn owner(&self) -> &Item<'a, String> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, String> {
        &self.admin
    }

    pub fn get_proxy_request(
        &self,
        store: &dyn Storage,
        id: u128,
    ) -> Result<CallServiceMessageRequest, ContractError> {
        self.proxy_request
            .load(store, id)
            .map_err(ContractError::Std)
    }

    pub fn store_proxy_request(
        &self,
        store: &mut dyn Storage,
        id: u128,
        request: &CallServiceMessageRequest,
    ) -> Result<(), ContractError> {
        self.proxy_request
            .save(store, id, request)
            .map_err(ContractError::Std)
    }

    pub fn remove_proxy_request(&self, store: &mut dyn Storage, id: u128) {
        self.proxy_request.remove(store, id)
    }

    pub fn contains_proxy_request(
        &self,
        store: &dyn Storage,
        request_id: u128,
    ) -> Result<(), ContractError> {
        match self.proxy_request.has(store, request_id) {
            true => Ok(()),
            false => Err(ContractError::InvalidRequestId { id: request_id }),
        }
    }

    pub fn get_call_request(
        &self,
        store: &dyn Storage,
        id: u128,
    ) -> Result<CallRequest, ContractError> {
        self.call_requests
            .load(store, id)
            .map_err(ContractError::Std)
    }

    pub fn remove_call_request(&self, store: &mut dyn Storage, id: u128) {
        self.call_requests.remove(store, id)
    }

    pub fn store_call_request(
        &self,
        store: &mut dyn Storage,
        id: u128,
        request: &CallRequest,
    ) -> Result<(), ContractError> {
        self.call_requests
            .save(store, id, request)
            .map_err(ContractError::Std)
    }

    pub fn fee_handler(&self) -> &Item<'a, String> {
        &self.fee_handler
    }

    pub fn store_default_connection(
        &self,
        store: &mut dyn Storage,
        nid: NetId,
        address: Addr,
    ) -> Result<(), ContractError> {
        self.default_connections
            .save(store, nid, &address)
            .map_err(ContractError::Std)
    }
    pub fn get_default_connection(
        &self,
        store: &dyn Storage,
        nid: NetId,
    ) -> Result<Addr, ContractError> {
        self.default_connections
            .load(store, nid)
            .map_err(ContractError::Std)
    }

    pub fn get_pending_requests_by_hash(
        &self,
        store: &dyn Storage,
        hash: Vec<u8>,
    ) -> Result<Vec<(String, bool)>, ContractError> {
        self.get_by_prefix(store, &self.pending_requests, hash)
    }

    pub fn remove_pending_request_by_hash(
        &self,
        store: &mut dyn Storage,
        hash: Vec<u8>,
    ) -> Result<(), ContractError> {
        self.remove_by_prefix(store, &self.pending_requests, hash)
    }

    pub fn save_pending_requests(
        &self,
        store: &mut dyn Storage,
        hash: Vec<u8>,
        caller: String,
    ) -> Result<(), ContractError> {
        self.pending_requests
            .save(store, (hash, caller), &true)
            .map_err(ContractError::Std)
    }

    pub fn get_pending_responses_by_hash(
        &self,
        store: &dyn Storage,
        hash: Vec<u8>,
    ) -> Result<Vec<(String, bool)>, ContractError> {
        self.get_by_prefix(store, &self.pending_responses, hash)
    }

    pub fn remove_pending_responses_by_hash(
        &self,
        store: &mut dyn Storage,
        hash: Vec<u8>,
    ) -> Result<(), ContractError> {
        self.remove_by_prefix(store, &self.pending_responses, hash)
    }

    pub fn save_pending_responses(
        &self,
        store: &mut dyn Storage,
        hash: Vec<u8>,
        caller: String,
    ) -> Result<(), ContractError> {
        self.pending_responses
            .save(store, (hash, caller), &true)
            .map_err(ContractError::Std)
    }

    pub fn get_all_connections(&self, store: &dyn Storage) -> Result<Vec<String>, ContractError> {
        let res = self.get_all_values::<NetId, Addr>(store, &self.default_connections)?;
        let addresses: Vec<String> = res.into_iter().map(|a| a.to_string()).collect();
        Ok(addresses)
    }

    fn get_by_prefix(
        &self,
        store: &dyn Storage,
        map: &Map<(Vec<u8>, String), bool>,
        hash: Vec<u8>,
    ) -> Result<Vec<(String, bool)>, ContractError> {
        let requests: StdResult<Vec<(String, bool)>> = map
            .prefix(hash)
            .range(store, None, None, cosmwasm_std::Order::Ascending)
            .collect();
        requests.map_err(ContractError::Std)
    }

    fn remove_by_prefix(
        &self,
        store: &mut dyn Storage,
        map: &Map<(Vec<u8>, String), bool>,
        hash: Vec<u8>,
    ) -> Result<(), ContractError> {
        let keys: StdResult<Vec<String>> = map
            .prefix(hash.clone())
            .keys(store, None, None, cosmwasm_std::Order::Ascending)
            .collect();
        let keys = keys.map_err(ContractError::Std)?;
        for key in keys {
            self.pending_requests.remove(store, (hash.clone(), key))
        }
        Ok(())
    }

    fn get_all_values<K: PrimaryKey<'a> + Clone + KeyDeserialize, V: DeserializeOwned + Serialize>(
        &self,
        store: &dyn Storage,
        map: &Map<'a, K, V>,
    ) -> Result<Vec<V>, ContractError>
    where
        K::Output: 'static,
    {
        let values = map
            .range(store, None, None, Order::Ascending)
            .map(|r| r.map(|v| v.1))
            .collect::<Result<Vec<V>, StdError>>();
        values.map_err(ContractError::Std)
    }

    pub fn get_protocol_fee(&self, store: &dyn Storage) -> u128 {
        self.protocol_fee.load(store).unwrap_or(0)
    }
    pub fn store_protocol_fee(
        &self,
        store: &mut dyn Storage,
        fee: u128,
    ) -> Result<(), ContractError> {
        self.protocol_fee
            .save(store, &fee)
            .map_err(ContractError::Std)
    }

    pub fn store_protocol_fee_handler(
        &self,
        store: &mut dyn Storage,
        handler: String,
    ) -> Result<(), ContractError> {
        self.fee_handler
            .save(store, &handler)
            .map_err(ContractError::Std)
    }

    pub fn get_successful_response(&self, store: &dyn Storage, sn: u128) -> bool {
        self.successful_responses.load(store, sn).unwrap_or(false)
    }

    pub fn set_successful_response(
        &self,
        store: &mut dyn Storage,
        sn: u128,
    ) -> Result<(), ContractError> {
        self.successful_responses
            .save(store, sn, &true)
            .map_err(ContractError::Std)
    }
}
