use crate::types::{
    address::Address, request::CallServiceMessageRequest, stroage_keys::StorageKey,
};
use cw_storage_plus::{Item, Map};

pub struct CwCallservice<'a> {
    last_sequence_no: Item<'a, u128>,
    last_request_id: Item<'a, u128>,
    owner: Item<'a, Address>,
    admin: Item<'a, Address>,
    message_request: Map<'a, u128, CallServiceMessageRequest>,
}

impl<'a> Default for CwCallservice<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwCallservice<'a> {
    pub fn new() -> Self {
        Self {
            last_sequence_no: Item::new(StorageKey::SequenceNo.as_str()),
            last_request_id: Item::new(StorageKey::RequestNo.as_str()),
            owner: Item::new(StorageKey::Owner.as_str()),
            admin: Item::new(StorageKey::Admin.as_str()),
            message_request: Map::new(StorageKey::MessageRequest.as_str()),
        }
    }

    pub fn last_sequence_no(&self) -> &Item<'a, u128> {
        &self.last_sequence_no
    }

    pub fn last_request_id(&self) -> &Item<'a, u128> {
        &self.last_request_id
    }

    pub fn owner(&self) -> &Item<'a, Address> {
        &self.owner
    }

    pub fn admin(&self) -> &Item<'a, Address> {
        &self.admin
    }

    pub fn message_request(&self) -> &Map<'a, u128, CallServiceMessageRequest> {
        &self.message_request
    }
}
