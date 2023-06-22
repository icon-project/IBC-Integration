use std::str::FromStr;

use cosmwasm_std::StdError;

pub struct NetworkAddress {
    nid: String,
    account: String,
}

impl NetworkAddress {
    pub fn new(nid:&str,address:&str)->Self{
        Self{
            nid:nid.to_owned(),
            account:address.to_owned()
        }
    }
    pub fn get_nid(&self) -> &str {
        return &self.nid;
    }

    pub fn get_account(&self) -> &str {
        return &self.account;
    }
}

impl ToString for NetworkAddress {
    fn to_string(&self) -> String {
        format!("{}/{}",self.nid,self.account)
    }
}

impl FromStr for NetworkAddress {
    type Err = StdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("/").collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(StdError::GenericErr {
                msg: "Invalid Input".to_owned(),
            });
        }
        let na = NetworkAddress {
            nid: parts[0].to_string(),
            account: parts[1].to_string(),
        };
        Ok(na)
    }
}
