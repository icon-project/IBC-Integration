[![Project Status: Initial Release](https://img.shields.io/badge/repo%20status-active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/xCall.svg?style=flat-square)](https://github.com/icon-project/xCall/blob/main/LICENSE)
[![Lines Of Code](https://img.shields.io/tokei/lines/github/icon-project/xCall?style=flat-square)](https://github.com/icon-project/xCall)
[![Version](https://img.shields.io/github/tag/icon-project/xCall.svg?style=flat-square)](https://github.com/icon-project/xCall)
![GitHub Workflow Status - cosmwasm](https://github.com/icon-project/xCall/actions/workflows/build-and-publish-cosmwasm.yml/badge.svg)
![GitHub Workflow Status - javascore](https://github.com/icon-project/xCall/actions/workflows/build-and-publish-javascore.yml/badge.svg)
| Language                            | Code Coverage                                  |
| ----------------------------------- | ---------------------------------------------- |
| [Java](./contracts/javascore)       | [![Java Cov][java-cov-badge]][java-cov-link]   |
| [Rust](./contracts/cosmwasm-vm)     | [![Rust Cov][rust-cov-badge]][rust-cov-link]   |

[java-cov-link]: https://app.codecov.io/gh/icon-project/xCall/tree/main/contracts/javascore
[rust-cov-link]: https://app.codecov.io/gh/icon-project/xCall/tree/main/contracts/cosmwasm-vm
[java-cov-badge]: https://codecov.io/gh/icon-project/xCall/branch/main/graph/badge.svg?token=KWDB59JITE&flag=java
[rust-cov-badge]: https://codecov.io/gh/icon-project/xCall/branch/main/graph/badge.svg?token=KWDB59JITE&flag=rust

# XCall
XCall is a cross chain messaging service built to mimic regular transaction flows across any interoperable solution.

For full xCall specification see [XCall Spec](./docs/adr/xcall.md).

## Building with xCall
For building dapps with xCall see official developer [docs](https://www.xcall.dev/).

### Project Structure
| Directory | Description |
|:----------|:------------|
| [/contracts/cosmwasm-vm](./contracts/cosmwasm-vm) | Includes contracts for cosmwasm based chains |
| [/contracts/evm](./contracts/evm) | Includes contracts for evm based chains |
| [/contracts/javascore](./contracts/javascore) | Includes contracts for ICON chain |
| [/docs](./docs) | Documentation |
| [/scripts](./scripts) | Scripts to automate task in project, for example build scripts, deploy scripts. |


## Available Connection implementations
* [IBC](https://github.com/icon-project/IBC-Integration/blob/main/docs/adr/XCall_IBC_Connection.md)
   * [Rust](https://github.com/icon-project/IBC-Integration/tree/main/contracts/cosmwasm-vm/cw-xcall-ibc-connection)
   * [Java](https://github.com/icon-project/IBC-Integration/tree/main/contracts/javascore/xcall-connection)
* [BTP](https://github.com/icon-project/btp2) is supported natively and does not need a connection contract.

## Building a xCall connection
If xCall is deployed, anyone can create a new connection contract to relay messages between xCall contracts.
To do this a connection contract has to be developed and deployed on both sides.

The base design for a connection can be found in the [xCall docs](./docs/adr/xcall.md#Connections)