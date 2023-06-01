use std::collections::HashMap;
use std::hash::Hash;

use crate::icon::icon::types::v1::MerkleNode;

pub fn keccak256(input: &[u8]) -> [u8; 32] {
    use sha3::{Digest, Keccak256};
    let mut hasher = Keccak256::new();
    hasher.update(input);
    let out: [u8; 32] = hasher.finalize().to_vec().try_into().unwrap();
    out
}

pub fn sha256(data: impl AsRef<[u8]>) -> Vec<u8> {
    use sha2::Digest;
    sha2::Sha256::digest(&data).to_vec()
}

pub fn calculate_root(leaf: [u8; 32], pathes: &[MerkleNode]) -> [u8; 32] {
    let mut temp = leaf;

    for path in pathes {
        // let mut out = [0u8; 32];
        let input = if path.dir == 0 {
            [path.value.clone(), temp.to_vec()].concat()
        } else {
            if path.value.is_empty() {
                continue;
            }
            [temp.to_vec(), path.value.clone()].concat()
        };
        let out = keccak256(&input);
        temp = out.try_into().unwrap();
    }

    temp
}

pub fn to_lookup<T: Eq + PartialEq + Hash + Clone>(vec: &Vec<T>) -> HashMap<T, bool> {
    let mut hash_map: HashMap<T, bool> = HashMap::new();

    for val in vec {
        hash_map.insert(val.clone(), true);
    }
    hash_map
}

// solidity bytes32 equivalent
pub fn bytes32(s: &[u8]) -> Option<[u8; 32]> {
    let s_len = s.len();

    if s_len > 32 {
        return None;
    }

    let mut result: [u8; 32] = Default::default();
    result[..s_len].clone_from_slice(s);
    Some(result)
}
