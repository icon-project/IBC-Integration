use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub xcall_address: Addr,
    pub owner: Addr,
    pub sequence: u64,
}

pub const STATE: Item<State> = Item::new("CORE_STATE");
