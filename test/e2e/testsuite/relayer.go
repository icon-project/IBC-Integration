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
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	test "github.com/strangelove-ventures/interchaintest/v7/testutil"
)

// SetupRelayer sets up the relayer, creates interchain networks, builds chains, and starts the relayer.
// It returns a Relayer interface and an error if any.
func (s *E2ETestSuite) SetupRelayer(ctx context.Context) (ibc.Relayer, error) {
	chainA, chainB := s.GetChains()
	r := relayer.New(s.T(), s.cfg.RelayerConfig, s.logger, s.DockerClient, s.network)
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
	if _, err := chainA.SetupIBC(ctx, Owner); err != nil {
		return nil, err
	}
	if _, err = chainB.SetupIBC(ctx, Owner); err != nil {
		return nil, err
	}
	if err := ic.BuildRelayer(ctx, eRep, buildOptions); err != nil {
		return nil, err
	}
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
			return fmt.Errorf("failed to wait for blocks: %v", err)
		}
		return nil
	}
	s.relayer = r
	return r, r.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName)
}

// CreateClient creates a client on the interchain network.
func (s *E2ETestSuite) CreateClient(ctx context.Context) error {
	eRep := s.GetRelayerExecReporter()
	pathName := s.GetPathName(s.pathNameIndex - 1)
	return s.relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
}

// GetClientState returns the client state for the given chain.
func (s *E2ETestSuite) GetClientState(ctx context.Context, chain chains.Chain, clientSuffix int) (any, error) {
	return chain.GetClientState(ctx, clientSuffix)
}

// GetClientSequence returns the client sequence for the given chain.
func (s *E2ETestSuite) GetClientSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetClientsCount(ctx)
}

// CreateChannel creates a channel on the interchain network.
func (s *E2ETestSuite) CreateChannel(ctx context.Context, portID string) error {
	eRep := s.GetRelayerExecReporter()
	pathName := s.GetPathName(s.pathNameIndex - 1)
	channelOptions := ibc.CreateChannelOptions{
		SourcePortName: portID,
		DestPortName:   portID,
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}
	return s.relayer.CreateChannel(ctx, eRep, pathName, channelOptions)
}

// GetChannels returns all channels for the given chain ID.
func (s *E2ETestSuite) GetChannels(ctx context.Context, chainID string) ([]ibc.ChannelOutput, error) {
	eRep := s.GetRelayerExecReporter()
	return s.relayer.GetChannels(ctx, eRep, chainID)
}

// GetChannel returns the channel for the given chain and channel suffix.
func (s *E2ETestSuite) GetChannel(ctx context.Context, chain chains.Chain, channelSuffix int, portID string) (*chantypes.Channel, error) {
	return chain.GetChannel(ctx, channelSuffix, portID)
}

// GetChannelSequence returns the channel sequence for the given chain.
func (s *E2ETestSuite) GetChannelSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetNextChannelSequence(ctx)
}

// CreateConnection creates a connection on the interchain network.
func (s *E2ETestSuite) CreateConnection(ctx context.Context) error {
	pathName := s.GetPathName(s.pathNameIndex - 1)
	eRep := s.GetRelayerExecReporter()
	return s.relayer.CreateConnections(ctx, eRep, pathName)
}

// GetConnectionState returns the connection state for the given chain.
func (s *E2ETestSuite) GetConnectionState(ctx context.Context, chain chains.Chain, suffix int) (*conntypes.ConnectionEnd, error) {
	return chain.GetConnectionState(ctx, suffix)
}

// GetNextConnectionSequence returns the next connection sequence for the given chain.
func (s *E2ETestSuite) GetNextConnectionSequence(ctx context.Context, chain chains.Chain) (int, error) {
	return chain.GetNextConnectionSequence(ctx)
}

// Configure
func (s *E2ETestSuite) PacketFlow(ctx context.Context, src, target chains.Chain, msg string) (*chains.XCallResponse, error) {
	dst := target.(ibc.Chain).Config().ChainID + "/" + target.GetIBCAddress("dapp")
	res, err := src.XCall(ctx, target, User, dst, []byte(msg), nil)
	if err != nil {
		return nil, fmt.Errorf("failed to execute contract: %s", err)
	}
	return res, nil
}

