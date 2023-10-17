# Setting Up IBC Using DIVE CLI

## Prerequisite

- [Docker installed and running](https://docs.docker.com/get-docker/)

- [Kurtosis installed and running ](https://docs.kurtosis.com/install#ii-install-the-cli) or [(upgrading to the latest)](https://docs.kurtosis.com/upgrade)

- [Install DIVE CLI ](https://github.com/HugoByte/DIVE#installing-dive-cli)

## Running Setup

- To setup IBC setup between Icon and Archway run the following command:

  ```shell
  dive bridge ibc --chainA icon --chainB archway
  ```

  - Spins icon and archway node
  - Opens Btp Network for BTP Blocks
  - Deploys the Contracts necessary for the IBC connection
  - Starts the relay

- To setup IBC setup between Icon and Archway run the following command:

  ```shell
  dive bridge ibc --chainA icon --chainB neutron
  ```

  - Spins icon and neutron node
  - Opens Btp Network for BTP Blocks
  - Deploys the Contracts necessary for the IBC connection
  - Starts the relay

## End-to-End Testing

To run end-to-end testing script please follow from the second setp mentioned [here](https://github.com/HugoByte/DIVE/blob/main/test/README.md#end-to-end-icon---archway-demo)
