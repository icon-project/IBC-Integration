# IBC Contracts Deployment

## Building Javascore Contracts
Compile Java smart contracts using a Gradle task located at `./contracts/javascore` within the project repository.

```
./gradlew clean build
   ./gradlew optimizedJar       
```
## Deploying Javascore Contracts
Javascore contracts can be deployed either by using goloop cli tools or using gradle task.  
### Deploy using goloop cli tool on local icon network:

```
# Installing goloop
go install github.com/icon-project/goloop/cmd/goloop@latest

```

```
tx_call_args_icon_common=" --uri $ICON_NODE  --nid $ICON_SOURCE_ID  --step_limit 100000000000 --key_store $ICON_WALLET --key_password *** "

goloop rpc sendtx deploy $XCALL_MULTI_ICON \
			--content_type application/java \
			--to cx0000000000000000000000000000000000000000 \
			--param networkId=$ICON_DEFAULT_NID \
			$tx_call_args_icon_common 

```

Where,

ICON_NODE=`http://localhost:9082/api/v3`  
XCALL_MULTI_ICON=`xcall-multi-protocol-0.1.0-optimized.jar`  
ICON_SOURCE_ID=`7`  
ICON_WALLET=`godWallet.json`  


### Deploy using gradle task:

```
./gradlew deployToLocal -PdeploymentENV=local -PkeystoreName=$UAT_KEYSTORE_PATH -PkeystorePass=$UAT_PASSWD
```

## Build Cosmwasm Contracts

Execute the following command to generate artifacts in the `./artifacts/archway` directory.

```
make optimize-cosmwasm
```

## Deploy Cosmwasm Contracts on Archway Network

### Deploy using script
```
bash ./scripts/deploy_cosmwasm.sh
```
### Using archwayd Command
```

archwayd tx wasm store $CONTRACT_WASM --keyring-backend test --from $WALLET --node $ENDPOINT --chain-id $CHAIN_ID --gas-prices 0.02$TOKEN --gas auto --gas-adjustment 1.3 -y --output json -b block


```


WALLET=`fd` # This account is default while running the chain on docker.  
ENDPOINT=`http://localhost:26657`  
CHAIN_ID=`localnet`  
TOKEN=`stake`  

