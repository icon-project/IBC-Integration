#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum CrossContractMessage {
    XCallMessage { data: Vec<u8> },
}
