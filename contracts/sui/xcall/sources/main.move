module xcall::main {

    // Part 1: Imports
    use sui::object::{Self, UID,ID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::linked_table::{Self, LinkedTable};
    use sui::types as sui_types;
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use xcall::types::{Self,NetworkAddress,RollbackData,network_address};
    use xcall::messages::{Self,XCallEnvelope,decode_envelope};
    use xcall::connection_out::{Self,init_register};
    use sui::bag::{Bag, Self};
    use sui::table::{Table,Self};
    use sui::package::{Self,Publisher};
  
    use sui::vec_map::{Self, VecMap};
    use sui::versioned::{Self, Versioned};


    const ENotOneTimeWitness: u64 = 0;
    const ENotAdmin: u64 = 1;
    const ENotUpgrade: u64 = 2;
    const EWrongVersion: u64 = 3;

    const NID: vector<u8> = b"nid";

    const CURRENT_VERSION: u64 = 1;

     struct IDCap has key,store {
        id:UID,
        xcall_id:ID,
    }
    struct PackageCap has store {
        package_id:String,
    }
     struct AdminCap has key {
        id: UID
    }


     struct Storage has key {
        id: UID,
        version:u64,
        admin:ID,
        requests:LinkedTable<u128, vector<u8>>,
        sequence_no:u128,
        protocol_fee:u128,
        protocol_fee_handler:address,
        connection_states:Bag,
        rollbacks:Table<u64,RollbackData>
    }
    
    fun init(ctx: &mut TxContext) {
        let admin = AdminCap {
            id: object::new(ctx),
        };
        let storage = Storage {
            id: object::new(ctx),
            version:CURRENT_VERSION,
            admin:object::id(&admin),
            requests:linked_table::new<u128, vector<u8>>(ctx),
            sequence_no:0,
            protocol_fee:0,
            protocol_fee_handler: tx_context::sender(ctx),
            connection_states:bag::new(ctx),
            rollbacks:table::new<u64,RollbackData>(ctx),
        };

        transfer::share_object(storage);
        transfer::transfer(admin, tx_context::sender(ctx));
    }

    public fun register_dapp<T: drop>(self:&Storage,
        witness: T,
        ctx: &mut TxContext
    ):IDCap {
        assert!(sui_types::is_one_time_witness(&witness), ENotOneTimeWitness);

       IDCap {
            id: object::new(ctx),
            xcall_id:object::id(self)

        }

       
    }

    public fun register_connection(self:&mut Storage,package_id:String){
        init_register(&mut self.connection_states,package_id);
    }

    fun send_call_inner(self:&mut Storage,from:types::NetworkAddress,to:types::NetworkAddress,envelope:XCallEnvelope){
        /*
         let caller = info.sender.clone();
        let config = self.get_config(deps.as_ref().storage)?;
        let nid = config.network_id;
        self.validate_payload(deps.as_ref(), &caller, &envelope)?;

        let sequence_no = self.get_next_sn(deps.storage)?;

        let from = NetworkAddress::new(&nid, caller.as_ref());
         if envelope.message.rollback().is_some() {
            let rollback_data = envelope.message.rollback().unwrap();
            let request = Rollback::new(
                caller.clone(),
                to.clone(),
                envelope.sources.clone(),
                rollback_data,
                false,
            );

            self.store_call_request(deps.storage, sequence_no, &request)?;
        }
        let call_request = CSMessageRequest::new(
            from,
            to.account(),
            sequence_no,
            envelope.message.msg_type().clone(),
            envelope.message.data(),
            envelope.destinations,
        );
        let need_response = call_request.need_response();

        let event = event_xcall_message_sent(caller.to_string(), to.to_string(), sequence_no);
        // if contract is in reply state
        if envelope.message.rollback().is_none()
            && self.is_reply(deps.as_ref(), to.nid(), &envelope.sources)
        {
            self.save_call_reply(deps.storage, &call_request)?;
            let res = self.send_call_response(event, sequence_no);
            return Ok(res);
        }

        let mut confirmed_sources = envelope.sources;
        if confirmed_sources.is_empty() {
            let default = self.get_default_connection(deps.as_ref().storage, to.nid())?;
            confirmed_sources = vec![default.to_string()]
        }
        let message: CSMessage = call_request.into();
        let sn: i64 = if need_response { sequence_no as i64 } else { 0 };
        let mut total_spent = 0_u128;

        let submessages = confirmed_sources
            .iter()
            .map(|r| {
                return self
                    .query_connection_fee(deps.as_ref(), to.nid(), need_response, r)
                    .and_then(|fee| {
                        let fund = if fee > 0 {
                            total_spent = total_spent.checked_add(fee).unwrap();
                            coins(fee, config.denom.clone())
                        } else {
                            vec![]
                        };
                        let address = deps.api.addr_validate(r)?;

                        self.call_connection_send_message(&address, fund, to.nid(), sn, &message)
                    });
            })
            .collect::<Result<Vec<SubMsg>, ContractError>>()?;

        let total_paid = self.get_total_paid(deps.as_ref(), &info.funds)?;
        let fee_handler = self.fee_handler().load(deps.storage)?;
        let protocol_fee = self.get_protocol_fee(deps.as_ref().storage);
        let total_fee_required = protocol_fee + total_spent;

        if total_paid < total_fee_required {
            return Err(ContractError::InsufficientFunds);
        }
        let remaining = total_paid - total_spent;

        println!("{LOG_PREFIX} Sent Bank Message");
        let mut res = self
            .send_call_response(event, sequence_no)
            .add_submessages(submessages);

        if remaining > 0 {
            let msg = BankMsg::Send {
                to_address: fee_handler,
                amount: coins(remaining, config.denom),
            };
            res = res.add_message(msg);
        }

        Ok(res)
        
        
        
        */
        let rollback=messages::rollback(&envelope);
        if(option::is_some(&rollback)){
            let rollback_data=option::extract<vector<u8>>(&mut rollback);
            let rollback_queue= types::create_rollback(from,to,messages::sources(&envelope),rollback_data,false);

            

        }


    }

    fun get_next_sequence(self:&mut Storage):u128 {
        let sn=self.sequence_no+1;
        self.sequence_no=sn;
        sn
    }


    entry fun set_protocol_fee(self:&mut Storage,admin:&AdminCap,fee:u128){
        self.protocol_fee=fee;
    }

    entry fun set_protocol_fee_handler(self:&mut Storage,admin:&AdminCap,fee_handler:address){
        self.protocol_fee_handler=fee_handler;
    }

    entry fun send_call(self:&mut Storage,idCap:&IDCap,to:String,envelope_bytes:vector<u8>){
        let envelope=decode_envelope(envelope_bytes);
        let to = types::network_address_from_string(to);
        let from= types::network_address(string::utf8(NID),string::utf8(object::id_to_bytes(&object::id(idCap))));
        send_call_inner(self,from,to,envelope)
    }


    entry fun migrate(self: &mut Storage, a: &AdminCap) {
        assert!(self.admin == object::id(a), ENotAdmin);
        assert!(self.version < CURRENT_VERSION, ENotUpgrade);
        self.version = CURRENT_VERSION;
       
    }

    

    


}