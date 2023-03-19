use std::str::FromStr;

use cosmwasm_std::{StdError, Storage};
use cw_storage_plus::{Key, KeyDeserialize, PrimaryKey};

use super::*;
pub struct CwIbcRouter<'a>(Map<'a, ModuleId, Arc<dyn Module>>);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModuleId {
    key: String,
    value: IbcModuleId,
}

impl ModuleId {
    pub fn new(s: String) -> Self {
        let ibc_module_id = IbcModuleId::from_str(&s).unwrap();
        Self {
            key: ibc_module_id.to_string(),
            value: ibc_module_id,
        }
    }
    pub fn module_id(&self) -> IbcModuleId {
        self.value.clone()
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.key.as_bytes()
    }
}

impl<'a> PrimaryKey<'a> for ModuleId {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = ();
    type SuperSuffix = ();

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.as_bytes())]
    }
}

impl KeyDeserialize for ModuleId {
    type Output = ModuleId;
    fn from_vec(value: Vec<u8>) -> cosmwasm_std::StdResult<Self::Output> {
        let result = String::from_utf8(value)
            .map_err(StdError::invalid_utf8)
            .unwrap();
        let module_id = IbcModuleId::from_str(&result).unwrap();
        Ok(ModuleId {
            key: module_id.to_string(),
            value: module_id,
        })
    }
}

impl<'a> Default for CwIbcRouter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcRouter<'a> {
    pub fn new() -> Self {
        Self(Map::new(StorageKey::Router.as_str()))
    }
    pub fn router(&self) -> &Map<'a, ModuleId, Arc<dyn Module>> {
        &self.0
    }
}

impl<'a> CwIbcCoreContext<'a> {
    fn add_route(&self, store: &mut dyn Storage, module_id: ModuleId, module: impl Module) {
        todo!()
    }
    fn get_route(&self, module_id: ModuleId) -> Option<&dyn Module> {
        // self.ibc_router().router().load();
        todo!()
    }

    fn get_route_mut(&mut self, module_id: ModuleId) -> Option<&mut dyn Module> {
        todo!()
    }

    fn has_route(&self, module_id: ModuleId) -> bool {
        todo!()
    }
}
