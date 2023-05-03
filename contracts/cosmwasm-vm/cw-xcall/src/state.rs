use super::*;

pub const MAX_DATA_SIZE: u64 = 2048;
pub const MAX_ROLLBACK_SIZE: u64 = 1024;
pub const EXECUTE_CALL_ID: u64 = 0;
pub const EXECUTE_ROLLBACK_ID: u64 = 1;
pub const ACK_FAILURE_ID: u64 = 3;
pub const SEND_CALL_MESSAGE_REPLY_ID: u64 = 2;

#[cw_serde]
pub struct IbcConfig {
    sequence: u128,
    src: IbcEndpoint,
    dst: IbcEndpoint,
}

impl IbcConfig {
    pub fn new(src: IbcEndpoint, dst: IbcEndpoint) -> Self {
        Self {
            src,
            dst,
            sequence: u128::default(),
        }
    }

    pub fn src_endpoint(&self) -> &IbcEndpoint {
        &self.src
    }

    pub fn dst_endpoint(&self) -> &IbcEndpoint {
        &self.dst
    }

    pub fn sequence(&self) -> u128 {
        self.sequence
    }

    pub fn next_sequence(&self) -> Option<u128> {
        self.sequence.checked_add(1)
    }
}

pub struct CwCallService<'a> {
    last_sequence_no: Item<'a, u128>,
    last_request_id: Item<'a, u128>,
    owner: Item<'a, String>,
    admin: Item<'a, String>,
    message_request: Map<'a, u128, CallServiceMessageRequest>,
    requests: Map<'a, u128, CallRequest>,
    ibc_config: Item<'a, IbcConfig>,
    fee_handler: Item<'a, String>,
    fee: Item<'a, u128>,
    ibc_host: Item<'a, Addr>,
    timeout_height: Item<'a, u64>,
}

impl<'a> Default for CwCallService<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwCallService<'a> {
    pub fn new() -> Self {
        Self {
            last_sequence_no: Item::new(StorageKey::SequenceNo.as_str()),
            last_request_id: Item::new(StorageKey::RequestNo.as_str()),
            owner: Item::new(StorageKey::Owner.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            message_request: Map::new(StorageKey::MessageRequest.as_str()),
            requests: Map::new(StorageKey::Requests.as_str()),
            ibc_config: Item::new(StorageKey::IbcConfig.as_str()),
            fee_handler: Item::new(StorageKey::FeeHandler.as_str()),
            fee: Item::new(StorageKey::Fee.as_str()),
            ibc_host: Item::new(StorageKey::IbcHost.as_str()),
            timeout_height: Item::new(StorageKey::TimeoutHeight.as_str()),
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

    pub fn message_request(&self) -> &Map<'a, u128, CallServiceMessageRequest> {
        &self.message_request
    }

    pub fn call_requests(&self) -> &Map<'a, u128, CallRequest> {
        &self.requests
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
    pub fn get_host(&self, store: &dyn Storage) -> Result<Addr, ContractError> {
        self.ibc_host.load(store).map_err(ContractError::Std)
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
