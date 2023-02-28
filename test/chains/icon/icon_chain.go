package icon

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"strconv"
	"strings"
	"sync"

	dockertypes "github.com/docker/docker/api/types"
	volumetypes "github.com/docker/docker/api/types/volume"
	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/ibc-integration/test/internal/dockerutil"
	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"github.com/strangelove-ventures/interchaintest/v6/testutil"
	"go.uber.org/zap"
	"golang.org/x/sync/errgroup"
)

type IconChain struct {
	log           *zap.Logger
	testName      string
	cfg           ibc.ChainConfig
	numValidators int
	numFullNodes  int
	FullNodes     IconNodes
	findTxMu      sync.Mutex
}

func NewIconChain(testName string, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int, log *zap.Logger) *IconChain {
	return &IconChain{
		testName:      testName,
		cfg:           chainConfig,
		numValidators: numValidators,
		numFullNodes:  numFullNodes,
		log:           log,
	}
}

// Config fetches the chain configuration.
func (c *IconChain) Config() ibc.ChainConfig {
	return c.cfg
}

// Initialize initializes node structs so that things like initializing keys can be done before starting the chain
func (c *IconChain) Initialize(ctx context.Context, testName string, cli *client.Client, networkID string) error {
	chainCfg := c.Config()
	c.pullImages(ctx, cli)
	image := chainCfg.Images[0]

	newFullNodes := make(IconNodes, c.numFullNodes)
	copy(newFullNodes, c.FullNodes)

	eg, egCtx := errgroup.WithContext(ctx)
	for i := len(c.FullNodes); i < c.numFullNodes; i++ {
		i := i
		eg.Go(func() error {
			fn, err := c.NewChainNode(egCtx, testName, cli, networkID, image, false)
			if err != nil {
				return err
			}
			fn.Index = i
			newFullNodes[i] = fn
			return nil
		})
	}
	if err := eg.Wait(); err != nil {
		return err
	}
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	c.FullNodes = newFullNodes
	return nil
}

func (c *IconChain) pullImages(ctx context.Context, cli *client.Client) {
	for _, image := range c.Config().Images {
		rc, err := cli.ImagePull(
			ctx,
			image.Repository+":"+image.Version,
			dockertypes.ImagePullOptions{},
		)
		if err != nil {
			c.log.Error("Failed to pull image",
				zap.Error(err),
				zap.String("repository", image.Repository),
				zap.String("tag", image.Version),
			)
		} else {
			_, _ = io.Copy(io.Discard, rc)
			_ = rc.Close()
		}
	}
}

func (c *IconChain) NewChainNode(
	ctx context.Context,
	testName string,
	cli *client.Client,
	networkID string,
	image ibc.DockerImage,
	validator bool,
) (*IconNode, error) {
	// Construct the ChainNode first so we can access its name.
	// The ChainNode's VolumeName cannot be set until after we create the volume.
	in := &IconNode{
		log:          c.log,
		Chain:        c,
		DockerClient: cli,
		NetworkID:    networkID,
		TestName:     testName,
		Image:        image,
	}

	v, err := cli.VolumeCreate(ctx, volumetypes.VolumeCreateBody{
		Labels: map[string]string{
			dockerutil.CleanupLabel: testName,

			dockerutil.NodeOwnerLabel: in.Name(),
		},
	})
	if err != nil {
		return nil, fmt.Errorf("creating volume for chain node: %w", err)
	}
	in.VolumeName = v.Name

	if err := dockerutil.SetVolumeOwner(ctx, dockerutil.VolumeOwnerOptions{
		Log: c.log,

		Client: cli,

		VolumeName: v.Name,
		ImageRef:   image.Ref(),
		TestName:   testName,
		UidGid:     image.UidGid,
	}); err != nil {
		return nil, fmt.Errorf("set volume owner: %w", err)
	}
	return in, nil
}

// Start sets up everything needed (validators, gentx, fullnodes, peering, additional accounts) for chain to start from genesis.
func (c *IconChain) Start(testName string, ctx context.Context, additionalGenesisWallets ...ibc.WalletAmount) error {
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	eg, egCtx := errgroup.WithContext(ctx)
	for _, n := range c.FullNodes {
		n := n
		eg.Go(func() error {
			if err := n.CreateNodeContainer(egCtx); err != nil {
				return err
			}
			// All (validators, gentx, fullnodes, peering, additional accounts) are included in the image itself.
			return n.StartContainer(ctx)
		})
	}
	return eg.Wait()
}

// Exec runs an arbitrary command using Chain's docker environment.
// Whether the invoked command is run in a one-off container or execing into an already running container
// is up to the chain implementation.
//
// "env" are environment variables in the format "MY_ENV_VAR=value"
func (c *IconChain) Exec(ctx context.Context, cmd []string, env []string) (stdout []byte, stderr []byte, err error) {
	return c.getFullNode().Exec(ctx, cmd, env)
}

// ExportState exports the chain state at specific height.
func (c *IconChain) ExportState(ctx context.Context, height int64) (string, error) {
	block, err := c.getFullNode().GetBlockByHeight(ctx, height)
	return block, err
}

// GetRPCAddress retrieves the rpc address that can be reached by other containers in the docker network.
func (c *IconChain) GetRPCAddress() string {
	return c.getFullNode().HostRPCPort
}

// GetGRPCAddress retrieves the grpc address that can be reached by other containers in the docker network.
// Not Applicable for Icon
func (c *IconChain) GetGRPCAddress() string {
	panic("not implemented") // TODO: Implement
}

