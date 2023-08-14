# Javascore(Java Smart Contracts)

This folder contains the smart contracts for ICON-IBC in java. The gradle commands should be run inside this folder.

## Contracts

### [ibc protocol](ibc)

It is the implementation of the [core protocol](https://ibcprotocol.org/protocol/) of the Inter Blockchain Communication
(IBC) in Java smart contracts for ICON blockchain. The ibc protocol is modified to specifically work with ICON BTP
network. It sends BTP messages to communicate with other blockchains.

### [tendermint-light-client](lightclients/tendermint)

This is the light client implementation of tendermint in Java smart contracts.

### [xcall-connection](xcall-connection)

This contract is a utility contract to make a connection between the ibc protocol and the xcall contract.

### Requirement

- JDK 11+

### Build and Unit Test Contracts

The build command is used to compile java classes, create the jar files and run the unit tests.

```shell
# build and test specific contract
./gradlew ibc:build

# build and run unit test for all contracts
./gradlew build
```

### Optimized Jar

This step creates optimized Jar which can be used for deployment. The optimized jar is smaller compared to the jar. The
optimized jar is converted to bytes for deployment.

```shell
# Create optimized jar for deployment for specific contract
./gradlew ibc:optimizedJar

# Create optimized jar for all the contracts
./gradlew optimizedJar
```

### Integration

To run the integration tests, you need to have a local icon node running on your machine.
You can read more on how to start the local icon container [here](gochain-btp/README.md#start-the-container).
After the container is running, you can run the following gradle command.

```shell
./gradlew integrationTest
```

### Deploy Contract

```shell
./gradlew ibc:deployToLocal -PkeystoreName=<your_wallet_json> -PkeystorePass=<password>
```
