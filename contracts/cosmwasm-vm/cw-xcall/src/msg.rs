use super::*;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Address)]
    GetAdmin {},
    #[returns(u128)]
    GetProtocolFee {},
    #[returns(Address)]
    GetProtocolFeeHandler {},
}
