package testsuite

import (
	"context"
	"fmt"
	"strconv"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains/cosmos"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/icon-project/ibc-integration/test/e2e/relayer"
	"github.com/icon-project/ibc-integration/test/e2e/testconfig"
	test "github.com/strangelove-ventures/interchaintest/v7/testutil"

	"strings"

	"github.com/icon-project/ibc-integration/test/chains"

	dockerclient "github.com/docker/docker/client"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"github.com/stretchr/testify/suite"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

const (
	Owner = "gochain"
	User  = "User"
)

// E2ETestSuite has methods and functionality which can be shared among all test suites.
type E2ETestSuite struct {
	suite.Suite
	relayer ibc.Relayer
	cfg     *testconfig.TestConfig
	//grpcClients    map[string]GRPCClients
	paths          map[string]path
	relayers       relayer.RelayerMap
	logger         *zap.Logger
	DockerClient   *dockerclient.Client
	network        string
	startRelayerFn func(relayer ibc.Relayer) error

	// pathNameIndex is the latest index to be used for generating paths
	pathNameIndex int64
	pathNames     []string
}

func (s *E2ETestSuite) SetCfg() error {
	tc, err := testconfig.New()
	if err != nil {
		return err
	}
	s.cfg = tc
	return nil
}

// path is a pairing of two chains which will be used in a test.
type path struct {
	chainA, chainB chains.Chain
}

// newPath returns a path built from the given chains.
func newPath(chainA, chainB chains.Chain) path {
	return path{
		chainA: chainA,
		chainB: chainB,
	}
}

func (s *E2ETestSuite) SetupXCall(ctx context.Context, portId string) error {
	chainA, chainB := s.GetChains()
	if err := chainA.SetupXCall(ctx, portId, Owner); err != nil {
		return err
	}
	if err := chainB.SetupXCall(ctx, portId, Owner); err != nil {
		return err
	}

	if _, err := chainA.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
	}); err != nil {
		return err
	}
	if _, err := chainB.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
	}); err != nil {
		return err
	}
	return s.relayer.CreateChannel(ctx, s.GetRelayerExecReporter(), s.GetPathName(s.pathNameIndex-1), ibc.CreateChannelOptions{
		SourcePortName: portId,
		DestPortName:   portId,
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	})
}

// SetupChainsAndRelayer create two chains, a relayer, establishes a connection and creates a channel
// using the given channel options. The relayer returned by this function has not yet started. It should be started
// with E2ETestSuite.StartRelayer if needed.
// This should be called at the start of every test, unless fine grained control is required.
func (s *E2ETestSuite) SetupChainsAndRelayer(ctx context.Context, channelOpts ...func(*ibc.CreateChannelOptions)) ibc.Relayer {
	chainA, chainB := s.GetChains()
	r := relayer.New(s.T(), s.cfg.RelayerConfig, s.logger, s.DockerClient, s.network)

	pathName := s.generatePathName()

	channelOptions := ibc.DefaultChannelOpts()
	for _, opt := range channelOpts {
		opt(&channelOptions)
	}

	ic := interchaintest.NewInterchain().
		AddChain(chainA.(ibc.Chain)).
		AddChain(chainB.(ibc.Chain)).
		AddRelayer(r, "r").
		AddLink(interchaintest.InterchainLink{
			Chain1:  chainA.(ibc.Chain),
			Chain2:  chainB.(ibc.Chain),
			Relayer: r,
			Path:    pathName,
		})

	eRep := s.GetRelayerExecReporter()
	buildOptions := interchaintest.InterchainBuildOptions{
		TestName:          s.T().Name(),
		Client:            s.DockerClient,
		NetworkID:         s.network,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
		SkipPathCreation:  true,
	}
	s.Require().NoError(ic.BuildChains(ctx, eRep, buildOptions))
	s.Require().NoError(chainA.BuildWallets(ctx, Owner))
	s.Require().NoError(chainB.BuildWallets(ctx, Owner))

	s.Require().NoError(chainA.BuildWallets(ctx, User))
	s.Require().NoError(chainB.BuildWallets(ctx, User))
	ctx, err := chainA.SetupIBC(ctx, Owner)
	if err != nil {
		panic(err)
	}
	//contracts1 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	ctx, err = chainB.SetupIBC(ctx, Owner)
	if err != nil {
		panic(err)
	}
	//contracts2 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)

	// Start the Relay
	s.Require().NoError(ic.BuildRelayer(ctx, eRep, buildOptions))
	s.Require().NoError(r.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName))
	s.Require().NoError(r.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{
		TrustingPeriod: "100000m",
	}))

	s.Require().NoError(r.CreateConnections(ctx, eRep, pathName))

	s.startRelayerFn = func(relayer ibc.Relayer) error {
		if err := relayer.StartRelayer(ctx, eRep, pathName); err != nil {
			return fmt.Errorf("failed to start relayer: %s", err)
		}
		s.T().Cleanup(func() {
			if !s.T().Failed() {
				if err := relayer.StopRelayer(ctx, eRep); err != nil {
					s.T().Logf("error stopping relayer: %v", err)
				}
			}
		})
		if err := test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain)); err != nil {
			return fmt.Errorf("failed to wait for blocks: %s", err)
		}
		return nil
	}
	s.relayer = r
	return r
}

