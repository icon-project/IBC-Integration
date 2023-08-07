[![Project Status: Initial Release](https://img.shields.io/badge/repo%20status-active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration/blob/main/LICENSE)
[![Lines Of Code](https://img.shields.io/tokei/lines/github/icon-project/IBC-Integration?style=flat-square)](https://github.com/icon-project/IBC-Integration)
[![Version](https://img.shields.io/github/tag/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration)

| Language                            | Code Coverage                                  |
| ----------------------------------- | ---------------------------------------------- |
| [Java](./contracts/javascore)       | [![Java Cov][java-cov-badge]][java-cov-link]   |
| [Rust](./contracts/cosmwasm-vm)     | [![Rust Cov][rust-cov-badge]][rust-cov-link]   |

[java-cov-link]: https://app.codecov.io/gh/icon-project/IBC-Integration/tree/main/contracts/javascore
[rust-cov-link]: https://app.codecov.io/gh/icon-project/IBC-Integration/tree/main/contracts/cosmwasm-vm
[java-cov-badge]: https://codecov.io/gh/icon-project/IBC-Integration/branch/main/graph/badge.svg?token=8KX6y8aGom&flag=java
[rust-cov-badge]: https://codecov.io/gh/icon-project/IBC-Integration/branch/main/graph/badge.svg?token=8KX6y8aGom&flag=rust

# IBC-Integration
The Inter-Blockchain Communication protocol (IBC) is an end-to-end, connection-oriented, stateful protocol for reliable, ordered, and authenticated communication between heterogeneous blockchains arranged in an unknown and dynamic topology. xCall, a standard for generic cross-chain messaging along with IBC provides dynamic and coherent solution for inter-connected dapps.

Additional information on how IBC works can be found [here](https://ibc.cosmos.network/). and xCall spec is defined [here](https://github.com/icon-project/IIPs/blob/master/IIPS/iip-52.md)

# Table of Content

- [IBC INTEGRATION](#ibc-integration)
- [Table of Content](#table-of-content)
  - [About  ](#about--)
  - [Installation  ](#installation--)
  - [Getting Started  ](#getting-started--)
    - [Prerequiste](#prerequiste)
    - [Running the tests](#testing)

## About <a name = "about"> </a>

## Installation <a name = "installation"> </a>
This project uses git submodules. Use the following command to clone this repository including the required submodules. 
```sh
git clone --recurse-submodules https://github.com/icon-project/IBC-Integration
```
Or To update submodules, run the following command,
```
git submodule init
git submodule update --remote
```

## Getting Started <a name = "getting_started"> </a>

Terminologies used in this project.
  
- [ibc packet](./docs/terminologies/ibc_terminologies.md)
- [ibc message](./docs/terminologies/ibc_terminologies.md)
- [openInit](./docs/terminologies/ibc_terminologies.md)
- [openTry](./docs/terminologies/ibc_terminologies.md)
- [openAck](./docs/terminologies/ibc_terminologies.md)
- [openConfirm](./docs/terminologies/ibc_terminologies.md)

### Available Integrations
- ICON
- Archway

### Project Structure
| Directory | Description                                                                   |
|:----------|:------------------------------------------------------------------------------|
| [/contracts/cosmwasm-vm](./contracts/cosmwasm-vm) | Includes contracts for cosmwasm based chains                                  |
| [/contracts/evm](./contracts/evm) | Includes contracts for evm based chains                                       |
| [/contracts/javascore](./contracts/javascore) | Includes contracts for ICON chain                                             |
| [/docs](./docs) | Documentation                                                                 |
| [/libraries/rust](./libraries/rust) | Common rust libraries used across multiple integrations                       |
| [/proto](./proto) | Proto files used for IBC                                                      |
| [/resources](./resources) | Static resources in project. For example images, bin files, etc               |
| [/scripts](./scripts) | Scripts to automate task in project, for example build scripts, deploy scripts|
| [/test](./test) | Test Framework and Test Suite including e2e test and functional test          |
| [/utils](./utils) | Utilities used for build, setup, CI/CD                                        |

## Testing <a name = "testing"> </a>

### Integration Testing

```
go test -v ./test/integration --args -config=<path to config.json>
```


### End-to-End Testing for the System

[End to End Testing Setup](./docs/e2e_test_setup.md)