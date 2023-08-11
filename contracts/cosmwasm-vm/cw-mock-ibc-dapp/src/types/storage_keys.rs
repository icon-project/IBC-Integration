use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum StorageKey {
    Owner,
    Admin,
    IbcConfig,
    IbcHost,
    Config,
    ReceivedPackets,
}

impl StorageKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageKey::Owner => "owner",
            StorageKey::Config => "config",
            StorageKey::Admin => "admin",
            StorageKey::IbcConfig => "ibcconfig",
            StorageKey::IbcHost => "ibc_host",
            StorageKey::ReceivedPackets => "received_packets",
        }
    }
}