func (s *E2ETestSuite) DeployMockApp(ctx context.Context, port string) error {
	chainA, chainB := s.GetChains()
	if err := chainA.DeployXCallMockApp(ctx, chains.XCallConnection{
		KeyName:                Owner,
		CounterpartyNid:        chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:           "connection-0", //TODO
		PortId:                 port,
		CounterPartyPortId:     port,
		CounterPartyConnection: chainB.GetIBCAddress("connection"),
	}); err != nil {
		return err
	}
	if err := chainB.DeployXCallMockApp(ctx, chains.XCallConnection{
		KeyName:                Owner,
		CounterpartyNid:        chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:           "connection-0", //TODO
		PortId:                 port,
		CounterPartyPortId:     port,
		CounterPartyConnection: chainA.GetIBCAddress("connection"),
	}); err != nil {
		return err
	}
	return nil
}

// generatePathName generates the path name using the test suites name
func (s *E2ETestSuite) generatePathName() string {
	path := s.GetPathName(s.pathNameIndex)
	s.pathNameIndex++
	return path
}

func (s *E2ETestSuite) getLastPath() string {
	return s.GetPathName(s.pathNameIndex - 1)
}

// GetPathName returns the name of a path at a specific index. This can be used in tests
// when the path name is required.
func (s *E2ETestSuite) GetPathName(idx int64) string {
	pathName := fmt.Sprintf("%s-path-%d", s.T().Name(), idx)
	return strings.ReplaceAll(pathName, "/", "-")
}

// generatePath generates the path name using the test suites name
func (s *E2ETestSuite) generatePath(ctx context.Context, relayer ibc.Relayer) string {
	chainA, chainB := s.GetChains()
	chainAID := chainA.(ibc.Chain).Config().ChainID
	chainBID := chainB.(ibc.Chain).Config().ChainID

	pathName := s.generatePathName()

	err := relayer.GeneratePath(ctx, s.GetRelayerExecReporter(), chainAID, chainBID, pathName)
	s.Require().NoError(err)

	return pathName
}

// UpdateClients updates clients on chainA and chainB
func (s *E2ETestSuite) UpdateClients(ctx context.Context, relayer ibc.Relayer, pathName string) {
	err := relayer.UpdateClients(ctx, s.GetRelayerExecReporter(), pathName)
	s.Require().NoError(err)
}

// GetChains returns two chains that can be used in a test. The pair returned
// is unique to the current test being run. Note: this function does not create containers.
func (s *E2ETestSuite) GetChains(chainOpts ...testconfig.ChainOptionConfiguration) (chains.Chain, chains.Chain) {
	if s.paths == nil {
		s.paths = map[string]path{}
	}

	path, ok := s.paths[s.T().Name()]
	if ok {
		return path.chainA, path.chainB
	}

	chainOptions, err := testconfig.DefaultChainOptions()
	s.Require().NoError(err)
	for _, opt := range chainOpts {
		opt(chainOptions)
	}

	chainA, chainB := s.createChains(chainOptions)
	path = newPath(chainA, chainB)
	s.paths[s.T().Name()] = path
	return path.chainA, path.chainB
}

