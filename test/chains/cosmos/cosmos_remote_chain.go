package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"math/big"
	"strings"

	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	cryptocodec "github.com/cosmos/cosmos-sdk/crypto/codec"
	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/internal/dockerutil"
	"github.com/icon-project/ibc-integration/test/testsuite/testconfig"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"go.uber.org/zap"
)

// CosmosRemoteChain is a local docker testnet for a Cosmos SDK chain.
// Implements the ibc.Chain interface.
type CosmosRemoteChain struct {
	testName   string
	cfg        ibc.ChainConfig
	log        *zap.Logger
	Client     *client.Client
	Network    string
	testConfig *testconfig.Chain
}

func NewCosmosHeighlinerChainConfig(name string,
	binary string,
	bech32Prefix string,
	denom string,
	gasPrices string,
	gasAdjustment float64,
	trustingPeriod string,
	noHostMount bool) ibc.ChainConfig {
	return ibc.ChainConfig{
		Type:           "cosmos",
		Name:           name,
		Bech32Prefix:   bech32Prefix,
		Denom:          denom,
		GasPrices:      gasPrices,
		GasAdjustment:  gasAdjustment,
		TrustingPeriod: trustingPeriod,
		NoHostMount:    noHostMount,
		Images: []ibc.DockerImage{
			{
				Repository: fmt.Sprintf("ghcr.io/strangelove-ventures/heighliner/%s", name),
				UidGid:     dockerutil.GetHeighlinerUserString(),
			},
		},
		Bin: binary,
	}
}

func NewCosmosRemoteChain(testName string, chainConfig ibc.ChainConfig, client *client.Client, network string, log *zap.Logger, testConfig *testconfig.Chain) *CosmosRemoteChain {
	if chainConfig.EncodingConfig == nil {
		cfg := DefaultEncoding()
		chainConfig.EncodingConfig = &cfg
	}

	registry := codectypes.NewInterfaceRegistry()
	cryptocodec.RegisterInterfaces(registry)
	return &CosmosRemoteChain{
		testName:   testName,
		cfg:        chainConfig,
		log:        log,
		Client:     client,
		Network:    network,
		testConfig: testConfig,
	}
}

// // Implements Chain interface
func (c *CosmosRemoteChain) Config() ibc.ChainConfig {
	return c.cfg
}

// // Implements Chain interface
func (c *CosmosRemoteChain) Initialize(ctx context.Context, testName string, cli *client.Client, networkID string) error {
	return nil
}

// // Implements Chain interface
func (c *CosmosRemoteChain) GetRPCAddress() string {
	return c.testConfig.RPCUri
}

// // Implements Chain interface
func (c *CosmosRemoteChain) GetGRPCAddress() string {
	return c.testConfig.RPCUri
}

// // GetHostRPCAddress returns the address of the RPC server accessible by the host.
// // This will not return a valid address until the chain has been started.
func (c *CosmosRemoteChain) GetHostRPCAddress() string {
	return c.testConfig.RPCUri
}

// // GetHostGRPCAddress returns the address of the gRPC server accessible by the host.
// // This will not return a valid address until the chain has been started.
func (c *CosmosRemoteChain) GetHostGRPCAddress() string {
	return c.testConfig.RPCUri
}

// // HomeDir implements ibc.Chain.
func (c *CosmosRemoteChain) HomeDir() string {
	panic("not implemented")
}

// // Implements Chain interface
func (c *CosmosRemoteChain) CreateKey(ctx context.Context, keyName string) error {
	panic("not implemented")
}

// Implements Chain interface
func (c *CosmosRemoteChain) RecoverKey(ctx context.Context, keyName, mnemonic string) error {
	panic("not implemented")
}

// // Implements Chain interface
func (c *CosmosRemoteChain) GetAddress(ctx context.Context, keyName string) ([]byte, error) {
	panic("not implemented")
}

// // BuildWallet will return a Cosmos wallet
// // If mnemonic != "", it will restore using that mnemonic
// // If mnemonic == "", it will create a new key
func (c *CosmosRemoteChain) BuildWallet(ctx context.Context, keyName string, mnemonic string) (ibc.Wallet, error) {
	panic("not implemented")
}

// // BuildRelayerWallet will return a Cosmos wallet populated with the mnemonic so that the wallet can
// // be restored in the relayer node using the mnemonic. After it is built, that address is included in
// // genesis with some funds.
func (c *CosmosRemoteChain) BuildRelayerWallet(ctx context.Context, keyName string) (ibc.Wallet, error) {
	panic("not implemented")
}

