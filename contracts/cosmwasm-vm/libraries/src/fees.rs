use std::{collections::btree_map::Values, error::Error};


use cosmwasm_std::{Deps, DepsMut, StdResult};
use cw_storage_plus::{Key, Map};

type Account = String;

#[derive(Debug)]
pub struct Fees<'a>(Map<'a, Account, u128>);

impl Fees<'_> {
    pub fn new() -> Self {
        Fees(Map::new("Fees"))
    }

    pub fn add(&self, deps: DepsMut, address: Account, value: u128) {
     match !self.0.has(deps.storage, address.clone() ) {
        true => {
            self.0.save(deps.storage,address,&value);
            }
        false => (),
    }
        }
        
            
    pub fn remove(&self, deps: DepsMut, address: Account, value: u128)  {
        if self.0.has(deps.storage, address.clone())  {
            self.0.remove(deps.storage, address);
        }
    }

    pub fn get(&self, deps: DepsMut, address: Account, value: u128) -> StdResult<u128> {
        self.0.load(deps.storage, address)
    }

    pub fn contains(&self, deps: DepsMut, address: Account, value: u128) -> bool {
        self.0.has(deps.storage, address)
    }
}

mod tests {
    // use crate::tests;

    use crate::add;

    use super::{Fees, Account};
    use cosmwasm::mock::{self, MockApi, MockStorage};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    fn storage(){
        
    }

    #[test]
    fn contains_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage = mock::MockStorage::new();
         let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
         let value = 3402823669209384634;

        let result = fees.contains(deps.as_mut(), address, value);
        return 
    }

    #[test]
    fn removing_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage=mock::MockStorage::new();
        let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
        let value = 3402823669209384634;

        fees.contains(deps.as_mut(), address.clone(), value);
        let result = fees.remove(deps.as_mut(), address.clone(), value);
    }
     
    #[test]
    fn adding_existing_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage=mock::MockStorage::new();
        let address = " ".to_string();
        let value = 0;

        fees.contains(deps.as_mut(), address.clone(), value);
        let result = fees.add(deps.as_mut(), address.clone(), value);

    }

    #[test]
    fn removing_non_existing_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage = mock::MockStorage::new();
        let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
        let value = 3402823669209384634;

        fees.contains(deps.as_mut(), address, value);
        // match fees.get(deps.as_mut(), address, value) {
        //     true => {
        //         let result = fees.remove(deps.as_mut(), address, value);
        //         }
        //     false => return,
        // }

    }

    #[test]
    fn get_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage = mock::MockStorage::new();
        let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
        let value = 3402823669209384634;

        fees.get(deps.as_mut(), address, value);
    }

    #[test]
    fn setting_account(){
        let mut deps = mock_dependencies();
        let mut fees = Fees::new();
        let mock_storage = mock::MockStorage::new();
        let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
        let value = 3402823669209384634;

        fees.add(deps.as_mut(), address, value);
    }
   
    #[test]
    fn getting_non_existing_account(){
        // let mut deps = mock_dependencies();
        // let mut fees = Fees::new();
        // let mock_storage = mock::MockStorage::new();
        // let address = "88bd05442686be0a5df7da33b6f1089ebfea3769b19dbb2477fe0cd6e0f126e4".to_string();
        // let value = 3402823669209384634;


    }

   



    

}
