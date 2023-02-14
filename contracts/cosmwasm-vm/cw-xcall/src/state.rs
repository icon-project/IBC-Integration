use cosmwasm_schema::cw_serde;

use cw_storage_plus::Item;

use crate::types::{address::Address, admins::Admins, owners::Owners, request::CSMessageRequests};
#[cw_serde]
pub struct CallService {
    pub last_sequence_no: u128,
    pub last_request_id: u128,
    pub owners: Owners,
    pub admins: Admins,
    pub message_request: CSMessageRequests,
}

impl CallService {
    pub fn new(sq_no: u128, req_id: u128, owner: Address, admin: Address) -> CallService {
        let mut owners = Owners::default();
        owners.add(owner);

        let mut admins = Admins::default();
        admins.add(admin);

        CallService {
            last_sequence_no: sq_no,
            last_request_id: req_id,
            owners,
            admins,
            message_request: CSMessageRequests::default(),
            // message_request: HashMap::new(),
        }
    }
}

pub const CALLSERVICE: Item<CallService> = Item::new("call-service");
