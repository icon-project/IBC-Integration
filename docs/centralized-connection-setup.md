Centralized Connection Deployment
===
## Prerequisities
- git
- [foundry](https://book.getfoundry.sh/getting-started/installation)
- [goloop cli](https://github.com/icon-project/goloop/tree/master/cmd/cli) <br/>
    or <br/>
    [openjdk](https://openjdk.org/install/) (version 11 or higher)

## Steps to deploy centralized connection on mainnet

### Clone the xcall-repository
```sh
git clone https://github.com/icon-project/xcall-multi.git
cd xcall-multi
# checkout to correct version
git checkout v1.2.0
export PROJECT_ROOT=$PWD
```

> NOTE: Relayer is the admin of the centralized contract.


**Follow the instruction to deploy respective contracts**

### Solidity

Assuming you want to deploy centralized to {CHAIN_NAME} chain.

```sh
cd $PROJECT_ROOT/contracts/evm
forge build
cp .env.example .env
```
Edit the .env file. You need to change the following fields:
```sh
PRIVATE_KEY=YOUR_PRIVATE_KEY # should have 0x prefix
ADMIN= # address which can upgrade contract
{CHAIN_NAME}_CENTRALIZED_RELAYER=YOUR_RELAYER_ADDRESS
```

Verify, the xcall address and RPC URL of {CHAIN_NAME} is correct.
The xcall address can be verified from [here](https://github.com/icon-project/xcall-multi/wiki/xCall-Deployment-Info)
```env
{CHAIN_NAME}_XCALL=
{CHAIN_NAME}_RPC_URL=
```

Now, to deploy the centralized-connection contract:
```sh
# check ./deploy_script.sh options for CHAIN_NAME
# env can be mainnet or testnet or local
# ./deploy_script.sh --contract centralized --deploy --env testnet --chain sepolia
./deploy_script.sh --contract centralized --deploy --env mainnet --chain {CHAIN_NAME} 
```
Save the centralized connection address. You can find the centralized connection address in the console as `Centralized Connection address: {CONTRACT_ADDRESS}`

**Set fees on the connection contract**: (Optional)

- This can be called only by the relayer.
```sh
cast send <connection_contract_address>  "setFee(string calldata networkId, uint256 messageFee, uint256 responseFee)" "0x1.icon" 10000000000000000 10000000000000000 --rpc-url <rpc_url> --private-key  <private-key>
```

**Change relayer address**: (Optional)

> If you need to change the relayer address,

- This can only be called by current relayer.
```sh
cast send <connection_contract_address> "setAdmin(address _address)" <new-relayer-address> --rpc-url <rpc_url> --private-key  <private-key>
```

### ICON
You can use one of the following methods to deploy the contract on ICON.

1. **Using gradlew**

```sh
cd $PROJECT_ROOT
cd contracts/javascore/centralized-connection
```

Update the constructor parameters in `build.gradle`. Put correct address on xCall and relayer field. The, xcall addresses can be found [here](https://github.com/icon-project/xcall-multi/wiki). 

```gradle
parameters {
    arg('_relayer', "<your-relayer-address>")
    arg('_xCall', "<xcall-address>")
}
```

Then, you can deploy it as:

```sh
cd $PROJECT_ROOT/contracts/javascore
./gradlew :centralized-connection:build
./gradlew :centralized-connection:optimizedJar

# testnet
# ./gradlew :centralized-connection:deployToLisbon -PkeystoreName=<absolute/path/to/your_wallet_json> -PkeystorePass=<password>

# mainnet
./gradlew :centralized-connection:deployToMainnet -PkeystoreName=<absolute/path/to/your_wallet_json> 
-PkeystorePass=<password>
```


2. **Using goloop**

- Deploy centralized contract
```sh
# fetch jarfile from release
wget https://github.com/icon-project/xcall-multi/releases/download/v1.2.0/centralized-connection-0.1.0-optimized.jar

# deploy contract
goloop rpc sendtx deploy centralized-connection-0.1.0-optimized.jar \
    --content_type application/java \
    --uri https://ctz.solidwallet.io/api/v3  \
    --nid 1 \
    --step_limit 2200000000 \
    --to cx0000000000000000000000000000000000000000 \
    --param _relayer=<your-relayer-address> \
    --param _xCall=<xcall-address>\
    --key_store <your_wallet_json> \
    --key_password <password>
```
- Set fee for centralized connection (Optional)
```sh
goloop rpc sendtx call \
        --uri https://ctz.solidwallet.io/api/v3 \
        --to <centralized-connection-address> \
        --nid 1 \
        --method setFee \
        --step_limit 50_000_000 \
        --param networkId=<destination-chain-id> \
        --param messageFee=<message-fee> \
        --param responseFee=<response-fee> \
        --param _data=6869206176616c616e636865212069276d206d6573736167652066726f6d2069636f6e \
        --key_store <your_relayer_wallet_json> \
        --key_password <relayer_password>
```

Now, that the contracts are deployed. You are now ready to setup the relay.
The guide to setup relay is [here](https://github.com/icon-project/centralized-relay/wiki/Installation)