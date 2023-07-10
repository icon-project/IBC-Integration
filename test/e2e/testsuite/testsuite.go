package testsuite

import (
	"context"
	"fmt"

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
	// ChainARelayerName is the name given to the relayer wallet on ChainA
	ChainARelayerName = "rlyA"
	// ChainBRelayerName is the name given to the relayer wallet on ChainB
	ChainBRelayerName = "rlyB"
	// DefaultGasValue is the default gas value used to configure tx.Factory
	DefaultGasValue = 500000
	Owner           = "gochain"
	User            = "User"
)

// E2ETestSuite has methods and functionality which can be shared among all test suites.
type E2ETestSuite struct {
	suite.Suite
	relayer ibc.Relayer
	//grpcClients    map[string]GRPCClients
	paths          map[string]path
	relayers       relayer.RelayerMap
	logger         *zap.Logger
	DockerClient   *dockerclient.Client
	network        string
	startRelayerFn func(relayer ibc.Relayer)

	// pathNameIndex is the latest index to be used for generating paths
	pathNameIndex int64
	pathNames     []string
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

// GetRelayerUsers returns two ibc.Wallet instances which can be used for the relayer users
// on the two chains.
//
//	func (s *E2ETestSuite) GetRelayerUsers(ctx context.Context, chainOpts ...testconfig.ChainOptionConfiguration) (ibc.Wallet, ibc.Wallet) {
//		chainA, chainB := s.GetChains(chainOpts...)
//		chainAAccountBytes, err := chainA.GetAddress(ctx, ChainARelayerName)
//		s.Require().NoError(err)
//
//		chainBAccountBytes, err := chainB.GetAddress(ctx, ChainBRelayerName)
//		s.Require().NoError(err)
//
//		chainARelayerUser := chainB.BuildWallets(ctx context.Context, keyName string) error(ChainARelayerName, chainAAccountBytes, "", chainA.Config())
//		chainBRelayerUser := cosmos.NewWallet(ChainBRelayerName, chainBAccountBytes, "", chainB.Config())
//
//		if s.relayers == nil {
//			s.relayers = make(relayer.RelayerMap)
//		}
//		s.relayers.AddRelayer(s.T().Name(), chainARelayerUser)
//		s.relayers.AddRelayer(s.T().Name(), chainBRelayerUser)
//
//		return chainARelayerUser, chainBRelayerUser
//	}
func (s *E2ETestSuite) SetupXCall(ctx context.Context, portId string) {
	chainA, chainB := s.GetChains()
	var err error
	s.Require().NoError(chainA.SetupXCall(ctx, portId, Owner))
	s.Require().NoError(chainB.SetupXCall(ctx, portId, Owner))

	ctx, err = chainA.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
	})
	s.Require().NoError(err)
	ctx, err = chainB.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
	})
	s.Require().NoError(err)
	err = s.relayer.CreateChannel(ctx, s.GetRelayerExecReporter(), s.GetPathName(s.pathNameIndex-1), ibc.CreateChannelOptions{
		SourcePortName: portId,
		DestPortName:   portId,
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	})
	s.Require().NoError(err)
}

// SetupChainsAndRelayer create two chains, a relayer, establishes a connection and creates a channel
// using the given channel options. The relayer returned by this function has not yet started. It should be started
// with E2ETestSuite.StartRelayer if needed.
// This should be called at the start of every test, unless fine grained control is required.
func (s *E2ETestSuite) SetupChainsAndRelayer(ctx context.Context, channelOpts ...func(*ibc.CreateChannelOptions)) ibc.Relayer {
	config := testconfig.New()
	chainA, chainB := s.GetChains()
	r := relayer.New(s.T(), config.RelayerConfig, s.logger, s.DockerClient, s.network)

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
	chainA.BuildWallets(ctx, Owner)
	chainB.BuildWallets(ctx, Owner)

	chainA.BuildWallets(ctx, User)
	chainB.BuildWallets(ctx, User)
	var err error
	ctx, err = chainA.SetupIBC(ctx, Owner)
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

	s.startRelayerFn = func(relayer ibc.Relayer) {
		err := relayer.StartRelayer(ctx, eRep, pathName)
		s.Require().NoError(err, fmt.Sprintf("failed to start relayer: %s", err))
		s.T().Cleanup(func() {
			if !s.T().Failed() {
				if err := relayer.StopRelayer(ctx, eRep); err != nil {
					s.T().Logf("error stopping relayer: %v", err)
				}
			}
		})
		// wait for relayer to start.
		s.Require().NoError(test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain)), "failed to wait for blocks")
	}
	//nid1 := config.ChainConfigs[0].ChainConfig.ChainID
	//nid2 := config.ChainConfigs[1].ChainConfig.ChainID

	//s.InitGRPCClients(chainA.(ibc.Chain))
	//s.InitGRPCClients(chainB.(ibc.Chain))

	//chainAChannels, err := r.GetChannels(ctx, eRep, chainA.(ibc.Chain).Config().ChainID)
	//s.Require().NoError(err)
	s.relayer = r
	return r
}

