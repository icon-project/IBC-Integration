use super::*;

impl<'a> CwMockService<'a> {
    pub fn init_sequence(
        &self,
        store: &mut dyn Storage,
        sequence_no: u64,
    ) -> Result<(), ContractError> {
        match self.sequence().save(store, &sequence_no) {
            Ok(_) => Ok(()),
            Err(error) => Err(ContractError::Std(error)),
        }
    }

    pub fn increment_sequence(&self, store: &mut dyn Storage) -> Result<u64, ContractError> {
        self.sequence().update(store, |seq| Ok(seq + 1))
    }

    pub fn get_sequence(&self, store: &dyn Storage) -> Result<u64, ContractError> {
        match self.sequence().load(store) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(ContractError::Std(error)),
        }
    }
}
