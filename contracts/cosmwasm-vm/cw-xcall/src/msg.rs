use super::*;

#[cw_serde]
pub struct InstantiateMsg {
    pub timeout_height: u64,
    pub ibc_host: Addr,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(String)]
    GetAdmin {},
    #[returns(u128)]
    GetProtocolFee {},
    #[returns(String)]
    GetProtocolFeeHandler {},
    #[returns(u64)]
    GetTimeoutHeight {},
}
