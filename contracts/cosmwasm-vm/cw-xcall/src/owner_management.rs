use crate::{error::CallServiceError, state::CallService, types::address::Address};

impl CallService {
    pub fn add_owner(&mut self, address: Address) -> Result<(), CallServiceError> {
        match self.owners.contains(&address) {
            true => {
                return Err(CallServiceError::AdminAlreadyExist);
            }
            false => {
                self.owners.add(address);
                Ok(())
            }
        }
    }

    pub fn remove_owner(&mut self, address: Address) -> Result<(), CallServiceError> {
        match self.owners.contains(&address) {
            true => {
                self.owners.remove(&address);
                Ok(())
            }
            false => Err(CallServiceError::AdminNotExist),
        }
    }

    pub fn is_owner(&self, address: Address) -> bool {
        self.owners.contains(&address)
    }
}