// SendPacket sends a packet from src to dst
func (s *E2ETestSuite) SendPacket(ctx context.Context, src, target chains.Chain, msg string) (context.Context, error) {
	// Send packet
	dst := target.(ibc.Chain).Config().ChainID + "/" + target.GetIBCAddress("dapp")
	ctx, err := src.SendPacketXCall(ctx, User, dst, []byte(msg), nil)
	if err != nil {
		return nil, err
	}
	return ctx, nil
}

// QueryPacketCommitmentTarget queries the packet commitment on the target chain
func (s *E2ETestSuite) FindPacketSent(ctx context.Context, src, target chains.Chain, startHeight int64) (*chains.XCallResponse, error) {
	res, err := src.FindTargetXCallMessage(ctx, target, startHeight, target.GetIBCAddress("dapp"))
	if err != nil {
		return nil, err
	}
	return res, nil
}

// GetPacketReceipt queries the packet receipt on the target chain
func (s *E2ETestSuite) GetPacketReceipt(ctx context.Context, chain chains.Chain, channelID, portID string) (*chains.XCallResponse, error) {
	res, err := chain.GetPacketReceipt(ctx, channelID, portID)
	if err != nil {
		return nil, err
	}
	return res, nil
}

func (s *E2ETestSuite) QueryPacketCommitment(ctx context.Context, targetChain chains.Chain, reqID, data string) error {
	_, err := targetChain.ExecuteCall(ctx, reqID, data)
	return err
}

// Crash Node
func (s *E2ETestSuite) CrashNode(ctx context.Context, chain chains.Chain) error {
	return chain.PauseNode(ctx)
}

// Resume Node
func (s *E2ETestSuite) ResumeNode(ctx context.Context, chain chains.Chain) error {
	return chain.UnpauseNode(ctx)
}

func (s *E2ETestSuite) Crash(ctx context.Context, chainID string, height uint64) (time.Time, error) {
	eRep := s.GetRelayerExecReporter()
	s.logger.Info("crashing relayer")
	return time.Now(), s.relayer.(interchaintest.Relayer).StopRelayerContainer(ctx, eRep, chainID, height)
}

// Recover recover relay
func (s *E2ETestSuite) Recover(ctx context.Context, crashedAt time.Time) (time.Duration, error) {
	s.logger.Info("waiting for relayer to restart")
	if err := s.relayer.(interchaintest.Relayer).RestartRelayerContainer(ctx); err != nil {
		return 0, err
	}
	s.logger.Info("relayer restarted")
	// wait for relayer to start.
	chainA, chainB := s.GetChains()
	return time.Since(crashedAt), test.WaitForBlocks(ctx, 10, chainA.(ibc.Chain), chainB.(ibc.Chain))
}

// Ping checks if the relayer is running
func (s *E2ETestSuite) Ping(ctx context.Context) error {
	chainA, chainB := s.GetChains()
	var msg = "ping"
	res, err := s.PacketFlow(ctx, chainA, chainB, msg)
	if err != nil {
		return err
	}
	if err := s.QueryPacketCommitment(ctx, chainB, res.RequestID, res.Data); err != nil {
		return err
	}
	data, err := s.ConvertToPlainString(res.Data)
	if err != nil {
		return err
	}
	if data != msg {
		return fmt.Errorf("failed to ping from %s to %s", chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID)
	}
	res, err = s.PacketFlow(ctx, chainB, chainA, data)
	if err != nil {
		return err
	}
	if err := s.QueryPacketCommitment(ctx, chainA, res.RequestID, res.Data); err != nil {
		return err
	}
	data, err = s.ConvertToPlainString(res.Data)
	if err != nil {
		return err
	}
	if data != msg {
		return fmt.Errorf("failed to ping from %s to %s", chainB.(ibc.Chain).Config().ChainID, chainA.(ibc.Chain).Config().ChainID)
	}
	return nil
}
