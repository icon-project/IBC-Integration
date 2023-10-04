use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum StorageKey {
    Owner,
    Admin,
    IbcHost,
    XCallHost,
    NetworkFees,
    Config,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Config => "config",
            StorageKey::Admin => "admin",
            StorageKey::IbcHost => "ibc_host",
            StorageKey::XCallHost => "xcall_host",
            StorageKey::NetworkFees => "network_fees",
        }
    }
}
