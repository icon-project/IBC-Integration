[![Project Status: Initial Release](https://img.shields.io/badge/repo%20status-active-green.svg?style=flat-square)](https://www.repostatus.org/#active)
[![License: Apache-2.0](https://img.shields.io/github/license/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration/blob/main/LICENSE)
[![Lines Of Code](https://img.shields.io/tokei/lines/github/icon-project/IBC-Integration?style=flat-square)](https://github.com/icon-project/IBC-Integration)
[![Version](https://img.shields.io/github/tag/icon-project/IBC-Integration.svg?style=flat-square)](https://github.com/icon-project/IBC-Integration)

| Language                        | Code Coverage                                | Test                                            |
| ------------------------------- | -------------------------------------------- | ----------------------------------------------- |
| [Java](./contracts/javascore)   | [![Java Cov][java-cov-badge]][java-cov-link] | [![Java Test][java-test-badge]][java-test-link] |
| [Rust](./contracts/cosmwasm-vm) | [![Rust Cov][rust-cov-badge]][rust-cov-link] | [![Rust Test][rust-test-badge]][rust-test-link] |

[java-cov-link]: https://app.codecov.io/gh/icon-project/IBC-Integration/tree/main/contracts/javascore
[rust-cov-link]: https://app.codecov.io/gh/icon-project/IBC-Integration/tree/main/contracts/cosmwasm-vm
[java-cov-badge]: https://codecov.io/gh/icon-project/IBC-Integration/branch/main/graph/badge.svg?token=8KX6y8aGom&flag=java
[rust-cov-badge]: https://codecov.io/gh/icon-project/IBC-Integration/branch/main/graph/badge.svg?token=8KX6y8aGom&flag=rust
[java-test-badge]: https://github.com/icon-project/IBC-Integration/actions/workflows/java-contracts-test.yml/badge.svg
[java-test-link]: https://github.com/icon-project/IBC-Integration/actions/workflows/java-contracts-test.yml
[rust-test-badge]: https://github.com/icon-project/IBC-Integration/actions/workflows/basic-rust.yml/badge.svg
[rust-test-link]: https://github.com/icon-project/IBC-Integration/actions/workflows/basic-rust.yml

# IBC-Integration

The Inter-Blockchain Communication protocol (IBC) is an end-to-end, connection-oriented, stateful protocol for reliable,
ordered, and authenticated communication between heterogeneous blockchains arranged in an unknown and dynamic topology.
xCall, a standard for generic cross-chain messaging along with IBC provides dynamic and coherent solution for
interconnected dapps.

Additional information on how IBC works can be found [here](https://ibc.cosmos.network/). and xCall spec is
defined [here](https://github.com/icon-project/xCall/blob/main/docs/adr/xcall.md)

# Table of Contents

- [IBC INTEGRATION](#ibc-integration)
- [Table of Content](#table-of-content)
  - [About](#about--)
  - [Installation](#installation--)
  - [Getting Started](#getting-started--)
    - [Prerequisite](#prerequiste)
    - [Build](#build)
    - [Running the tests](#testing)
    - [Deploy](#deploy)
  - [Developing IBC Dapp](#developing-ibc-dapp--)
  - [Contributing](#contributing--)
  - [License](#license--)
  - [Contact](#contact--)

## About <a name = "about"> </a>

Relayer for these contracts deviates slightly from official cosmos relayer due to the fact that ICON uses BTP Blocks as storage for messages and also on cosmwasm side the ibc host is deployed as contracts rather than the native ibc host module of cosmos chain. Relayer for ibc-icon can be found [here](https://github.com/icon-project/ibc-relay).
The deviation from cosmos relayer is documented [here](https://github.com/icon-project/ibc-relay/blob/main/docs/deviations_from_ibc.md).

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

Make sure you have following installed on your machine to build the contracts or you can use docker.

### Prerequisites

- Go (at least version 1.20)
- Rust (version 1.68)
- Wasm-Opt (version 110)
- Java (JDK 11)
- Docker (for running tests)

### Available Integrations

- ICON
- Archway

### Project Structure

| Directory                                         | Description                                                                    |
| :------------------------------------------------ | :----------------------------------------------------------------------------- |
| [/contracts/cosmwasm-vm](./contracts/cosmwasm-vm) | Includes contracts for cosmwasm based chains                                   |
| [/contracts/evm](./contracts/evm)                 | Includes contracts for evm based chains                                        |
| [/contracts/javascore](./contracts/javascore)     | Includes contracts for ICON chain                                              |
| [/docs](./docs)                                   | Documentation                                                                  |
| [/libraries/rust](./libraries/rust)               | Common rust libraries used across multiple integrations                        |
| [/proto](./proto)                                 | Proto files used for IBC                                                       |
| [/resources](./resources)                         | Static resources in project. For example images, bin files, etc                |
| [/scripts](./scripts)                             | Scripts to automate task in project, for example build scripts, deploy scripts |
| [/test](./test)                                   | Test Framework and Test Suite including e2e test and functional test           |
| [/utils](./utils)                                 | Utilities used for build, setup, CI/CD                                         |

## Local Setup and End-to-End Testing Using Dive CLI

For Setting up nodes , deploy contracts and end-to-end testing follow setps provided [here](./docs/ibc-setup-using-dive-cli.md)

## Build <a name = "build"> </a>

Run following command on root directory to build the rust contracts. The built wasm files will be available in the artifacts directory in the root.

```
./scripts/optimize-cosmwasm.sh
```

To build the java contracts run following command.

```
make optimize-jar
```

To build all contracts using docker follow steps below
Step1: Update git submodules:

```
git submodule init
git submodule update --remote
```

Step2: Run following commands to build the builder image and compile contracts.

```
make build-builder-img
make optimize-build

```

## Testing <a name = "testing"> </a>

### End-to-End Testing for the System

[End-to-End Testing Setup](./docs/e2e-integration-test-setup.md)

## Deploy <a name = "deploy"> </a>

For deployment and usage follow steps provided [here](https://github.com/izyak/icon-ibc/tree/master).

## Developing IBC Dapp <a name = "developing-ibc-dapp"> </a>

To build dapp that is compatible with our smart contract based IBC Host you can follow the docs provided in mock ibc dapp samples.
Sample for cosmwasm contract [here](./contracts/cosmwasm-vm/cw-mock-ibc-dapp/README.md).
Sample for icon contract [here](./contracts/javascore/modules/mockapp/src/main/java/ibc/mockapp/MockApp.java)

## Contributing <a name = "contributing"> </a>

We highly value community contributions and appreciate your interest in improving the IBC Integration for ICON Project. This section outlines the steps you should follow to contribute effectively.

### Bug Reports and Feature Requests

If you encounter a bug or have an idea for a new feature, please follow these steps:

#### Bug Reports

1. Before submitting a bug report, search the [existing issues](https://github.com/icon-project/IBC-Integration/issues) to see if the bug has already been reported. If you find a similar issue, you can add relevant details in the comments.

2. If the bug hasn't been reported yet, [open a new issue](https://github.com/icon-project/IBC-Integration/issues/new/choose) with a clear and descriptive title. Provide as much detail as possible, including:

   - A clear description of the bug and its impact.
   - Steps to reproduce the bug.
   - Your environment details (OS, ICON node version, etc.).
   - Any relevant error messages or logs.

3. Assign appropriate labels to the issue, such as "bug" and any other relevant tags.

#### Feature Requests

1. Before submitting a feature request, again, check the [existing issues](https://github.com/icon-project/IBC-Integration/issues) to ensure that the feature hasn't already been requested. If you find a similar request, you can add your insights in the comments.

2. To submit a new feature request, [open a new issue](https://github.com/icon-project/IBC-Integration/issues/new/choose) and select the "Feature Request" template. Provide a clear and comprehensive description of the feature you're proposing, including:

   - The problem the feature aims to solve.
   - How the feature would work.
   - Any potential benefits or use cases.

3. Assign appropriate labels to the issue, such as "enhancement" or "feature request," and any other relevant tags.

### Pull Requests

If you're interested in contributing code to the project, follow these steps to submit a pull request (PR):

1. **Fork the Repository:** Click the "Fork" button at the top of the [repository page](https://github.com/icon-project/IBC-Integration) to create your own fork of the project. This will allow you to work on your changes in your own copy of the repository.

2. **Create a Branch:** Create a new branch in your forked repository that will contain your changes. Naming the branch according to the changes you're making is a good practice (e.g., `feature/new-feature` or `bugfix/fix-issue-123`).

3. **Make Changes:** Make your changes in the new branch. Follow the project's coding standards, and write clear and concise commit messages.

4. **Test Your Changes:** If applicable, ensure that your changes work as intended and do not introduce new bugs. If tests exist in the project, make sure to run them.

5. **Open a Pull Request:** When you're ready to submit your changes, [open a new pull request](https://github.com/icon-project/IBC-Integration/compare) from your branch to the `main` branch of the main repository. Provide a detailed description of your changes, including the problem you're solving, your solution, and any relevant context.

6. **Review and Iteration:** Your pull request will be reviewed by the project maintainers. Be prepared to address any feedback or changes requested by the reviewers. Collaboration and constructive discussion are key.

7. **Merge:** Once your pull request passes the review process and meets the project's standards, it will be merged into the `main` branch.

### Code of Conduct

Contributors are expected to adhere to the project's [Code of Conduct](CODE_OF_CONDUCT.md) at all times. This ensures a respectful and inclusive environment for all contributors and participants.

We appreciate your dedication to contributing to the IBC Integration for ICON Project. Your efforts help improve the project's quality and expand its capabilities. Thank you for being a part of our community!

## License <a name = "license"> </a>

This project is licensed under the [Apache 2.0](LICENSE).

## Contact <a name = "contact"> </a>

If you need assistance, have questions, or want to collaborate on this project, feel free to contact the team at our [community chat](https://discord.com/invite/7a75Hf3cFm).

---

Thank you for your interest in the IBC Integration for ICON Project. We look forward to your contributions and the positive impact they will bring to the ICON blockchain and its interoperability capabilities!
