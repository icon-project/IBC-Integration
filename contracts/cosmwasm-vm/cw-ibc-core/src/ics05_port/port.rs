use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn lookup_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
    ) -> Result<ModuleId, ContractError> {
        match self
            .ibc_store()
            .port_to_module()
            .may_load(store, port_id.clone())
        {
            Ok(result) => match result {
                Some(port_id) => Ok(port_id),
                None => Err(ContractError::IbcPortError {
                    error: PortError::UnknownPort {
                        port_id: port_id.ibc_port_id().clone(),
                    },
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn store_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
        module_id: ModuleId,
    ) -> Result<(), ContractError> {
        Ok(self
            .ibc_store()
            .port_to_module()
            .save(store, port_id, &module_id)?)
    }
}
