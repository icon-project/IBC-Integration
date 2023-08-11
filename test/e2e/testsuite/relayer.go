// Package testsuite provides a suite of end-to-end tests for the IBC relayer.
// This file contains the implementation of the E2ETestSuite struct and its methods.
// The E2ETestSuite struct provides methods for setting up the relayer, creating clients, connections, and channels,
// and executing packet flows between chains.
// It also provides methods for retrieving client, connection, and channel states and sequences.
// All methods in this file use the relayer package to interact with the relayer and the interchaintest package to build and manage interchain networks.
package testsuite

import (
	"context"
	"fmt"
	"time"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/e2e/relayer"
	"github.com/icon-project/ibc-integration/test/e2e/testconfig"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	test "github.com/strangelove-ventures/interchaintest/v7/testutil"
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

func (s *E2ETestSuite) PacketFlow(ctx context.Context, src, target chains.Chain, msg string) (string, string, error) {
	dst := target.(ibc.Chain).Config().ChainID + "/" + target.GetIBCAddress("dapp")
	_, reqID, data, err := src.XCall(ctx, target, User, dst, []byte(msg), nil)
	if err != nil {
		return "", "", fmt.Errorf("failed to execute contract: %s", err)
	}
	return reqID, data, nil
}

func (s *E2ETestSuite) QueryPacketCommitment(ctx context.Context, targetChain chains.Chain, reqID, data string) (string, error) {
	_, err := targetChain.ExecuteCall(ctx, reqID, data)
	if err != nil {
		return "", fmt.Errorf("failed to execute contract: %s", err)
	}
	return data, nil
}

func (s *E2ETestSuite) ConnectionFailedToEstablish(ctx context.Context) {}

func (s *E2ETestSuite) InvalidPacket(ctx context.Context) {}

func (s *E2ETestSuite) NotResponding(ctx context.Context) {}

// Crash Node
func (s *E2ETestSuite) CrashNode(ctx context.Context, chain chains.Chain) error {
	return chain.PauseNode(ctx)
}

// Resume Node
func (s *E2ETestSuite) ResumeNode(ctx context.Context, chain chains.Chain) error {
	return chain.UnpauseNode(ctx)
}

func (s *E2ETestSuite) Crash(ctx context.Context) (time.Time, error) {
	eRep := s.GetRelayerExecReporter()
	s.logger.Info("crashing relayer")
	return time.Now(), s.relayer.StopRelayer(ctx, eRep)
}

// Recover recover relay
func (s *E2ETestSuite) Recover(ctx context.Context, crashedAt time.Time) (time.Duration, error) {
	eRep := s.GetRelayerExecReporter()
	s.logger.Info("waiting for relayer to restart")
	if err := s.relayer.StartRelayer(ctx, eRep); err != nil {
		return 0, err
	}
	s.logger.Info("relayer restarted")
	// wait for relayer to start.
	chainA, chainB := s.GetChains()
	return time.Since(crashedAt), test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain))
}

// Ping checks if the relayer is running
func (s *E2ETestSuite) Ping(ctx context.Context) (string, error) {
	chainA, chainB := s.GetChains()
	reqID, data, err := s.PacketFlow(ctx, chainA, chainB, "ping")
	if err != nil {
		return "", err
	}
	_, err = s.QueryPacketCommitment(ctx, chainB, reqID, data)
	res, err := s.ConvertToPlainString(data)
	if err != nil {
		return "", err
	}

	reqID, data, err = s.PacketFlow(ctx, chainB, chainA, res)
	if err != nil {
		return "", err
	}
	_, err = s.QueryPacketCommitment(ctx, chainB, reqID, data)
	res, err = s.ConvertToPlainString(data)
	if err != nil {
		return "", err
	}
	if res != "ping" {
		return "", fmt.Errorf("unexpected response: %s", res)
	}
	return "pong", nil
}
