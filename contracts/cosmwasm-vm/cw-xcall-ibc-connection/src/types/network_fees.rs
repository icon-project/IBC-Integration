use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct NetworkFees {
    pub send_packet_fee: u128,
    pub ack_fee: u128,
}
