# IBC INTEGRATION

To integrate the IBC between Icon and Archway

# Table of Content

- [IBC INTEGRATION](#ibc-integration)
- [Table of Content](#table-of-content)
  - [About  ](#about--)
  - [Getting Started  ](#getting-started--)
    - [Prerequiste](#prerequiste)
    - [Running the tests](#running-the-tests)
    - [Breakdown into end to end test](#breakdown-into-end-to-end-test)
  - [Deployment  ](#deployment--)
  - [Usage  ](#usage--)
  - [Built Using  ](#built-using--)
  - [Contributing  ](#contributing--)
  - [Acknowledgments  ](#acknowledgments--)

## About <a name = "about"> </a>

Integrating the Icon and Archway chain, where testnets message are transfered by the xCall.

## Getting Started <a name = "getting_started"> </a>

<!-- This will give the instructions to get you the copy of the project up and running on your local machine for development and testing purposes.  -->

Terminologies used in this project.
  
- [ibc packet](../docs/terminologies/ibc_terminologies.md)
- [ibc message](../docs/terminologies/ibc_terminologies.md)
- [openInit](./docs/terminologies/ibc_terminologies.md)
- [openTry](./docs/terminologies/ibc_terminologies.md)
- [openAck](./docs/terminologies/ibc_terminologies.md)
- [openConfirm](./docs/terminologies/ibc_terminologies.md)

### Prerequiste

<!-- What things needed to install the software and how it is installed.
```
Examples
``` -->
To go with the flow of the project, we should install some softwares.

- To install Rust in Linux and Mac,

First, [install rustup](https://rustup.rs/). Once installed, make sure you have the wasm32 target:
  
```shell
rustup default stable
cargo version
# If this is lower than 1.55.0+, update
rustup update stable

rustup target list --installed
rustup target add wasm32-unknown-unknown
```

For windows,
Download and run, [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

### Running the tests

Explain how to run the automated tests in the project

### Breakdown into end to end test

Explaing what these tests test
```
Example
```

## Deployment <a name = "deployment"> </a>

Steps to be included on how the project has been deployed.

-  
## Usage <a name = "usage"> </a>

IBC stands for Interblockchain Communication Protocol, which have been widely used to allow direct communication and asset trading between independent blockchains.

The Cosmos ecosystem has a vision of creating the "internet of blockchains," or a network of independent chains that can communicate in a decentralized way. To reach this goal, the Inter‐Blockchain Communication protocol (**IBC**) was created.

As part of the CosmosSDK-based ecosystem, Icon integrates the IBC protocol to Archway, our CosmWasm smart contract-based decentralized exchange with multi-chain interoperability, optimal speed and lots of liquidity options for users.

## Built Using <a name = "built_using"> </a>

Listing the frameworks or the environment in which we used to run our project

- [cosmwasm contract](https://book.cosmwasm.com/)
  
## Contributing <a name = "contributing"> </a>

Contributors name should be listed 
  
- 

## Acknowledgments <a name = "acknowledgement"> </a>

- References




















