package testsuite

import (
	"context"
	"fmt"
	"time"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/e2e/relayer"
	"github.com/icon-project/ibc-integration/test/e2e/testconfig"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	test "github.com/strangelove-ventures/interchaintest/v7/testutil"
	"golang.org/x/sync/errgroup"
)

var (
	rlyArgs     = []string{"--log-format", "json", "--debug", "--json"}
	channelName = "ICON-ARCHWAY"
)

func (s *E2ETestSuite) SetupRelayer(ctx context.Context) (ibc.Relayer, error) {
	config := testconfig.New()
	chainA, chainB := s.GetChains()
	r := relayer.New(s.T(), config.RelayerConfig, s.logger, s.DockerClient, s.network)
	pathName := s.generatePathName()
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
	if err := ic.BuildChains(ctx, eRep, buildOptions); err != nil {
		return nil, err
	}
	if err := chainA.BuildWallets(ctx, Owner); err != nil {
		return nil, err
	}
	if err := chainB.BuildWallets(ctx, Owner); err != nil {
		return nil, err
	}
	if err := chainA.BuildWallets(ctx, User); err != nil {
		return nil, err
	}
	if err := chainB.BuildWallets(ctx, User); err != nil {
		return nil, err
	}
	var err error
	ctx, err = chainA.SetupIBC(ctx, Owner)
	if err != nil {
		return nil, err
	}
	ctx, err = chainB.SetupIBC(ctx, Owner)
	if err != nil {
		return nil, err
	}
	if err := ic.BuildRelayer(ctx, eRep, buildOptions); err != nil {
		return nil, err
	}
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
		s.Require().NoError(test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain)), "failed to wait for blocks")
	}
	s.relayer = r
	return r, r.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName)
}

func (s *E2ETestSuite) CreateClient(ctx context.Context) error {
	eRep := s.GetRelayerExecReporter()
	pathName := s.GetPathName(s.pathNameIndex - 1)
	return s.relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
}

func (s *E2ETestSuite) GetClientState(ctx context.Context, chain chains.Chain, clientSuffix int) (context.Context, error) {
	return chain.GetClientState(ctx, clientSuffix)
}

func (s *E2ETestSuite) GetClientSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetClientsCount(ctx)
}

func (s *E2ETestSuite) CreateConnection(ctx context.Context) error {
	pathName := s.GetPathName(s.pathNameIndex - 1)
	eRep := s.GetRelayerExecReporter()
	return s.relayer.CreateConnections(ctx, eRep, pathName)
}

// GetConnectionState returns the client state for the given chain
func (s *E2ETestSuite) GetConnectionState(ctx context.Context, chain chains.Chain, suffix int) (*conntypes.ConnectionEnd, error) {
	return chain.GetConnectionState(ctx, suffix)
}

func (s *E2ETestSuite) PacketFlow(ctx context.Context, chain chains.Chain, messages ...string) {
	var wg errgroup.Group

	for _, msg := range messages {
		msg := fmt.Sprintf(`{"msg": "%s"}`, msg)
		wg.Go(func() error {
			ctx, err := chain.ExecuteContract(ctx, chain.GetIBCAddress("ibc"), User, "sendPacket", msg)
			if err != nil {
				return fmt.Errorf("failed to execute contract: %s", err)
			}
			if err := test.WaitForBlocks(ctx, 10, chain.(ibc.Chain)); err != nil {
				return fmt.Errorf("failed to wait for blocks: %s", err)
			}
			return nil
		})
	}

	if err := wg.Wait(); err != nil {
		s.Require().NoError(err, "failed to send packets")
	}
}

func (s *E2ETestSuite) PacketNotSentFromIconAndArchway(ctx context.Context) ibc.RelayerExecResult {
	var commands []string
	return s.ExecTxRelay(ctx, commands...)
}

func (s *E2ETestSuite) ConnectionFailedToEstablish(ctx context.Context) ibc.RelayerExecResult {
	var commands []string
	return s.ExecTxRelay(ctx, commands...)
}

func (s *E2ETestSuite) InvalidPacket(ctx context.Context) ibc.RelayerExecResult {
	var commands []string
	return s.ExecTxRelay(ctx, commands...)
}

func (s *E2ETestSuite) NotResponding(ctx context.Context) ibc.RelayerExecResult {
	var commands []string
	return s.ExecTxRelay(ctx, commands...)
}

func (s *E2ETestSuite) CrashAndRecover(ctx context.Context) (time.Duration, error) {
	startTime := time.Now()
	eRep := s.GetRelayerExecReporter()
	s.logger.Info("crashing relayer")
	if err := s.relayer.StopRelayer(ctx, eRep); err != nil {
		return 0, err
	}
	s.logger.Info("waiting for relayer to restart")
	if err := s.relayer.StartRelayer(ctx, eRep); err != nil {
		return 0, err
	}
	s.logger.Info("relayer restarted")
	return time.Since(startTime), nil
}

func (s *E2ETestSuite) ClaimFee(ctx context.Context) ibc.RelayerExecResult {
	var commands []string
	return s.ExecTxRelay(ctx, commands...)
}

func (s *E2ETestSuite) ExecRelay(ctx context.Context, args []string) ibc.RelayerExecResult {
	reporter := s.GetRelayerExecReporter()
	rly := append([]string{"rly"}, args...)
	return s.relayer.Exec(ctx, reporter, append(rly, rlyArgs...), nil)
}

func (s *E2ETestSuite) ExecTxRelay(ctx context.Context, args ...string) ibc.RelayerExecResult {
	tx := []string{"tx", channelName}
	return s.ExecRelay(ctx, append(tx, args...))
}

// ExecQueryRelay exec query relay
func (s *E2ETestSuite) ExecQueryRelay(ctx context.Context, args ...string) ibc.RelayerExecResult {
	query := []string{"query", channelName}
	return s.ExecRelay(ctx, append(query, args...))
}
