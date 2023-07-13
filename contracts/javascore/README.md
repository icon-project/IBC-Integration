# Javascore

This folder contains the smart contracts for ICON-XCall in java.

### Requirement

- JDK 11+

### Build Contracts

```shell
# build specific contract
./gradlew xcall:build

# build all
./gradlew build
```

### Optimized Jar
```shell
./gradlew xcall:optimizedJar
```

### Deploy Contract
```shell
./gradlew xcall:deployToLocal -PkeystoreName=<your_wallet_json> -PkeystorePass=<password>
```
