# Javascore

This folder contains the smart contracts for ICON-IBC in java.

### Requirement

- JDK 11+

### Build Contracts

```shell
# build specific contract
./gradlew ibc:build

# build all
./gradlew build
```

### Optimized Jar
```shell
./gradlew ibc:optimizedJar
```

### Deploy Contract
```shell
./gradlew ibc:deployToLocal -PkeystoreName=<your_wallet_json> -PkeystorePass=<password>
```
