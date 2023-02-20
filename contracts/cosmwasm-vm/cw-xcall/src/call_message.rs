use std::{env, vec};

use cosmwasm_std::Env;
use cosmwasm_std::{IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock};

use crate::state::CwCallservice;

impl<'a> CwCallservice<'a> {
    pub fn send_call_message(&self, env: Env) {
        // let packet = self.send_Packet(env, src, dst, sequence, time_out_height, data);
    }

    fn send_Packet(
        &self,
        env: Env,
        src: IbcEndpoint,
        dst: IbcEndpoint,
        sequence: u64,
        time_out_height: u64,
        data: &[u8],
    ) -> IbcPacket {
        let timeout_block = IbcTimeoutBlock {
            revision: 0,
            height: time_out_height,
        };
        let timeout = IbcTimeout::with_both(timeout_block, env.block.time.plus_seconds(300));
        IbcPacket::new(data, src, dst, sequence, timeout)
    }
}
