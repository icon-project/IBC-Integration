use std::{
    collections::HashMap,
    env,
    ffi::OsString,
    fs::{self, read_dir, File},
    io::{self, ErrorKind, Read},
    path::PathBuf,
};

use serde::Deserialize;

use common::icon::icon::types::v1::BtpHeader;
use common::icon::icon::types::v1::MerkleNode;
use common::icon::icon::types::v1::SignedHeader;
use cosmwasm_std::Attribute;

pub mod constants;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestHeader {
    pub main_height: u64,
    pub round: u32,
    pub next_proof_context_hash: String,
    pub network_section_to_root: Vec<TestMerkleNode>,
    pub network_id: u64,
    pub update_number: u64,
    pub prev_network_section_hash: String,
    pub message_count: u64,
    pub message_root: String,
    pub next_validators: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct TestHeaderData {
    pub signed_header: TestSignedHeader,
    pub encoded_protobuf: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestSignedHeader {
    #[serde(rename(deserialize = "BTPHeader"))]
    pub btp_header: TestHeader,
    pub signature: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TestMerkleNode {
    pub dir: i32,
    pub value: String,
}

impl TryFrom<TestMerkleNode> for MerkleNode {
    type Error = hex::FromHexError;

    fn try_from(value: TestMerkleNode) -> Result<Self, Self::Error> {
        let node = MerkleNode {
            dir: value.dir,
            value: hex::decode(value.value.replace("0x", "")).unwrap(),
        };
        Ok(node)
    }
}

impl TryFrom<TestHeader> for BtpHeader {
    type Error = hex::FromHexError;

    fn try_from(value: TestHeader) -> Result<Self, Self::Error> {
        let btp_header = BtpHeader {
            main_height: value.main_height,
            message_count: value.message_count,
            message_root: hex::decode(value.message_root.replace("0x", ""))?,
            network_id: value.network_id,
            network_section_to_root: value
                .network_section_to_root
                .into_iter()
                .map(|tn| {
                    let node: MerkleNode = tn.try_into().unwrap();
                    node
                })
                .collect(),
            next_proof_context_hash: hex::decode(value.next_proof_context_hash.replace("0x", ""))?,
            next_validators: value
                .next_validators
                .into_iter()
                .map(|v| hex::decode(v.replace("0x", "")).unwrap())
                .collect(),
            prev_network_section_hash: hex::decode(
                value.prev_network_section_hash.replace("0x", ""),
            )?,
            round: value.round,
            update_number: value.update_number,
        };
        Ok(btp_header)
    }
}

impl TryFrom<TestSignedHeader> for SignedHeader {
    type Error = hex::FromHexError;

    fn try_from(value: TestSignedHeader) -> Result<Self, Self::Error> {
        let btp_header: BtpHeader = value.btp_header.try_into()?;
        let signatures = value
            .signature
            .iter()
            .map(|s| hex::decode(s.replace("0x", "")).unwrap())
            .collect();
        return Ok(SignedHeader {
            header: Some(btp_header),
            signatures,
        });
    }
}

pub fn load_test_headers() -> Vec<TestHeaderData> {
    let mut root = get_project_root().unwrap();
    root.push("test_data/test_headers.json");
    let mut file = File::open(root).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let data: Vec<TestHeaderData> =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    data
}

pub fn get_test_headers() -> Vec<BtpHeader> {
    return load_test_headers()
        .into_iter()
        .map(|th| {
            let btp: BtpHeader = th.signed_header.btp_header.try_into().unwrap();
            btp
        })
        .collect::<Vec<BtpHeader>>();
}

pub fn get_test_signed_headers() -> Vec<SignedHeader> {
    return load_test_headers()
        .into_iter()
        .map(|th| {
            let btp: SignedHeader = th.signed_header.try_into().unwrap();
            btp
        })
        .collect::<Vec<SignedHeader>>();
}

pub fn get_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            return Ok(PathBuf::from(p));
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}

pub fn to_attribute_map(attrs: &Vec<Attribute>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for attr in attrs {
        map.insert(attr.key.clone(), attr.value.clone());
    }
    return map;
}
