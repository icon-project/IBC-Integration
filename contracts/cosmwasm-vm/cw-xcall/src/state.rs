use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdResult;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

use crate::types::{
    address::Address,
    message::{CallRequest, CallServiceMessageRequest},
};
#[derive(Serialize, Deserialize)]
pub struct CallService<'T> {
    last_sequence_no: u128,
    last_request_id: u128,
    owner: Address,
    admin: Address,
    request: Map<'T, u128, CallRequest>,
    message_request: Map<'T, u128, CallServiceMessageRequest>,
}

impl CallService<'_> {
    pub fn new(sq_no: u128, req_id: u128, owner: Address, admin: Address) -> CallService<'static> {
        CallService {
            last_sequence_no: sq_no,
            last_request_id: req_id,
            owner,
            admin,
            request: Map::new("request"),
            message_request: Map::new("message_request"),
        }
    }
    pub fn last_sequence_no(&self) -> u128 {
        self.last_sequence_no
    }
    pub fn last_request_id(&self) -> u128 {
        self.last_request_id
    }

    pub fn owner(&self) -> Address {
        self.owner.clone()
    }
    pub fn admin(&self) -> Address {
        self.admin.clone()
    }

    pub fn request(&self) -> &Map<'_, u128, CallRequest> {
        &self.request
    }

    pub fn message_request(&self) -> &Map<'_, u128, CallServiceMessageRequest> {
        &self.message_request
    }
}

pub const CALLSERVICE: Item<CallService> = Item::new("call-service");

// pub const CALLSERVICE: Item<CallService> = Item::new("call-service");
// pub const CALLREQUEST: Map<'static, u128, CallRequest> = Map::new("callrequest");
//pub const CALLMESSAGE: Map<'static, u128, CallServiceMessageRequest> = Map::new("callmessage");
