package testsuite

import (
	"context"
	"fmt"
	"strings"
	"time"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

const (
	IconChainName            = "icon"
	CentauriChainName        = "centauri"
	ArchwayChainName         = "archway"
	CentauriIconRelayPath    = "centauri-icon"
	CentauriArchwayRelayPath = "centauri-archway"
)

func (s *E2ETestSuite) SetupXCall(ctx context.Context, portId string, duration int) error {
	createdChains := s.GetChains()
	chainA, chainB := createdChains[0], createdChains[1]
	if err := chainA.SetupXCall(ctx, portId, interchaintest.XCallOwnerAccount); err != nil {
		return err
	}
	if err := chainB.SetupXCall(ctx, portId, interchaintest.XCallOwnerAccount); err != nil {
		return err
	}
	var err error
	_, err = chainA.ConfigureBaseConnection(ctx, chains.XCallConnection{
		KeyName:            interchaintest.XCallOwnerAccount,
		CounterpartyNid:    chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
		TimeoutHeight:      duration,
	})
	if err != nil {
		return err
	}
	_, err = chainB.ConfigureBaseConnection(ctx, chains.XCallConnection{
		KeyName:            interchaintest.XCallOwnerAccount,
		CounterpartyNid:    chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
		TimeoutHeight:      duration,
	})
	if err != nil {
		return err
	}
	err = s.relayer.CreateChannel(ctx, s.GetRelayerExecReporter(), s.GetPathName(s.pathNameIndex-1), ibc.CreateChannelOptions{
		SourcePortName: portId,
		DestPortName:   portId,
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	})
	return err
}

// SetupChainsAndRelayer create two chains, a relayer, establishes a connection and creates a channel
// using the given channel options. The relayer returned by this function has not yet started. It should be started
// with E2ETestSuite.StartRelayer if needed.
// This should be called at the start of every test, unless fine grained control is required.
func (s *E2ETestSuite) SetupChainsAndRelayer(ctx context.Context, channelOpts ...func(*ibc.CreateChannelOptions)) ibc.Relayer {
	relayer, err := s.SetupRelayer(ctx)
	s.Require().NoErrorf(err, "Error while configuring relayer %v", err)
	eRep := s.GetRelayerExecReporter()

	pathName := s.GeneratePathName()
	createdChains := s.GetChains()
	chainA, chainB := createdChains[0], createdChains[1]

	s.Require().NoErrorf(relayer.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName), "Error on generating path, %v", err)
	err = relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{
		TrustingPeriod: "100000m",
	})
	s.Require().NoErrorf(err, "Error while creating client relayer : %s, %v", pathName, err)

	s.Require().NoError(relayer.CreateConnections(ctx, eRep, pathName))
	s.Require().NoError(s.StartRelayer(relayer, pathName))
	return relayer
}

func getIconChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == IconChainName {
			return chain
		}
	}
	return nil
}

func getCentauriChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == CentauriChainName {
			return chain
		}
	}
	return nil
}

func getArchwayChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == ArchwayChainName {
			return chain
		}
	}
	return nil
}
func (s *E2ETestSuite) SetupICS20ChainsAndRelayer(ctx context.Context, channelOpts ...func(*ibc.CreateChannelOptions)) ibc.Relayer {
	relayer, err := s.SetupICS20Relayer(ctx)
	s.Require().NoErrorf(err, "Error while configuring relayer %v", err)
	if !s.cfg.RelayerConfig.UseExistingConfig {
		eRep := s.GetRelayerExecReporter()
		relayChains := s.GetChains()
		iconChain := getIconChain(relayChains)
		archwayChain := getArchwayChain(relayChains)
		centauriChain := getCentauriChain(relayChains)
		s.Require().NoErrorf(relayer.GeneratePath(ctx, eRep, iconChain.(ibc.Chain).Config().ChainID, centauriChain.(ibc.Chain).Config().ChainID, CentauriIconRelayPath), "Error on generating path, %v", err)
		s.Require().NoErrorf(relayer.GeneratePath(ctx, eRep, archwayChain.(ibc.Chain).Config().ChainID, centauriChain.(ibc.Chain).Config().ChainID, CentauriArchwayRelayPath), "Error on generating path, %v", err)
		time.Sleep(4 * time.Second)
		for _, pathName := range []string{CentauriIconRelayPath, CentauriArchwayRelayPath} {
			trustingPeriod := "5040m"
			if strings.Contains(pathName, "archway") {
				trustingPeriod = "500h"
			}
			err = relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{
				TrustingPeriod: trustingPeriod,
			})
			time.Sleep(30 * time.Second)
			s.Require().NoErrorf(err, "Error while creating client relayer : %s, %v", pathName, err)
			s.Require().NoError(relayer.CreateConnections(ctx, eRep, pathName))
			time.Sleep(60 * time.Second)
			s.Require().NoError(relayer.CreateChannel(ctx, eRep, pathName, ibc.CreateChannelOptions{
				SourcePortName: "transfer",
				DestPortName:   "transfer",
			}))
			if err != nil {
				fmt.Println(err)
			}
		}
	}

	s.Require().NoError(s.StartRelayer(relayer, CentauriIconRelayPath))
	return relayer
}
