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

#### Setting up the Environment

1. Build the `ibc-relayer` image:

   ```bash
   $ git clone https://github.com/icon-project/ibc-relay/
   $ cd ibc-relay/
   $ docker build -t relayer .
   ```

2. Build the builder image for bundling contracts:

   ```bash
   make build-builder-img
   ```

3. Optimize contracts:

   Before starting to bundle contracts, update all submodules:

   ```bash
   git submodule init
   git submodule update --remote
   ```

   Start bundling Icon and Rust contracts:

   ```bash
   make optimize-build
   ```

#### Additional Steps for Apple Silicon

**Build an `icon-chain` image**

```bash
git clone https://github.com/icon-project/goloop.git
cd goloop
make gochain-icon-image
``` 

**Build a `goloop` image**

```bash
git clone https://github.com/icon-project/goloop/
cd goloop/ 
make goloop-icon-image
```

**Build an `archway` or `neutron` image**

> For Archway:

```bash
git clone https://github.com/archway-network/archway/
cd archway
docker build -f Dockerfile.deprecated -t archway .
```

> For Neutron:

```bash
git clone https://github.com/neutron-org/neutron.git
cd neutron
make build-docker-image
```

Update the image name and versions of Archway/Neutron in `e2e-config.yaml` or `e2e-config-neutron.yaml`.

#### Running the End-to-End Tests

1. Export the following system variables:

    - `E2E_CONFIG_PATH`: Absolute path to the config file (`e2e-config.yaml` for Archway or `e2e-config-neutron.yaml` for Neutron).
    - `GOLOOP_IMAGE_ENV`: Goloop image name.
    - `GOLOOP_IMAGE_TAG_ENV`: Goloop image version.

   Example:

   ```bash
   export E2E_CONFIG_PATH=/home/User/IBC-integration/e2e-config.yaml
   export GOLOOP_IMAGE_ENV=goloop
   export GOLOOP_IMAGE_TAG_ENV=latest
   ```

2. Run the End-to-End Tests:

   ```bash
   go test -v ./test/e2e
   ```
   