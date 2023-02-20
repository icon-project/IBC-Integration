use cw_xcall::types::address::Address;

pub fn alice() -> Address {
    Address::from_str("alice")
}

pub fn bob() -> Address {
    Address::from_str("bob")
}

pub fn admin_one() -> Address {
    Address::from_str("adminone")
}

pub fn admin_two() -> Address {
    Address::from_str("admintwo")
}
