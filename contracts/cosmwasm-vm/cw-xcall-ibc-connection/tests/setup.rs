use cw_xcall_lib::network_address::NetId;
use std::str::FromStr;

use cosmwasm_std::{
    coins,
    testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
        MOCK_CONTRACT_ADDR,
    },
    to_json_binary as to_binary, Addr, BlockInfo, ContractInfo, DepsMut, Empty, Env, Event,
    IbcChannel, IbcEndpoint, IbcPacket, IbcTimeout, IbcTimeoutBlock, MessageInfo, OwnedDeps,
    Storage, SubMsgResponse, Timestamp, TransactionInfo,
};

use cw_xcall_ibc_connection::{
    state::{CwIbcConnection, IbcConfig},
    types::{
        channel_config::ChannelConfig, config::Config, connection_config::ConnectionConfig,
        network_fees::NetworkFees,
    },
};

pub struct MockEnvBuilder {
    env: Env,
}

impl Default for MockEnvBuilder {
    fn default() -> Self {
        Self::new()
    }
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

pub fn get_dummy_sub_msg_res() -> SubMsgResponse {
    SubMsgResponse {
        events: vec![Event::new("empty")],
        data: Some(to_binary("data").unwrap()),
    }
}

pub fn get_dummy_endpoints() -> (IbcEndpoint, IbcEndpoint) {
    let endpoint = IbcEndpoint {
        port_id: "port_src".to_owned(),
        channel_id: "channel_src".to_owned(),
    };

    let counterparty_endpoint = IbcEndpoint {
        port_id: "port_dst".to_owned(),
        channel_id: "channel_dst".to_owned(),
    };

    (endpoint, counterparty_endpoint)
}

pub fn get_dummy_channel() -> IbcChannel {
    let (endpoint, counterparty_endpoint) = get_dummy_endpoints();

    IbcChannel::new(
        endpoint,
        counterparty_endpoint,
        cosmwasm_std::IbcOrder::Unordered,
        "ics20-1".to_owned(),
        "connection_0".to_owned(),
    )
}

pub fn get_dummy_channel_config() -> ChannelConfig {
    ChannelConfig {
        client_id: "default_0".to_owned(),
        timeout_height: 100,
        counterparty_nid: NetId::from_str("nid").unwrap(),
    }
}

pub fn get_dummy_connection_config() -> ConnectionConfig {
    ConnectionConfig {
        client_id: "default_0".to_owned(),
        timeout_height: 100,
    }
}

pub fn get_dummy_ibc_config() -> IbcConfig {
    let (src, dst) = get_dummy_endpoints();
    IbcConfig::new(src, dst)
}

pub fn get_dummy_network_fees() -> NetworkFees {
    NetworkFees {
        ack_fee: 0,
        send_packet_fee: 0,
    }
}

pub fn get_dummy_ibc_packet() -> IbcPacket {
    let (src, dst) = get_dummy_endpoints();
    let timeout = IbcTimeout::with_block(IbcTimeoutBlock {
        revision: 0,
        height: 0,
    });
    IbcPacket::new(to_binary(&vec![0]).unwrap(), src, dst, 0, timeout)
}

pub struct TestContext {
    pub network_id: NetId,
    pub client_id: String,
    pub connection_id: String,
    pub info: MessageInfo,
    pub channel: IbcChannel,
    pub env: Env,
    pub packet: Option<IbcPacket>,
}

impl TestContext {
    pub fn default() -> Self {
        Self {
            network_id: NetId::from_str("nid").unwrap(),
            client_id: "default_0".to_owned(),
            connection_id: "connection_0".to_owned(),
            info: mock_info("owner", &[]),
            channel: get_dummy_channel(),
            env: mock_env(),
            packet: None,
        }
    }

    pub fn for_packet_ack(packet: &IbcPacket) -> Self {
        let mut ctx = TestContext::default();
        ctx.packet = Some(packet.clone());

        ctx
    }

    pub fn for_packet_timeout(packet: &IbcPacket) -> Self {
        let mut ctx = TestContext::default();
        ctx.packet = Some(packet.clone());

        ctx
    }

    pub fn init_context(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        self.add_owner(storage, contract);
        self.update_admin(storage, contract);
        self.store_ibc_host(storage, contract);
        self.store_xcall_host(storage, contract);
        self.store_config(storage, contract);
        self.store_connection_config(storage, contract);
        self.store_channel_config(storage, contract);
        self.store_network_fees(storage, contract);
        self.store_ibc_config(storage, contract);
    }

    pub fn init_channel_open(&self, deps: DepsMut, contract: &CwIbcConnection) {
        self.init_context(deps.storage, contract);

        self.store_counterparty_nid(deps.storage, contract);
        self.configure_connection(deps, contract);
    }

    pub fn init_channel_connect(&self, deps: DepsMut, contract: &CwIbcConnection) {
        self.init_context(deps.storage, contract);

        self.store_counterparty_nid(deps.storage, contract);
        self.configure_connection(deps, contract);
    }

    pub fn add_owner(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        let _res = contract
            .add_owner(storage, self.info.sender.clone())
            .unwrap();
    }

    pub fn update_admin(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        let _res = contract
            .update_admin(storage, self.info.sender.clone())
            .unwrap();
    }

    pub fn store_ibc_host(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .set_ibc_host(storage, Addr::unchecked("ibc_host"))
            .unwrap()
    }

    pub fn store_xcall_host(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .set_xcall_host(storage, Addr::unchecked("xcall_host"))
            .unwrap()
    }

    pub fn store_config(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        let config = Config {
            port_id: "port_src".to_owned(),
            denom: "arch".to_owned(),
        };
        contract.store_config(storage, &config).unwrap()
    }

    pub fn store_channel_config(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .store_channel_config(storage, "channel_src", &get_dummy_channel_config())
            .unwrap()
    }

    pub fn store_connection_config(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .store_connection_config(storage, &self.connection_id, &get_dummy_connection_config())
            .unwrap()
    }

    pub fn store_ibc_config(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        let ibc_config = get_dummy_ibc_config();
        contract
            .store_ibc_config(storage, &self.network_id, &ibc_config)
            .unwrap();
    }

    pub fn store_network_fees(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .store_network_fees(storage, self.network_id.clone(), &get_dummy_network_fees())
            .unwrap()
    }

    pub fn store_counterparty_nid(&self, storage: &mut dyn Storage, contract: &CwIbcConnection) {
        contract
            .store_counterparty_nid(storage, &self.connection_id, "port_dst", &self.network_id)
            .unwrap()
    }

    pub fn configure_connection(&self, deps: DepsMut, contract: &CwIbcConnection) {
        let config = get_dummy_connection_config();

        contract
            .configure_connection(
                deps,
                self.connection_id.clone(),
                "port_dst".to_string(),
                self.network_id.clone(),
                config.client_id,
                config.timeout_height,
            )
            .unwrap()
    }
}
