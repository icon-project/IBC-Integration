module xcall::connection_out {
use std::string::{Self, String};
use sui::bag::{Bag, Self};
use xcall::centralized_connection::{Self};

const EConnectionNotFound:u64=0;

const ConnCentralized:vector<u8> =b"centralized";


    public fun init_register(states:&mut Bag,package_id:String){
       
        if (package_id==string::utf8(ConnCentralized)){
              let state= centralized_connection::connect();
              bag::add(states, package_id, state);

        }else{
           abort EConnectionNotFound
        }
       
        
    }

    
}