use std::{borrow::Borrow, task::Context};

use crate::{state::CwCallservice, types::address::Address};


impl<'a> CwCallservice<'a>{
    pub fn execute_call( &self,  request_id: i128) {
        let mut proxyReqs = Self::last_request_id;

        assert!(proxyReqs.has() > 0, "InvalidRequestId");
        proxyReqs.remove(request_id);

        let network_address = Address::from_str(proxyReqs);


    }
    }

