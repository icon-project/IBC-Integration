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
   $ cd ..
   ```

2. Build the builder image in IBC Integration repo for bundling contracts:

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
   cd .. 
   ``` 

* Build a `goloop` image

   ```bash
   git clone https://github.com/icon-project/goloop/
   cd goloop/ 
   make goloop-icon-image
   cd ..
   ```

* Build an `archway` or `neutron` image

  **For Archway:**

   ```bash
   git clone https://github.com/archway-network/archway/
   cd archway
   docker build -f Dockerfile.deprecated -t archway . --build-arg arch=aarch64
   cd ..
   ```

  **For Neutron:**

   ```bash
   git clone https://github.com/neutron-org/neutron.git
   cd neutron
   make build-docker-image
   cd ..
   ```

ℹ️ Change the image name and version of Archway/Neutron in `e2e-config.yaml` or `e2e-config-neutron.yaml`.

### Running IBC Integration System Tests

To conduct tests for IBC integration system, carefully adhere to the provided instructions:

#### 1. Configure Environment Variables

Prior to initiating the tests, ensure proper configuration of essential environment variables, which play a pivotal role in the testing process:

- **`E2E_CONFIG_PATH`**: Set this variable to the absolute path of your chosen configuration file. For Archway, utilize `sample-config-archway.yaml`, and for Neutron, employ `sample-config-neutron.yaml`.
- **`GOLOOP_IMAGE_ENV`**: Indicate the name of the Goloop image.
- **`GOLOOP_IMAGE_TAG_ENV`**: Specify the version of the Goloop image.

Here's an example of environment variable configuration:

```bash
export E2E_CONFIG_PATH=/home/User/IBC-integration/sample-config-archway.yaml
export GOLOOP_IMAGE_ENV=goloop-icon
export GOLOOP_IMAGE_TAG_ENV=latest
```

#### 2. Run the Test Script

Use the appropriate command to run the test suite. Depending on your specific testing requirements, you can use the following command:

```bash
./scripts/execute-test.sh [options]
```

Replace `[options]` with any command-line options or arguments that the test script supports. To view more details about available options and usage, run the following command:

```bash
./scripts/execute-test.sh --help
```

This will display the available options, explain how to use them, and provide additional information about running the tests.


#### 3. Execute the Test Suite

Depending on your specific testing requirements, employ the appropriate commands to run the test suite:


- To execute the end-to-end tests:
```bash
go test -v ./test/e2e -timeout 0
```

- To run the integration tests:
```bash
go test -v ./test/integration -timeout 0
```

#### 3. Set Up the Demo Test Environment (Optional)

If necessary, establish the e2e demo test environment by executing the following command:

```bash
make e2e-demo-setup
```

During the setup process, distinct configuration files are generated in the `test/e2e-demo/ibc-config` directory. These files include contract addresses, along with wallets containing mnemonic/private keys. These keys are essential for conducting subsequent tests.

#### 4. Clean Up the Demo Test Environment (Optional)

Upon completion of the testing process, if you've set up the e2e demo environment, you can execute the following command to perform a cleanup:

```bash
make e2e-demo-clean
```
