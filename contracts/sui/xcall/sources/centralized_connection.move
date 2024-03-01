module xcall::centralized_connection {
  

    struct State has store {
        fee:u64,
       
    }

    public fun connect():State{

        State {
            fee:0,
            
        }
    }

}