// // Implements Chain interface
func (c *CosmosRemoteChain) SendFunds(ctx context.Context, keyName string, amount ibc.WalletAmount) error {
	panic("not implemented")
}

// // Implements Chain interface
func (c *CosmosRemoteChain) SendIBCTransfer(
	ctx context.Context,
	channelID string,
	keyName string,
	amount ibc.WalletAmount,
	options ibc.TransferOptions,
) (tx ibc.Tx, _ error) {
	panic("not implemented")
}

// // StoreContract takes a file path to smart contract and stores it on-chain. Returns the contracts code id.
func (c *CosmosRemoteChain) StoreContract(ctx context.Context, keyName string, fileName string) (string, error) {
	panic("not implemented")
}

// // InstantiateContract takes a code id for a smart contract and initialization message and returns the instantiated contract address.
func (c *CosmosRemoteChain) InstantiateContract(ctx context.Context, keyName string, codeID string, initMessage string, needsNoAdminFlag bool, extraExecTxArgs ...string) (string, error) {
	panic("not implemented")
}

// // QueryContract performs a smart query, taking in a query struct and returning a error with the response struct populated.
func (c *CosmosRemoteChain) QueryContract(ctx context.Context, contractAddress string, query any, response any) error {
	panic("not implemented")
}

// // ExportState exports the chain state at specific height.
// // Implements Chain interface
func (c *CosmosRemoteChain) ExportState(ctx context.Context, height int64) (string, error) {
	panic("not implemented")
}

func (c *CosmosRemoteChain) GetGasFeesInNativeDenom(gasPaid int64) int64 {
	panic("not implemented")
}

func (c *CosmosRemoteChain) UpgradeVersion(ctx context.Context, cli *client.Client, containerRepo, version string) {
	panic("not implemented")
}

// // Bootstraps the chain and starts it from genesis
func (c *CosmosRemoteChain) Start(testName string, ctx context.Context, additionalGenesisWallets ...ibc.WalletAmount) error {
	panic("not implemented")
}

// Height implements ibc.Chain
func (c *CosmosRemoteChain) Height(ctx context.Context) (uint64, error) {
	panic("not implemented")
}

// // Acknowledgements implements ibc.Chain, returning all acknowledgments in block at height
func (c *CosmosRemoteChain) Acknowledgements(ctx context.Context, height uint64) ([]ibc.PacketAcknowledgement, error) {
	panic("not implemented")
}

// // Timeouts implements ibc.Chain, returning all timeouts in block at height
func (c *CosmosRemoteChain) Timeouts(ctx context.Context, height uint64) ([]ibc.PacketTimeout, error) {
	panic("not implemented")
}

// // Exec implements ibc.Chain.
func (c *CosmosRemoteChain) Exec(ctx context.Context, cmd []string, env []string) (stdout, stderr []byte, err error) {
	cmd = append([]string{"centaurid"}, cmd...)
	job := dockerutil.NewImage(c.log, c.Client, c.Network, c.testName, c.cfg.Images[0].Repository, c.cfg.Images[0].Version)
	opts := dockerutil.ContainerOptions{
		Binds: []string{
			c.testConfig.ContractsPath + ":/contracts",
			c.testConfig.ConfigPath + ":/centauri/.banksy",
		},
		Env: env,
	}

	res := job.Run(ctx, cmd, opts)
	return res.Stdout, res.Stderr, res.Err
}

func (c *CosmosRemoteChain) extractAmount(text string) (*big.Int, error) {
	entries := strings.Split(text, "\n")
	for _, entry := range entries {
		values := strings.Split(entry, ":")
		key := values[0]
		value := values[1]
		if key == "amount" {
			var balance string
			json.Unmarshal([]byte(value), &balance)
			n := new(big.Int)
			n, ok := n.SetString(balance, 10)
			if !ok {
				return nil, fmt.Errorf("error converting")

			}
			return n, nil

		}
	}
	return nil, fmt.Errorf("balance not found")
}

// // GetBalance fetches the current balance for a specific account address and denom.
// // Implements Chain interface
func (c *CosmosRemoteChain) GetWalletBalance(ctx context.Context, address string, denom string) (*big.Int, error) {
	commands := []string{"query", "bank", "balances", address}
	commands = append(commands,
		"--node", c.GetHostRPCAddress(),
		"--denom", denom,
	)
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return nil, err
	}
	amount, err := c.extractAmount(string(stdout[:]))
	return amount, err
}
