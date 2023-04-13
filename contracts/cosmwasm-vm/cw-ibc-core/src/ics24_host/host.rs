use super::*;

impl<'a> CwIbcCoreContext<'a> {
    pub fn store_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: Vec<String>,
    ) -> Result<(), ContractError> {
        match self.ibc_store().capabilities().save(store, name, &address) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::IbcDecodeError {
                error: format!("FailedToStore {}", error),
            }),
        }
    }

    pub fn get_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        Ok(self
            .ibc_store()
            .capabilities()
            .load(store, name)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "CapabilityNotFound".into(),
            })?)
    }
    pub fn claim_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: String,
    ) -> Result<(), ContractError> {
        self.ibc_store().capabilities().update(
            store,
            name.clone(),
            |update| -> Result<_, ContractError> {
                match update {
                    Some(mut value) => {
                        value.push(address);
                        Ok(value)
                    }
                    None => Err(ContractError::IbcDecodeError {
                        error: "CapabilityNotFound".into(),
                    }),
                }
            },
        )?;

        Ok(())
    }

    pub fn authenticate_capability(
        &self,
        store: &mut dyn Storage,
        info: MessageInfo,
        name: Vec<u8>,
    ) -> bool {
        let caller = info.sender.to_string();
        let capability = self.get_capability(store, name).unwrap();
        if capability.contains(&caller) {
            return true;
        }
        return false;
    }

    pub fn lookup_modules(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
    ) -> Result<Vec<String>, ContractError> {
        let capabilities = self.get_capability(store, name)?;
        if capabilities.len() == 0 {
            return Err(ContractError::Unauthorized {});
        }
        Ok(capabilities)
    }
}
