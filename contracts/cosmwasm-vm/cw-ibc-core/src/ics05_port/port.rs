use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn lookup_module_by_port(
        &self,
        store: &mut dyn Storage,
        port_id: PortId,
    ) -> Result<ModuleId, ContractError> {
        match self
            .ibc_store()
            .port_to_moulde()
            .may_load(store, port_id.clone())
        {
            Ok(result) => match result {
                Some(port_id) => Ok(port_id),
                None => Err(ContractError::IbcPortError {
                    error: PortError::UnknownPort {
                        port_id: port_id.port_id().clone(),
                    },
                }),
            },
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
