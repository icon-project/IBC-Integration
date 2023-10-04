[![Maven Central](https://maven-badges.herokuapp.com/maven-central/xyz.venture23/xcall-lib/badge.svg)](https://search.maven.org/search?q=g:xyz.venture23%20a:xcall-lib)
# Javascore

This repo contains the smart contracts for ICON-XCall in java.

# xcall-lib library

You can include this package from [Maven Central](https://central.sonatype.com/search?q=g:xyz.venture23%20a:xcall-lib&smo=true) by adding the following dependency in your build.gradle.

```
implementation group: 'xyz.venture23', name: 'xcall-lib', version: '0.1.2'
```

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
