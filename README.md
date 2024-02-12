[![Project Status: Initial Release](https://img.shields.io/badge/repo%20status-active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/xcall-multi.svg?style=flat-square)](https://github.com/icon-project/xcall-multi/blob/main/LICENSE)
[![Lines Of Code](https://img.shields.io/tokei/lines/github/icon-project/xcall-multi?style=flat-square)](https://github.com/icon-project/xcall-multi)
[![Version](https://img.shields.io/github/tag/icon-project/xcall-multi.svg?style=flat-square)](https://github.com/icon-project/xcall-multi)
![GitHub Workflow Status - cosmwasm](https://github.com/icon-project/xcall-multi/actions/workflows/build-and-publish-cosmwasm.yml/badge.svg)
![GitHub Workflow Status - javascore](https://github.com/icon-project/xcall-multi/actions/workflows/build-and-publish-javascore.yml/badge.svg)
| Language                            | Code Coverage                                              |
| ----------------------------------- | ---------------------------------------------------------- |
| [Java](./contracts/javascore)       | [![Java Cov][java-cov-badge]][java-cov-link]               |
| [Rust](./contracts/cosmwasm-vm)     | [![Rust Cov][rust-cov-badge]][rust-cov-link]               |
| [Solidity](./contracts/evm)         | [![Solidity Cov][solidity-cov-badge]][solidity-cov-link]   |

[java-cov-link]: https://app.codecov.io/gh/icon-project/xcall-multi/tree/main/contracts/javascore
[rust-cov-link]: https://app.codecov.io/gh/icon-project/xcall-multi/tree/main/contracts/cosmwasm-vm
[solidity-cov-link]: https://app.codecov.io/gh/icon-project/xcall-multi/tree/main/contracts/evm
[java-cov-badge]: https://codecov.io/gh/icon-project/xcall-multi/branch/main/graph/badge.svg?token=KWDB59JITE&flag=java
[rust-cov-badge]: https://codecov.io/gh/icon-project/xcall-multi/branch/main/graph/badge.svg?token=KWDB59JITE&flag=rust
[solidity-cov-badge]: https://codecov.io/gh/icon-project/xcall-multi/branch/main/graph/badge.svg?token=KWDB59JITE&flag=solidity

# xcall-multi
xcall-multi is a cross chain messaging service built to mimic regular transaction flows across any interoperable solution.

For full xcall-multi specification see [xcall-multi Spec](./docs/adr/xcall.md).

## Contract Addresses
See [Live Contracts](https://github.com/icon-project/xcall-multi/wiki/xCall-Deployment-Info) for current testnet and mainnet deployments of the xcall-multi contracts on all chains where xCall is deployed. 

## Building with xcall-multi
For building dapps with xcall-multi see official developer [docs](https://www.xcall.dev/).

## Available Connection implementations
* [IBC](https://github.com/icon-project/IBC-Integration/blob/main/docs/adr/xcall-multi_IBC_Connection.md)
   * [Rust](https://github.com/icon-project/IBC-Integration/tree/main/contracts/cosmwasm-vm/cw-xcall-ibc-connection)
   * [Java](https://github.com/icon-project/IBC-Integration/tree/main/contracts/javascore/xcall-connection)
* [BTP](https://github.com/icon-project/btp2) is supported natively and does not need a connection contract.
* Centralized Connection
   * [Java](https://github.com/icon-project/xcall-multi/tree/main/contracts/javascore/centralized-connection)
   * [Solidity](https://github.com/icon-project/xcall-multi/blob/main/contracts/evm/contracts/adapters/CentralizedConnection.sol)
   * [Rust](https://github.com/icon-project/xcall-multi/tree/main/contracts/cosmwasm-vm/cw-centralized-connection)
* Wormhole
   * [Solidity](https://github.com/icon-project/xcall-multi/blob/main/contracts/evm/contracts/adapters/WormholeAdapter.sol)
* LayerZero Adapter
   * [Solidity](https://github.com/icon-project/xcall-multi/blob/main/contracts/evm/contracts/adapters/LayerZeroAdapter.sol)

## Building a xcall-multi connection
If xcall-multi is deployed, anyone can create a new connection contract to relay messages between xcall-multi contracts.
To do this a connection contract has to be developed and deployed on both sides.
The base design for a connection can be found in the [xcall-multi docs](./docs/adr/xcall.md#Connections)