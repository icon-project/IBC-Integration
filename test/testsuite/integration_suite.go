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
	"go.uber.org/zap"
	"golang.org/x/sync/errgroup"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

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
func (s *E2ETestSuite) FindPacketSent(ctx context.Context, src, target chains.Chain, startHeight uint64) (*chains.XCallResponse, error) {
	res, err := src.FindTargetXCallMessage(ctx, target, startHeight, target.GetIBCAddress("dapp"))
	if err != nil {
		return nil, err
	}
	return res, nil
}

// GetPacketReceipt queries the packet receipt on the target chain
func (s *E2ETestSuite) GetPacketReceipt(ctx context.Context, chain chains.Chain, channelID, portID string) error {
	return chain.GetPacketReceipt(ctx, channelID, portID)
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

func (s *E2ETestSuite) Crash(ctx context.Context, chain ibc.Chain, callbacks ...func() error) (uint64, error) {
	eRep := s.GetRelayerExecReporter()
	s.logger.Info("crashing relayer")
	now := time.Now()
	if err := s.relayer.(interchaintest.Relayer).StopRelayerContainer(ctx, eRep); err != nil {
		return 0, err
	}
	if len(callbacks) > 0 {
		var eg errgroup.Group
		for _, cb := range callbacks {
			eg.Go(cb)
		}
		if err := eg.Wait(); err != nil {
			return 0, err
		}
	}
	s.logger.Info("relayer crashed", zap.Duration("elapsed", time.Since(now)))
	return chain.Height(ctx)
}

// WriteBlockHeight writes the block height to the given file.
func (s *E2ETestSuite) WriteBlockHeight(ctx context.Context, chain chains.Chain) func() error {
	return func() error {
		height, err := chain.(ibc.Chain).Height(ctx)
		if err != nil {
			return err
		}
		chanID := chain.(ibc.Chain).Config().ChainID
		return s.relayer.(interchaintest.Relayer).WriteBlockHeight(ctx, chanID, height-1)
	}
}

// Recover recovers a relay and waits for the relay to catch up to the current height of the stopped chains.
// This is because relay needs to sync with the counterchain network when it was on crashed state.
func (s *E2ETestSuite) Recover(ctx context.Context, waitDuration time.Duration) error {
	s.logger.Info("waiting for relayer to restart")
	now := time.Now()
	if err := s.relayer.(interchaintest.Relayer).RestartRelayerContainer(ctx); err != nil {
		return err
	}
	time.Sleep(waitDuration)
	s.logger.Info("relayer restarted", zap.Duration("elapsed", time.Since(now)))
	return nil
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
