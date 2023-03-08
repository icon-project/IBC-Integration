# IBC-Integration
The Inter-Blockchain Communication protocol (IBC) is an end-to-end, connection-oriented, stateful protocol for reliable, ordered, and authenticated communication between heterogeneous blockchains arranged in an unknown and dynamic topology. xCall, a standard for generic cross-chain messaging along with IBC provides dynamic and coherent solution for inter-connected dapps.

Additional information on how IBC works can be found [here](https://ibc.cosmos.network/). and xCall spec is defined [here](https://github.com/icon-project/IIPs/blob/master/IIPS/iip-52.md)

# Table of Content

- [IBC INTEGRATION](#ibc-integration)
- [Table of Content](#table-of-content)
  - [About  ](#about--)
  - [Getting Started  ](#getting-started--)
    - [Prerequiste](#prerequiste)
    - [Running the tests](#testing)

## About <a name = "about"> </a>

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
| Directory | Description |
|:----------|:------------|
| [/contracts/cosmwasm-vm](./contracts/cosmwasm-vm) | Includes contrcats for multiple chains |
| /contracts/evm | Includes contrcats for multiple chains |
| /contracts/javascore | Includes contrcats for multiple chains |
| /docs | Documentation |
| /libraries/rust | Common rust libraries used across multiple integrations |
| /proto | Proto files used for IBC |
| /resources | Static resources in project . For example images , bin files, etc |
| /scripts | Scripts to automate task in project , for example build scripts, deploy scripts. |
| /test | Test Framework and Test Suite including e2e test and functional test |
| /utils | utilities used for build, setup, CI/CD |

## Testing <a name = "testing"> </a>

### Integration Testing

```
go test -v ./test/integration --args -config=<path to config.json>
```


### E2E Testing

```
go test -v ./test/e2e --args -config=<path to config.json>
```