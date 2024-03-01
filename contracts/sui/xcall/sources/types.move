module xcall::types {
    use std::string::{Self, String};
    use sui::object::{Self, ID, UID};

   
   
    struct NetworkAddress has drop,store{
        net_id:String,
        addr:String,
    }

    struct CSMessageRequest has store{
    from:NetworkAddress,
    to: NetworkAddress,
    sn:u128,
    message_type:u8,
    data:vector<u8>,
    protocols:vector<String>,
   }

   struct RollbackData has store,drop{
        from:NetworkAddress,
        to:NetworkAddress,
        sources:vector<String>,
        rollback:vector<u8>,
        enabled:bool, 
    }

    struct CSMessage has store{
        msg_type:u8,
        payload:vector<u8>,
    }

    struct CSMessageResponse has store{
        sn:u128,
        code:u8,
    }


    public fun network_address(nid:String,addr:String):NetworkAddress {
        return NetworkAddress{
            net_id:nid,
            addr:addr,
        }
    }

    public fun network_address_from_string(net_addr:String):NetworkAddress {
        return NetworkAddress {
            net_id:string::utf8(b"nid"),
            addr:string::utf8(b"addr"),
        }
    }

    public fun create_rollback(from:NetworkAddress,to:NetworkAddress,sources:vector<String>,rollback:vector<u8>,enabled:bool):RollbackData{
        RollbackData {
            from:from,
            to:to,
            sources:sources,
            rollback:rollback,
            enabled:enabled
        }

        
    }

    

}