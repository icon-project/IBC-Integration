# End-to-End Testing Setup and Demo

## Prerequisites

To run the demo, the following software needs to be installed.

* Docker compose \[[download](https://docs.docker.com/compose/install/)\]

### Setting up the Environment

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

### Additional steps for Apple Silicon 

* Build an `icon-chain` image
   
   ```bash
   git clone https://github.com/icon-project/goloop.git
   cd goloop
   make gochain-icon-image
   ``` 
   
* Build a `goloop` image
   
   ```bash
   git clone https://github.com/icon-project/goloop/
   cd goloop/ 
   make goloop-icon-image
   ```
   
* Build an `archway` or `neutron` image

   **For Archway:**
   
   ```bash
   git clone https://github.com/archway-network/archway/
   cd archway
   docker build -f Dockerfile.deprecated -t archway . --build-arg arch=aarch64
   ```
   
   **For Neutron:**
   
   ```bash
   git clone https://github.com/neutron-org/neutron.git
   cd neutron
   make build-docker-image
   ```

ℹ️ Change the image name and version of Archway/Neutron in `e2e-config.yaml` or `e2e-config-neutron.yaml`.

### Running the End-to-End Tests

1. Export the following system variables:

    - `E2E_CONFIG_PATH`: Absolute path to the config file (`e2e-config.yaml` for Archway or `e2e-config-neutron.yaml` for Neutron).
    - `GOLOOP_IMAGE_ENV`: Goloop image name.
    - `GOLOOP_IMAGE_TAG_ENV`: Goloop image version.

   Example:

   ```bash
   export TEST_CONFIG_PATH=/home/User/IBC-integration/sample-config-archway.yaml
   export GOLOOP_IMAGE_ENV=goloop
   export GOLOOP_IMAGE_TAG_ENV=latest
   ```

2. Run the End-to-End Tests:

   ```bash
   go test -v ./test/e2e
   ```
   