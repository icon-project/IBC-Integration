use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    Addr, BlockInfo, ContractInfo, Empty, Env, MessageInfo, OwnedDeps, Timestamp, TransactionInfo,
};

use ibc::core::ics24_host::identifier::{ChannelId, ConnectionId, PortId};
use ibc_proto::ibc::core::channel::v1::Counterparty as RawCounterparty;
use ibc_proto::ibc::core::channel::v1::{
    MsgChannelOpenAck as RawMsgChannelOpenAck, MsgChannelOpenConfirm as RawMsgChannelOpenConfirm,
    MsgChannelOpenInit as RawMsgChannelOpenInit, MsgChannelOpenTry as RawMsgChannelOpenTry,
};
use ibc_proto::ibc::core::{channel::v1::Channel as RawChannel, client::v1::Height};

pub struct MockEnvBuilder {
    env: Env,
}

impl MockEnvBuilder {
    pub fn new() -> MockEnvBuilder {
        MockEnvBuilder {
            env: Env {
                block: BlockInfo {
                    height: 0,
                    time: Timestamp::from_nanos(0),
                    chain_id: "".to_string(),
                },
                transaction: None,
                contract: ContractInfo {
                    address: Addr::unchecked("input"),
                },
            },
        }
    }
    pub fn add_block(mut self, block: BlockInfo) -> MockEnvBuilder {
        self.env.block = block;
        self
    }

    pub fn add_txn_info(mut self, txn_info: Option<TransactionInfo>) -> MockEnvBuilder {
        self.env.transaction = txn_info;
        self
    }

    pub fn add_contract_info(mut self, contract_info: ContractInfo) -> MockEnvBuilder {
        self.env.contract = contract_info;
        self
    }

    pub fn build(self) -> Env {
        Env {
            block: self.env.block,
            transaction: self.env.transaction,
            contract: self.env.contract,
        }
    }
}

pub fn create_mock_info(creator: &str, denom: &str, amount: u128) -> MessageInfo {
    let funds = coins(amount, denom);
    mock_info(creator, &funds)
}

pub fn deps() -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    mock_dependencies()
}

#[test]
fn test() {
    let mock = mock_env();

    let block_info = BlockInfo {
        height: 12_345,
        time: Timestamp::from_nanos(1_571_797_419_879_305_533),
        chain_id: "cosmos-testnet-14002".to_string(),
    };

    let transaction = None;
    let contract = ContractInfo {
        address: Addr::unchecked(MOCK_CONTRACT_ADDR),
    };

    let mock_env_builder: Env = MockEnvBuilder::new()
        .add_block(block_info)
        .add_txn_info(transaction)
        .add_contract_info(contract)
        .build();

    assert_ne!(mock, mock_env_builder)
}

pub fn get_dummy_raw_counterparty(channel_id: String) -> RawCounterparty {
    RawCounterparty {
        port_id: PortId::default().to_string(),
        channel_id,
    }
}

// Returns a dummy `RawChannel`, for testing only!
pub fn get_dummy_raw_channel_end(channel_id: Option<u64>) -> RawChannel {
    let channel_id = match channel_id {
        Some(id) => ChannelId::new(id).to_string(),
        None => "".to_string(),
    };
    RawChannel {
        state: 1,
        ordering: 2,
        counterparty: Some(get_dummy_raw_counterparty(channel_id)),
        connection_hops: vec![ConnectionId::default().to_string()],
        version: "".to_string(), // The version is not validated.
    }
}

// Returns a dummy `RawMsgChannelOpenInit`, for testing only!
pub fn get_dummy_raw_msg_chan_open_init(
    counterparty_channel_id: Option<u64>,
) -> RawMsgChannelOpenInit {
    RawMsgChannelOpenInit {
        port_id: PortId::default().to_string(),
        channel: Some(get_dummy_raw_channel_end(counterparty_channel_id)),
        signer: get_dummy_bech32_account(),
    }
}
pub fn get_dummy_bech32_account() -> String {
    "cosmos1wxeyh7zgn4tctjzs0vtqpc6p5cxq5t2muzl7ng".to_string()
}

// Returns a dummy `RawMsgChannelOpenInit`, for testing only!
pub fn get_dummy_raw_msg_chan_open_ack(proof_height: u64) -> RawMsgChannelOpenAck {
    RawMsgChannelOpenAck {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        counterparty_channel_id: ChannelId::default().to_string(),
        counterparty_version: "".to_string(),
        proof_try: get_dummy_proof(),
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

pub fn get_dummy_proof() -> Vec<u8> {
    "Y29uc2Vuc3VzU3RhdGUvaWJjb25lY2xpZW50LzIy"
        .as_bytes()
        .to_vec()
}

// Returns a dummy `RawMsgChannelOpenConfirm`, for testing only!
pub fn get_dummy_raw_msg_chan_open_confirm(proof_height: u64) -> RawMsgChannelOpenConfirm {
    RawMsgChannelOpenConfirm {
        port_id: PortId::default().to_string(),
        channel_id: ChannelId::default().to_string(),
        proof_ack: get_dummy_proof(),
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}

// Returns a dummy `RawMsgChannelOpenTry`, for testing only!
pub fn get_dummy_raw_msg_chan_open_try(proof_height: u64) -> RawMsgChannelOpenTry {
    #[allow(deprecated)]
    RawMsgChannelOpenTry {
        port_id: PortId::default().to_string(),
        previous_channel_id: ChannelId::default().to_string(),
        channel: Some(get_dummy_raw_channel_end(Some(0))),
        counterparty_version: "".to_string(),
        proof_init: get_dummy_proof(),
        proof_height: Some(Height {
            revision_number: 0,
            revision_height: proof_height,
        }),
        signer: get_dummy_bech32_account(),
    }
}
