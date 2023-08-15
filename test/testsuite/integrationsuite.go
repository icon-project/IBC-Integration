package testsuite

import (
	"context"
	"fmt"
	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	test "github.com/strangelove-ventures/interchaintest/v7/testutil"
	"golang.org/x/sync/errgroup"
	"time"
)

var (
	rlyArgs     = []string{"--log-format", "json", "--debug", "--json"}
	channelName = "ICON-ARCHWAY"
)

func (s *E2ETestSuite) CreateClient(ctx context.Context) error {
	eRep := s.GetRelayerExecReporter()
	pathName := s.GetPathName(s.pathNameIndex - 1)
	return s.relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
}

func (s *E2ETestSuite) GetClientState(ctx context.Context, chain chains.Chain, clientSuffix int) (any, error) {
	return chain.GetClientState(ctx, clientSuffix)
}

func (s *E2ETestSuite) GetClientSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetClientsCount(ctx)
}

func (s *E2ETestSuite) GetChannels(ctx context.Context, chainID string) ([]ibc.ChannelOutput, error) {
	eRep := s.GetRelayerExecReporter()
	return s.relayer.GetChannels(ctx, eRep, chainID)
}

func (s *E2ETestSuite) GetChannel(ctx context.Context, chain chains.Chain, channelSuffix int, portID string) (*chantypes.Channel, error) {
	return chain.GetChannel(ctx, channelSuffix, portID)
}

func (s *E2ETestSuite) GetChannelSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetNextChannelSequence(ctx)
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

// GetConnectionState returns the client state for the given chain
func (s *E2ETestSuite) GetNextConnectionSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetNextConnectionSequence(ctx)
}

// Configure

func (s *E2ETestSuite) PacketFlow(ctx context.Context, src, dst chains.Chain, messages ...string) {
	var wg errgroup.Group

	for _, msg := range messages {
		msg := fmt.Sprintf(`{"msg": "%s"}`, msg)
		wg.Go(func() error {
			_, err := src.ExecuteContract(ctx, dst.GetIBCAddress("ibc"), User, "sendPacket", msg)
			if err != nil {
				return fmt.Errorf("failed to execute contract: %s", err)
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

// Task_
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
	// wait for relayer to start.
	chainA, chainB := s.GetChains()
	s.Require().NoError(test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain)), "failed to wait for blocks")
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