// GetHostRPCAddress returns the rpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
func (c *IconChain) GetHostRPCAddress() string {
	return "http://" + c.getFullNode().HostRPCPort
}

// GetHostGRPCAddress returns the grpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
// Not applicable for Icon
func (c *IconChain) GetHostGRPCAddress() string {
	panic("not implemented") // TODO: Implement
}

// HomeDir is the home directory of a node running in a docker container. Therefore, this maps to
// the container's filesystem (not the host).
func (c *IconChain) HomeDir() string {
	return c.getFullNode().HomeDir()
}

// CreateKey creates a test key in the "user" node (either the first fullnode or the first validator if no fullnodes).
func (c *IconChain) CreateKey(ctx context.Context, password string) error {
	return c.getFullNode().CreateKey(ctx, password)
}

// RecoverKey recovers an existing user from a given mnemonic.
func (c *IconChain) RecoverKey(ctx context.Context, name string, mnemonic string) error {
	panic("not implemented") // TODO: Implement
}

// GetAddress fetches the bech32 address for a test key on the "user" node (either the first fullnode or the first validator if no fullnodes).
func (c *IconChain) GetAddress(ctx context.Context, keyName string) ([]byte, error) {
	addrInByte, err := json.Marshal(keyName)
	if err != nil {
		return nil, err
	}
	return addrInByte, nil
}

// SendFunds sends funds to a wallet from a user account.
func (c *IconChain) SendFunds(ctx context.Context, keyName string, amount ibc.WalletAmount) error {
	panic("not implemented") // TODO: Implement
}

// SendIBCTransfer sends an IBC transfer returning a transaction or an error if the transfer failed.
func (c *IconChain) SendIBCTransfer(ctx context.Context, channelID string, keyName string, amount ibc.WalletAmount, options ibc.TransferOptions) (ibc.Tx, error) {
	panic("not implemented") // TODO: Implement
}

// Height returns the current block height or an error if unable to get current height.
func (c *IconChain) Height(ctx context.Context) (uint64, error) {
	return c.getFullNode().Height(ctx)
}

// GetBalance fetches the current balance for a specific account address and denom.
func (c *IconChain) GetBalance(ctx context.Context, address string, denom string) (int64, error) {
	return c.getFullNode().GetBalance(ctx, address)
}

// GetGasFeesInNativeDenom gets the fees in native denom for an amount of spent gas.
func (c *IconChain) GetGasFeesInNativeDenom(gasPaid int64) int64 {
	gasPrice, _ := strconv.ParseFloat(strings.Replace(c.cfg.GasPrices, c.cfg.Denom, "", 1), 64)
	fees := float64(gasPaid) * gasPrice
	return int64(fees)
}

// Acknowledgements returns all acknowledgements in a block at height.
func (c *IconChain) Acknowledgements(ctx context.Context, height uint64) ([]ibc.PacketAcknowledgement, error) {
	panic("not implemented") // TODO: Implement
}

// Timeouts returns all timeouts in a block at height.
func (c *IconChain) Timeouts(ctx context.Context, height uint64) ([]ibc.PacketTimeout, error) {
	panic("not implemented") // TODO: Implement
}

// BuildRelayerWallet will return a chain-specific wallet populated with the mnemonic so that the wallet can
// be restored in the relayer node using the mnemonic. After it is built, that address is included in
// genesis with some funds.
func (c *IconChain) BuildRelayerWallet(ctx context.Context, keyName string) (ibc.Wallet, error) {
	return c.BuildWallet(ctx, keyName, "")
}

func (c *IconChain) BuildWallet(ctx context.Context, keyName string, mnemonic string) (ibc.Wallet, error) {
	if err := c.CreateKey(ctx, keyName); err != nil {
		return nil, fmt.Errorf("failed to create key with name %q on chain %s: %w", keyName, c.cfg.Name, err)
	}
	addr := c.getFullNode().Address
	addrBytes, err := c.GetAddress(ctx, addr)
	if err != nil {
		return nil, fmt.Errorf("failed to get account address for key %q on chain %s: %w", keyName, c.cfg.Name, err)
	}

	return NewWallet(keyName, addrBytes, mnemonic, c.cfg), nil
}

func (c *IconChain) getFullNode() *IconNode {
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	if len(c.FullNodes) > 0 {
		// use first full node
		return c.FullNodes[0]
	}
	return c.FullNodes[0]
}

func (c *IconChain) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	fn := c.getFullNode()
	return fn.FindTxs(ctx, height)
}

// DeployContract takes a file path to smart contract and initialization message and returns the instantiated contract/SCORE address.
func (c *IconChain) DeployContract(ctx context.Context, scorePath, keystorePath, initMessage string) (string, error) {
	return c.getFullNode().DeployContract(ctx, scorePath, keystorePath, initMessage)
}

func (c *IconChain) QueryContract(ctx context.Context, scoreAddress, methodName, params string) (string, error) {
	return c.getFullNode().QueryContract(ctx, scoreAddress, methodName, params)
}

func (c *IconChain) GetTransactionResult(ctx context.Context, hash string) (icontypes.TransactionResult, error) {
	return c.getFullNode().TransactionResult(ctx, hash)
}

func (c *IconChain) WaitForBlocks(ctx context.Context, numBlocks int) {
	testutil.WaitForBlocks(ctx, numBlocks, c.getFullNode())
}

func (c *IconChain) ExecuteContract(ctx context.Context, scoreAddress, keystorePath, methodName, params string) (string, error) {
	return c.getFullNode().ExecuteContract(ctx, scoreAddress, methodName, keystorePath, params)
}
