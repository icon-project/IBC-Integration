use super::*;

/// Storage for modules based on the module id
pub struct CwIbcRouter<'a>(Map<'a, ModuleId, Addr>);

impl<'a> Default for CwIbcRouter<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CwIbcRouter<'a> {
    pub fn new() -> Self {
        Self(Map::new(StorageKey::Router.as_str()))
    }
}

impl<'a> CwIbcCoreContext<'a> {
    pub fn add_route(
        &self,
        store: &mut dyn Storage,
        module_id: ModuleId,
        module: &Addr,
    ) -> Result<(), ContractError> {
        match self.ibc_router().0.save(store, module_id, module) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
    pub fn get_route(
        &self,
        store: &dyn Storage,
        module_id: ModuleId,
    ) -> Result<Addr, ContractError> {
        match self.ibc_router().0.may_load(store, module_id) {
            Ok(result) => match result {
                Some(address) => Ok(address),
                None => Err(ContractError::IbcDecodeError {
                    error: "Module Id Not Found".to_string(),
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn has_route(&self, store: &dyn Storage, module_id: ModuleId) -> bool {
        self.ibc_router()
            .0
            .may_load(store, module_id)
            .unwrap()
            .is_some()
    }
}
