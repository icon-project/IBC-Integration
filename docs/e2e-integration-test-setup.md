# End-to-End Testing Setup and Demo

## Prerequisites

To run the demo, the following software needs to be installed.

* Docker compose \[[download](https://docs.docker.com/compose/install/)\]

## Setting up the Environment

1. Create an `ibc-e2e-tests` folder and clone the `IBC-Integration` repository:

    ```bash
    mkdir ibc-e2e-tests
    cd ibc-e2e-tests
    git clone https://github.com/icon-project/IBC-Integration.git
    ```

2. Build the `ibc-relayer` image:

   ```bash
   git clone https://github.com/icon-project/ibc-relay.git
   cd ibc-relay
   docker build -t relayer .
   cd -  # Back to the root folder
   ```

### Additional Images Required for Apple Silicon 

If you are using an Apple Silicon machine, follow these additional steps to build images:

* Build an `icon-chain` image

   ```bash
    git clone https://github.com/icon-project/goloop.git 
    cd goloop
    make gochain-icon-image
    cd -  # Back to the root folder
   ``` 

* Build a `goloop` image

   ```bash
    git clone https://github.com/icon-project/goloop/
    cd goloop/ 
    make goloop-icon-image
    cd -  # Back to the root folder
   ```

* Build an `archway` or `neutron` image

  **For Archway:**

   ```bash
   git clone https://github.com/archway-network/archway/
   cd archway
   docker build -f Dockerfile.deprecated -t archway . --build-arg arch=aarch64
   cd -  # Back to the root folder
   ```

  **For Neutron:**

   ```bash
   git clone https://github.com/neutron-org/neutron.git
   cd neutron
   make build-docker-image
   cd -  # Back to the root folder
   ```

## Running IBC Integration System Tests

To conduct tests for the IBC integration system, follow these steps:

#### 1. Configure Environment Variables

Before initiating the tests, configure essential environment variables:

- **`E2E_CONFIG_PATH`**: Set this variable to the absolute path of your chosen configuration file. You can create these configuration files using the sample files provided in the `IBC-Integration` source folder. Sample configuration files are available at the following locations:
    - For Archway, use: `IBC-Integration/test/testsuite/sample-config-archway.yaml`
    - For Neutron, use: `IBC-Integration/test/testsuite/sample-config-neutron.yaml`
- **`GOLOOP_IMAGE_ENV`**: Specify the name of the Goloop image.
- **`GOLOOP_IMAGE_TAG_ENV`**: Specify the version of the Goloop image.

Here's an example of environment variable configuration:

```bash
export E2E_CONFIG_PATH=/path/to/config.yaml
export GOLOOP_IMAGE_ENV=goloop-icon
export GOLOOP_IMAGE_TAG_ENV=latest
```

ℹ️ Please note that most of the config content can be used same as it in sample config however you may need to update the image name and version for Archway, Neutron, and Icon in the configuration file you create.


After configuring these variables, navigate to the `IBC-Integration` source folder:

```bash
cd IBC-Integration
```

#### 2. Run the Test Script

Use the appropriate command to run the test suite. Depending on your specific testing requirements, you can use the following command:

```bash
./scripts/execute-test.sh [options]
```

Replace `[options]` with any command-line options or arguments that the test script supports. Here's an option block to help you:

```markdown
Options:
 --clean: Clean contract directories (true/false, default: false).
 --build-ibc: Build IBC contracts (true/false, default: false).
 --build-xcall: Build xCall contracts (true/false, default: false).
 --xcall-branch <branch>: Specify the xCall branch to build (default: main).
 --use-docker: Use Docker for building contracts(true/false, default: false).
 --test <test_type>: Specify the type of test (e2e, e2e-demo, integration, default: e2e).
```

To perform an end-to-end (e2e) test with all the necessary builds, execute the following command:
```bash
./scripts/execute-test.sh --build-ibc --build-xcall --use-docker --test e2e
```
This command covers building IBC and xCall contracts while utilizing Docker and running an end-to-end test.

Once you've initially built the contracts using the command above, you can easily execute the e2e test by using the following simplified command:
```bash
./scripts/execute-test.sh  --test e2e
```

### Set Up the Demo Test Environment (Optional)

If necessary, establish the e2e demo test environment by executing the following command:

```bash
make e2e-demo-setup
```

During the setup process, distinct configuration files are generated in the `IBC-Integration/test/e2e-demo/ibc-config` directory. These files include contract addresses, along with wallets containing mnemonic/private keys. These keys are essential for conducting subsequent tests.

#### Clean Up the Demo Test Environment (Optional)

Upon completion of the testing process, if you've set up the e2e demo environment, you can execute the following command to perform a cleanup:

```bash
make e2e-demo-clean
```

### Other commands available inside IBC-Integration repo

1. Build the builder image for bundling contracts:

   ```bash
   make build-builder-img
   ```

2. Bundle and optimize IBC core contracts:

   ```bash
   make optimize-build
   ``` 

3. Bundle and optimize xcall-multi contracts:

    ```bash
   make optimize-xcall
   ```