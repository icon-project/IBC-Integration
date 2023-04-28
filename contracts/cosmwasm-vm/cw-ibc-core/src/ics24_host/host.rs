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
        self.ibc_store()
            .capabilities()
            .load(store, name)
            .map_err(|_| ContractError::IbcDecodeError {
                error: "CapabilityNotFound".into(),
            })
    }
    pub fn set_expected_time_per_block(
        &self,
        store: &mut dyn Storage,
        expected_time_per_block: u64,
    ) -> Result<(), ContractError> {
        self.ibc_store()
            .expected_time_per_block()
            .save(store, &expected_time_per_block)?;

        Ok(())
    }

    pub fn get_expected_time_per_block(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.ibc_store().expected_time_per_block().may_load(store)? {
            Some(time) => Ok(time),
            None => Err(ContractError::IbcDecodeError {
                error: "NotFound".to_string(),
            }),
        }
    }

    pub fn claim_capability(
        &self,
        store: &mut dyn Storage,
        name: Vec<u8>,
        address: String,
    ) -> Result<(), ContractError> {
        self.ibc_store().capabilities().update(
            store,
            name,
            |update| -> Result<_, ContractError> {
                match update {
                    Some(mut value) => {
                        if value.contains(&address) {
                            return Err(ContractError::IbcContextError {
                                error: "Capability already claimed".to_string(),
                            });
                        }
                        value.push(address);
                        Ok(value)
                    }
                    None => Err(ContractError::IbcDecodeError {
                        error: "KeyNotFound".into(),
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
        false
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

    pub fn calc_block_delay(&self, delay_period_time: &Duration) -> u64 {
        let max_expected_time_per_block = self.max_expected_time_per_block();

        if max_expected_time_per_block.is_zero() {
            return 0;
        }

        let delay = delay_period_time
            .as_secs()
            .checked_div(max_expected_time_per_block.as_secs())
            .unwrap();

        delay_period_time
            .checked_add(Duration::from_secs(delay))
            .unwrap()
            .as_secs()
    }
}
