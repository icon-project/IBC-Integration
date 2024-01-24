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

Replace the placeholders with your specific values:

- `<contract>`: Contract to deploy or upgrade
- `<action>`: Choose either "--deploy" to deploy contracts or "--upgrade" to upgrade existing contracts.
- `<environment>`: Select the deployment environment ("mainnet," "testnet," or "local").
- `<chain1>`, `<chain2>`, ...: Specify one or more chains for deployment. Use "all" to deploy to all valid chains for the environment.
- `filename-version`: filename of new contract to upgrade like, CallServiceV2.sol (only needed in upgrade)

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

### Adapter Configurations(Wormhole and Layerzero)

```shell
./deploy_script.sh --contract <contract> --configure --env <environment> --chain <chain1> <chain2> 
```
### Valid Options

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