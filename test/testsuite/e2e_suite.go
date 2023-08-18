package testsuite

import (
	"context"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

func (s *E2ETestSuite) SetupXCall(ctx context.Context, portId string) error {
	chainA, chainB := s.GetChains()
	if err := chainA.SetupXCall(ctx, portId, Owner); err != nil {
		return err
	}
	if err := chainB.SetupXCall(ctx, portId, Owner); err != nil {
		return err
	}
	var err error
	_, err = chainA.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainB.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
	})
	if err != nil {
		return err
	}
	_, err = chainB.ConfigureBaseConnection(context.Background(), chains.XCallConnection{
		KeyName:            Owner,
		CounterpartyNid:    chainA.(ibc.Chain).Config().ChainID,
		ConnectionId:       "connection-0", //TODO
		PortId:             portId,
		CounterPartyPortId: portId,
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
