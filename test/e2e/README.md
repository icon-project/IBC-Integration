# Setup

## Docker images

make gochain-icon-image from:
https://github.com/AntonAndell/goloop

make docker from relay repo where you have to add the following line to the Dockerfile
COPY ./godWallet.json /home/relayer/keys/godwallet.json

## Contracts

Currently the relay is slightly behind on the newest changes to ICON
used optimized jars from the gochain-btp repo. Dapp can be used from this repo

## Config Setup

Config supports expanding with any available runtime OS environment variables,
e.g. ${HOME}, ${USER}, ${PWD}, ${BASE_PATH}

`$BASE_PATH` environment variable is also available which points to the root of the project.
