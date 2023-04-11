use std::collections::HashMap;
use std::hash::Hash;

use crate::icon::icon::types::v1::MerkleNode;
use keccak_hash::keccak_256;

pub fn keccak256(input: &[u8]) -> [u8; 32] {
    let mut data = [0u8; 32];
    keccak_256(input, &mut data);
    data
}

pub fn calculate_root(leaf: [u8; 32], pathes: &[MerkleNode]) -> [u8; 32] {
    let mut temp = leaf.clone();

    for path in pathes {
        let mut out = [0u8; 32];
        let input = if path.dir == 0 {
            [path.value.clone(), temp.to_vec()].concat()
        } else {
            [temp.to_vec(), path.value.clone()].concat()
        };
        keccak_256(&input, &mut out);
        temp = out;
    }

    temp
}

pub fn to_lookup<T: Eq + PartialEq + Hash + Clone>(vec: &Vec<T>) -> HashMap<T, bool> {
    let mut hash_map: HashMap<T, bool> = HashMap::new();

    for val in vec {
        hash_map.insert(val.clone(), true);
    }
    return hash_map;
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
