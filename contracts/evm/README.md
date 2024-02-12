## Solidity XCall

**Foundry is a blazing fast, portable and modular toolkit for Ethereum application development written in Rust.**

Foundry consists of:

-   **Forge**: Ethereum testing framework (like Truffle, Hardhat and DappTools).
-   **Cast**: Swiss army knife for interacting with EVM smart contracts, sending transactions and getting chain data.
-   **Anvil**: Local Ethereum node, akin to Ganache, Hardhat Network.
-   **Chisel**: Fast, utilitarian, and verbose solidity REPL.

## Documentation

https://book.getfoundry.sh/

## Usage

### Build

```shell
$ forge build
```

### Test


```shell
$ forge test -vv
```

To learn more about logs and traces, check out the documentation [here](https://book.getfoundry.sh/forge/tests?highlight=-vv#logs-and-traces).

To view all of the supported logging methods, check out the documentation [here](https://book.getfoundry.sh/reference/ds-test#logging).

### Gas Snapshots

```shell
$ forge snapshot
```

### Anvil
You can start the local EVM test network at any time:

```shell
$ anvil
```

### Deploy

```shell
./deploy_script.sh --contract <contract> --<action> --env <environment> --chain <chain1> <chain2> ... --version <filename-version>
```

Adapter(Wormhole and Layerzero) Configuration between 2 chains
```shell
./deploy_script.sh --contract <contract> --configure --env <environment> --chain <chain1> <chain2> 
```
Replace the placeholders with your specific values:

- `<contract>`: Contract to deploy or upgrade
- `<action>`: Choose either "--deploy" to deploy contracts or "--upgrade" to upgrade existing contracts.
- `<environment>`: Select the deployment environment ("mainnet," "testnet," or "local").
- `<chain1>`, `<chain2>`, ...: Specify one or more chains for deployment. Use "all" to deploy to all valid chains for the environment.
- `filename-version`: filename of new contract to upgrade like, CallServiceV2.sol (only needed in upgrade)

 Valid Options

- *Actions*: "deploy", "upgrade"
- *Environments*: "mainnet", "testnet", "local"
- *Contract Types*: "callservice" "wormhole" "layerzero" "centralized" "mock"

### Examples

#### Deploy the "callservice" contract to mainnet on Ethereum and Binance chains:

```shell
./deploy_script.sh --contract callservice --deploy --env mainnet --chain ethereum binance
```

#### Upgrade the "callservice" contract to testnet on all available chains:

```shell
./deploy_script.sh --contract callservice --upgrade --env testnet --chain all --version CallServiceV2.sol
```
### xCall Configurations

```shell
cast send <contract_address>  "setProtocolFee(uint256 _value)" <value> --rpc-url <rpc_url> --private-key  <private-key>
```

```shell
cast send <contract_address>  "setProtocolFeeHandler(address _addr)" <addr> --rpc-url <rpc_url> --private-key <private-key>
```

```shell
cast send <contract_address>  "setDefaultConnection(string memory _nid,address connection)" <nid> <connection> --rpc-url <rpc_url> --private-key <private-key>
```

### xCall Flow Test

#### Step 0: Copy Environment File

Start by copying the `.env.example` file to `.env`:

```bash
cp .env.example .env
```

#### Step 1: Deploy Contracts

```bash
#deploy xcall
./deploy_script.sh --contract callservice --deploy --env testnet --chain <source_chain> <destination_chain> 
```

```bash
#deploy adapter (wormhole layerzero centralized)
./deploy_script.sh --contract wormhole --deploy --env testnet --chain <source_chain> <destination_chain> 
```

```bash
#deploy dapp
./deploy_script.sh --contract mock --deploy --env testnet --chain <source_chain> <destination_chain> 
```
#### Step 3: Configure Connections

If you are using Wormhole or Layerzero for cross-chain communication, you will need to configure the connection. Execute the provided script to set up the connection. 

```bash
./deploy_script.sh --contract <adapter> --configure --env testnet --chain <source_chain> <destination_chain>
```
- *adapter*: "layerzero", "wormhole", "centralized"

#### Step 4: Add Connections in Dapp

In your dapp, add the connections for cross-chain communication.

```bash
forge script DeployCallService  -s "addConnection(string memory chain1, string memory chain2)" <source_chain> <destination_chain> --fork-url <source_chain> --broadcast        
forge script DeployCallService  -s "addConnection(string memory chain1, string memory chain2)" <destination_chain> <source_chain> --fork-url <destination_chain> --broadcast   
```

#### Step 5: Execute Test
```bash
$ ./test_xcall_flow.sh --src <source_chain> --dest <destination_chain> --fee <value>
```

- `--fee <value>`: Sets the transaction fee (in wei). The value must be a number.
- `--src <source_chain>`: Sets the source chain for the transaction. Valid chain options are `fuji`, `bsctest`, `base_goerli`, `optimism_sepolia`, and `arbitrum_goerli`.
- `--dest <destination_chain>`: Sets the destination chain for the transaction. Valid chain options are `fuji`, `bsctest`, `base_goerli`, `optimism_sepolia`, and `arbitrum_goerli`.

### Cast
Set the CONTRACT_ADDRESS variable in your terminal:

```sh
export CONTRACT_ADDRESS=<your-contract-address>
```

Call initialize on the contract

```sh
cast send $CONTRACT_ADDRESS "initialize(string)" "ENTER Chain NID HERE" --private-key $PRIVATE_KEY
```

We can then use cast to interact with it.

For read operations, we can use cast call: For Example:

```sh
cast call $CONTRACT_ADDRESS "admin()(address)"
```

For transactions, we can use cast send, passing in a private key and any arguments:

```sh
cast send $CONTRACT_ADDRESS "setAdmin(address)" 0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc --private-key $PRIVATE_KEY
```

To test that the greeting has been updated, run the `call` command again:

```sh
cast call $CONTRACT_ADDRESS "admin()(address)"
```


## Installing packages

You can install packages using the `forge install` command.

To try this out, let's install OpenZeppelin Contracts, then we'll use them to create an ERC721 token:

> You may need to add and commit any changes to your code to `git` in order to run the install script.

```sh
forge install OpenZeppelin/openzeppelin-contracts
```

Next, create a file named `remappings.txt` in the root of the project and add the following configuration:

```
@openzeppelin/=lib/openzeppelin-contracts/
```

This will allow us to easily import with the following syntax:

```solidity
import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
```

You can view all of the automatically inferred remappings for the project by running the following command:

```sh
forge remappings
```


### Test coverage

You can check for test coverage by running the `coverage` command:

```sh
forge coverage
```

To debug in more details what has not been covered, use the `debug` report:

```sh
forge coverage --report debug
```



### Help

```shell
$ forge --help
$ anvil --help
$ cast --help
```