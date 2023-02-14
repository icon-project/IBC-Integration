use crate::{error::CallServiceError, state::CallService, types::address::Address};

impl CallService {
    pub fn add_admin(&mut self, address: Address) -> Result<(), CallServiceError> {
        match self.admins.contains(&address) {
            true => {
                return Err(CallServiceError::AdminAlreadyExist);
            }
            false => {
                self.admins.add(address);
                Ok(())
            }
        }
    }

    pub fn remove_admin(&mut self, address: Address) -> Result<(), CallServiceError> {
        match self.admins.contains(&address) {
            true => {
                self.admins.remove(&address);
                Ok(())
            }
            false => Err(CallServiceError::AdminNotExist),
        }
    }

    pub fn is_admin(&self, address: Address) -> bool {
        self.admins.contains(&address)
    }
}
