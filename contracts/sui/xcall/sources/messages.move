module xcall::messages {
use std::string::{Self, String};
use std::vector;
use std::option::{Self, Option,some,none};
     const CS_REQUEST:u8 = 1;

    const CS_RESULT:u8 = 2;

    const CALL_MESSAGE_TYPE:u8 = 0;
    const CALL_MESSAGE_ROLLBACK_TYPE:u8 = 1;
    const PERSISTENT_MESSAGE_TYPE:u8 = 2;
    
    struct XCallEnvelope has drop{
        message_type:u8,
        message:vector<u8>,
        sources:vector<String>,
        destinations:vector<String>,
    }

    struct CallMessage has drop{
         data:vector<u8>
    }

    struct CallMessageWithRollback has drop{
       data:vector<u8>,
       rollback:vector<u8>,
    }

    public fun encode_call_message_rollback(msg:CallMessageWithRollback):vector<u8>{
         vector::empty<u8>()
    }

    public fun encode_envelope(msg:XCallEnvelope):vector<u8>{
         vector::empty<u8>()
    }

     public fun decode_envelope(bytes:vector<u8>):XCallEnvelope{
          XCallEnvelope {
            message_type:1,
            message:vector::empty<u8>(),
            sources:vector::empty<String>(),
            destinations:vector::empty<String>(),

         }
    }

     public fun decode_call_message_rollback(bytes:vector<u8>):CallMessageWithRollback{
          CallMessageWithRollback {
           data:vector::empty<u8>(),
           rollback:vector::empty<u8>(),

         }
    }


    public fun call_message(data:vector<u8>,sources:vector<String>,destinations:vector<String>): XCallEnvelope {
        let envelope= XCallEnvelope {
            message_type:CALL_MESSAGE_TYPE,
            message:data,
            sources:sources,
            destinations:destinations,

        };
        envelope

    }

     public fun call_message_rollback(data:vector<u8>,rollback:vector<u8>,sources:vector<String>,destinations:vector<String>): XCallEnvelope {
        let message= CallMessageWithRollback {
            data:data,
            rollback:rollback,
        };
        let envelope= XCallEnvelope {
            message_type:CALL_MESSAGE_ROLLBACK_TYPE,
            message:encode_call_message_rollback(message),
            sources:sources,
            destinations:destinations,

        };
        envelope

    }

    public fun rollback(self:&XCallEnvelope):Option<vector<u8>>{
        if (self.message_type==CALL_MESSAGE_ROLLBACK_TYPE) {
            let msg= decode_call_message_rollback(self.message);
             some(msg.rollback)

        }else {
         none()
        }
             
    }

    public fun sources(self:&XCallEnvelope):vector<String>{
        self.sources
    }

    

    

   
}