package testsuite

import (
	"context"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

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
	ctx, relayer, err := s.SetupRelayer(ctx)
	s.Require().NoError(err, "Error while configuring relayer")
	eRep := s.GetRelayerExecReporter()
	response := ctx.Value("relayer-response").(map[string]string)
	var pathName = response["pathName"]
	s.Require().NoError(relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{
		TrustingPeriod: "100000m",
	}))

	s.Require().NoError(relayer.CreateConnections(ctx, eRep, pathName))
	return relayer
}
