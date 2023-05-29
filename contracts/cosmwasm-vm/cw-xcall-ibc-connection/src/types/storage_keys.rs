use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum StorageKey {
    Owner,
    Admin,
    IbcConfig,
    IbcHost,
    TimeoutHeight,
    XCallHost,
    FeeHandler,
    Balance,
    Fee,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Admin => "admin",
            StorageKey::IbcConfig => "ibcconfig",
            StorageKey::IbcHost => "ibc_host",
            StorageKey::TimeoutHeight => "timeout_height",
            StorageKey::XCallHost => "xcall_host",
            StorageKey::FeeHandler => "feehandler",
            StorageKey::Balance => "balance",
            StorageKey::Fee => "fee",
        }
    }
}