func (s *E2ETestSuite) DeployMockApp(ctx context.Context, port string) {
	chainA, chainB := s.GetChains()
	var err error
	err = chainA.DeployXCallMockApp(ctx, chains.XCallConnection{
		KeyName:                Owner,
		CounterpartyNid:        chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:           "connection-0", //TODO
		PortId:                 port,
		CounterPartyPortId:     port,
		CounterPartyConnection: chainB.GetIBCAddress("connection"),
	})
	s.Require().NoError(err)
	err = chainB.DeployXCallMockApp(ctx, chains.XCallConnection{
		KeyName:                Owner,
		CounterpartyNid:        chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:           "connection-0", //TODO
		PortId:                 port,
		CounterPartyPortId:     port,
		CounterPartyConnection: chainA.GetIBCAddress("connection"),
	})
	s.Require().NoError(err)
}

// generatePathName generates the path name using the test suites name
func (s *E2ETestSuite) generatePathName() string {
	path := s.GetPathName(s.pathNameIndex)
	s.pathNameIndex++
	return path
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

// SetupClients creates clients on chainA and chainB using the provided create client options
func (s *E2ETestSuite) SetupClients(ctx context.Context, relayer ibc.Relayer, opts ibc.CreateClientOptions) string {
	pathName := s.generatePath(ctx, relayer)
	err := relayer.CreateClients(ctx, s.GetRelayerExecReporter(), pathName, opts)
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

	chainOptions := testconfig.DefaultChainOptions()
	for _, opt := range chainOpts {
		opt(&chainOptions)
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

// RecoverRelayerWallets adds the corresponding relayer address to the keychain of the chain.
// This is useful if commands executed on the chains expect the relayer information to present in the keychain.
func (s *E2ETestSuite) RecoverRelayerWallets(ctx context.Context, relayer ibc.Relayer) error {
	chainARelayerWallet, chainBRelayerWallet, err := s.GetRelayerWallets(relayer)
	if err != nil {
		return err
	}

	chainA, chainB := s.GetChains()

	if err := chainA.(ibc.Chain).RecoverKey(ctx, ChainARelayerName, chainARelayerWallet.Mnemonic()); err != nil {
		return fmt.Errorf("could not recover relayer wallet on chain A: %s", err)
	}
	if err := chainB.(ibc.Chain).RecoverKey(ctx, ChainBRelayerName, chainBRelayerWallet.Mnemonic()); err != nil {
		return fmt.Errorf("could not recover relayer wallet on chain B: %s", err)
	}
	return nil
}

// StartRelayer starts the given relayer.
func (s *E2ETestSuite) StartRelayer(relayer ibc.Relayer) {
	if s.startRelayerFn == nil {
		panic("cannot start relayer before it is created!")
	}

	s.startRelayerFn(relayer)
}

// StopRelayer stops the given relayer.
func (s *E2ETestSuite) StopRelayer(ctx context.Context, relayer ibc.Relayer) {
	err := relayer.StopRelayer(ctx, s.GetRelayerExecReporter())
	s.Require().NoError(err)
}

// CreateUserOnChainA creates a User with the given amount of funds on chain A.
//func (s *E2ETestSuite) CreateUserOnChainA(ctx context.Context, amount int64) ibc.Wallet {
//	chainA, _ := s.GetChains()
//	return interchaintest.GetAndFundTestUsers(s.T(), ctx, strings.ReplaceAll(s.T().Name(), " ", "-"), amount, chainA.(ibc.Chain))[0]
//}

// CreateUserOnChainB creates a User with the given amount of funds on chain B.
//func (s *E2ETestSuite) CreateUserOnChainB(ctx context.Context, amount int64) ibc.Wallet {
//	_, chainB := s.GetChains()
//	return interchaintest.GetAndFundTestUsers(s.T(), ctx, strings.ReplaceAll(s.T().Name(), " ", "-"), amount, chainB.(ibc.Chain))[0]
//}

// GetChainANativeBalance gets the balance of a given User on chain A.
func (s *E2ETestSuite) GetChainANativeBalance(ctx context.Context, user ibc.Wallet) (int64, error) {
	chainA, _ := s.GetChains()
	return GetNativeChainBalance(ctx, chainA.(ibc.Chain), user)
}

// GetChainBNativeBalance gets the balance of a given User on chain B.
func (s *E2ETestSuite) GetChainBNativeBalance(ctx context.Context, user ibc.Wallet) (int64, error) {
	_, chainB := s.GetChains()
	return GetNativeChainBalance(ctx, chainB.(ibc.Chain), user)
}

// GetChainGRCPClients gets the GRPC clients associated with the given chain.
//func (s *E2ETestSuite) GetChainGRCPClients(chain ibc.Chain) GRPCClients {
//	cs, ok := s.grpcClients[chain.Config().ChainID]
//	s.Require().True(ok, "chain %s does not have GRPC clients", chain.Config().ChainID)
//	return cs
//}

// AssertPacketRelayed asserts that the packet commitment does not exist on the sending chain.
// The packet commitment will be deleted upon a packet acknowledgement or timeout.
//func (s *E2ETestSuite) AssertPacketRelayed(ctx context.Context, chain *cosmos.CosmosChain, portID, channelID string, sequence uint64) {
//	commitment, _ := s.QueryPacketCommitment(ctx, chain, portID, channelID, sequence)
//	s.Require().Empty(commitment)
//}

// createChains creates two separate chains in docker containers.
// test and can be retrieved with GetChains.
func (s *E2ETestSuite) createChains(chainOptions testconfig.ChainOptions) (chains.Chain, chains.Chain) {
	client, network := interchaintest.DockerSetup(s.T())
	t := s.T()

	s.logger = zap.NewExample()
	s.DockerClient = client
	s.network = network

	logger := zaptest.NewLogger(t)

	chainA, _ := buildChain(logger, t.Name(), *chainOptions.ChainAConfig)

	chainB, _ := buildChain(logger, t.Name(), *chainOptions.ChainBConfig)
	//numValidators, numFullNodes := getValidatorsAndFullNodes(0)
	//chainA := cosmos.NewCosmosChain(t.Name(), *chainOptions.ChainAConfig, numValidators, numFullNodes, logger)
	//numValidators, numFullNodes = getValidatorsAndFullNodes(1)
	//chainB := cosmos.NewCosmosChain(t.Name(), *chainOptions.ChainBConfig, numValidators, numFullNodes, logger)

	// this is intentionally called after the interchaintest.DockerSetup function. The above function registers a
	// cleanup task which deletes all containers. By registering a cleanup function afterwards, it is executed first
	// this allows us to process the logs before the containers are removed.
	//t.Cleanup(func() {
	//	diagnostics.Collect(t, s.DockerClient, chainOptions)
	//})

	return chainA, chainB
}

func buildChain(log *zap.Logger, testName string, cfg testconfig.Chain) (chains.Chain, error) {
	switch cfg.ChainConfig.Type {
	case "icon":
		return icon.NewIconLocalnet(testName, log, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystoreFile, cfg.KeystorePassword, cfg.Contracts), nil
	case "cosmos", "archway":
		return cosmos.NewCosmosLocalnet(testName, log, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystorePassword, cfg.Contracts)
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

// TransferChannelOptions configures both of the chains to have non-incentivized transfer channels.
//func (s *E2ETestSuite) TransferChannelOptions() func(options *ibc.CreateChannelOptions) {
//	return func(opts *ibc.CreateChannelOptions) {
//		opts.Version = transfertypes.Version
//		opts.SourcePortName = transfertypes.PortID
//		opts.DestPortName = transfertypes.PortID
//	}
//}

// GetTimeoutHeight returns a timeout height of 1000 blocks above the current block height.
// This function should be used when the timeout is never expected to be reached
//func (s *E2ETestSuite) GetTimeoutHeight(ctx context.Context, chain ibc.Chain) clienttypes.Height {
//	height, err := chain.Height(ctx)
//	s.Require().NoError(err)
//	return clienttypes.NewHeight(clienttypes.ParseChainID(chain.Config().ChainID), uint64(height)+1000)
//}

// GetNativeChainBalance returns the balance of a specific User on a chain using the native denom.
func GetNativeChainBalance(ctx context.Context, chain ibc.Chain, user ibc.Wallet) (int64, error) {
	bal, err := chain.GetBalance(ctx, user.FormattedAddress(), chain.Config().Denom)
	if err != nil {
		return -1, err
	}
	return bal, nil
}

// GetIBCToken returns the denomination of the full token denom sent to the receiving channel
//func GetIBCToken(fullTokenDenom string, portID, channelID string) transfertypes.DenomTrace {
//	return transfertypes.ParseDenomTrace(fmt.Sprintf("%s/%s/%s", portID, channelID, fullTokenDenom))
//}
