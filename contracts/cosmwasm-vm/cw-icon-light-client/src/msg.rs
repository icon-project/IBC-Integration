use cosmwasm_schema::{cw_serde, QueryResponses};
use ibc_proto::google::protobuf::Any;

use crate::traits::{Config, ConsensusStateUpdate};

#[cw_serde]
pub struct InstantiateMsg {
    pub src_network_id: String,
    pub network_id: u64,
    pub network_type_id: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateClient {
        client_id: String,
        client_state_bytes: Vec<u8>,
        consensus_state_bytes: Vec<u8>,
    },
    UpdateClient {
        client_id: String,
        signed_header: Vec<u8>,
    },
    VerifyMembership {
        message_bytes: Vec<u8>,
        proofs: Vec<u8>,
        height: u64,
    },
}




#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
   
    #[returns(u64)]
    GetLatestHeight{
        client_id:String
    },
    #[returns(Vec<u8>)]
    GetConsensusState{
        client_id:String,
        height:u64,
    },
    #[returns(Vec<u8>)]
    GetClientState{
        client_id:String
    }
}