// GetRelayerWallets returns the relayer wallets associated with the chains.
func (s *E2ETestSuite) GetRelayerWallets(relayer ibc.Relayer) (ibc.Wallet, ibc.Wallet, error) {
	chainA, chainB := s.GetChains()
	chainARelayerWallet, ok := relayer.GetWallet(chainA.(ibc.Chain).Config().ChainID)
	if !ok {
		return nil, nil, fmt.Errorf("unable to find chain A relayer wallet")
	}

	chainBRelayerWallet, ok := relayer.GetWallet(chainB.(ibc.Chain).Config().ChainID)
	if !ok {
		return nil, nil, fmt.Errorf("unable to find chain B relayer wallet")
	}
	return chainARelayerWallet, chainBRelayerWallet, nil
}

// StartRelayer starts the given relayer.
func (s *E2ETestSuite) StartRelayer(relayer ibc.Relayer) error {
	if s.startRelayerFn == nil {
		return fmt.Errorf("cannot start relayer before it is created: %v", relayer)
	}
	return s.startRelayerFn(relayer)
}

// StopRelayer stops the given relayer.
func (s *E2ETestSuite) StopRelayer(ctx context.Context, relayer ibc.Relayer) {
	err := relayer.StopRelayer(ctx, s.GetRelayerExecReporter())
	s.Require().NoError(err)
}

// createChains creates two separate chains in docker containers.
// test and can be retrieved with GetChains.
func (s *E2ETestSuite) createChains(chainOptions *testconfig.ChainOptions) (chains.Chain, chains.Chain) {
	client, network := interchaintest.DockerSetup(s.T())
	t := s.T()

	s.logger = zap.NewExample()
	s.DockerClient = client
	s.network = network

	logger := zaptest.NewLogger(t)

	chainA, _ := buildChain(logger, t.Name(), chainOptions.ChainAConfig)

	chainB, _ := buildChain(logger, t.Name(), chainOptions.ChainBConfig)

	// this is intentionally called after the interchaintest.DockerSetup function. The above function registers a
	// cleanup task which deletes all containers. By registering a cleanup function afterwards, it is executed first
	// this allows us to process the logs before the containers are removed.
	//t.Cleanup(func() {
	//	diagnostics.Collect(t, s.DockerClient, chainOptions)
	//})

	return chainA, chainB
}

func buildChain(log *zap.Logger, testName string, cfg *testconfig.Chain) (chains.Chain, error) {
	var (
		chain chains.Chain
		err   error
	)
	ibcChainConfig := cfg.ChainConfig.GetIBCChainConfig(&chain)
	switch cfg.ChainConfig.Type {
	case "icon":
		chain = icon.NewIconLocalnet(testName, log, ibcChainConfig, chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystoreFile, cfg.KeystorePassword, cfg.Contracts)
		return chain, nil
	case "cosmos", "wasm":
		enc := cosmos.DefaultEncoding()
		ibcChainConfig.EncodingConfig = &enc
		chain, err = cosmos.NewCosmosLocalnet(testName, log, ibcChainConfig, chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystorePassword, cfg.Contracts)
		return chain, err
	default:
		return nil, fmt.Errorf("unexpected error, unknown chain type: %s for chain: %s", cfg.ChainConfig.Type, cfg.Name)
	}
}

// GetRelayerExecReporter returns a testreporter.RelayerExecReporter instances
// using the current test's testing.T.
func (s *E2ETestSuite) GetRelayerExecReporter() *testreporter.RelayerExecReporter {
	rep := testreporter.NewNopReporter()
	return rep.RelayerExecReporter(s.T())
}

func (s *E2ETestSuite) ConvertToPlainString(input string) (string, error) {
	var plainString []byte
	if strings.HasPrefix(input, "[") && strings.HasSuffix(input, "]") {
		input = input[1 : len(input)-1]
		for _, part := range strings.Split(input, ", ") {
			value, err := strconv.Atoi(part)
			if err != nil {
				return "", err
			}
			plainString = append(plainString, byte(value))
		}
		return string(plainString), nil
	} else if strings.HasPrefix(input, "0x") {
		input = input[2:]
		for i := 0; i < len(input); i += 2 {
			value, err := strconv.ParseUint(input[i:i+2], 16, 8)
			if err != nil {
				return "", err
			}
			plainString = append(plainString, byte(value))
		}
	}
	return "", fmt.Errorf("invalid input length")
}
