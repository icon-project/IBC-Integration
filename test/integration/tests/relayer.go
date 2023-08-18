package tests

import (
	"context"
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestClientCreation(ctx context.Context, relayer ibc.Relayer) {
	//portID := "transfer"
	eRep := r.GetRelayerExecReporter()

	r.T.Run("should not able to create a client for a non-existent path", func(t *testing.T) {
		pathName := "random-path"
		err := relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
		expectedErrorMsg := fmt.Sprintf("Error: path with name %s does not exist", pathName)
		assert.Containsf(t, err.Error(), expectedErrorMsg, "Error should be: %v, got: %v", expectedErrorMsg, err)
	})

	r.T.Run("should able to client for an existing pat", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		pathName := r.GeneratePathName()

		var err error

		err = relayer.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName)

		assert.NoErrorf(t, err, "Error on generating path %s, got: %v", pathName, err)

		err = relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})

		assert.NoErrorf(t, err, "Error on creating client %s, got: %v", pathName, err)

		r.T.Run("validate client state", func(t *testing.T) {
			res, err := r.GetClientState(ctx, chainA, 0)
			t.Log(res)
			r.Require().NoError(err)
			res, err = r.GetClientState(ctx, chainB, 0)
			t.Log(res)
			r.Require().NoError(err)
			count, err := r.GetClientSequence(ctx, chainA)
			r.Require().NoError(err)
			r.Require().Equal(1, count)
			count, err = r.GetClientSequence(ctx, chainB)
			r.Require().NoError(err)
			r.Require().Equal(1, count)
		})
	})

}

func (r *RelayerTestSuite) TestConnection(ctx context.Context, relayer ibc.Relayer) {
	chainA, chainB := r.GetChains()
	eRep := r.GetRelayerExecReporter()
	pathName := r.GeneratePathName()
	T := r.T
	err := relayer.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName)
	r.Require().NoErrorf(err, "Error on generating path, %s", err)
	err = relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
	r.Require().NoErrorf(err, "Error on creating client, %s", err)
	//portID := "transfer"
	T.Run("should not able to create a connection for a non-existent path", func(t *testing.T) {
		notExistentPath := "non-existent-path"
		err := relayer.CreateConnections(ctx, eRep, notExistentPath)
		expectedErrorMsg := fmt.Sprintf("Error: path with name %s does not exist", notExistentPath)
		assert.Containsf(t, err.Error(), expectedErrorMsg, "Error should be: %v, got: %v", expectedErrorMsg, err)
	})

	T.Run("should able to create a connection for an existing pat", func(t *testing.T) {
		err := relayer.CreateConnections(ctx, eRep, pathName)
		assert.NoErrorf(t, err, "Error on creating connection %s, got: %v", pathName, err)

		T.Run("validate connection state", func(t *testing.T) {

			stateA, err := r.GetConnectionState(ctx, chainA, 0)
			t.Log(stateA)
			assert.NoErrorf(t, err, "Error on fetching connection state got: %v", err)
			assert.Equal(t, stateA.GetState(), int32(3))
			stateB, err := r.GetConnectionState(ctx, chainB, 0)
			t.Log(stateB)
			assert.NoErrorf(t, err, "Error on fetching connection state got: %v", err)
			assert.Equal(t, stateB.GetState(), int32(3))
			seq, err := r.GetNextConnectionSequence(ctx, chainA)
			assert.NoErrorf(t, err, "Error on fetching next connection sequence got: %v", err)
			assert.Equal(t, 1, seq)
			seq, err = r.GetNextConnectionSequence(ctx, chainB)
			assert.NoErrorf(t, err, "Error on fetching next connection sequence got: %v", err)
			assert.Equal(t, 1, seq)
		})

	})

}

func (r *RelayerTestSuite) TestRelayer(ctx context.Context, relayer ibc.Relayer) {
	chainA, chainB := r.GetChains()
	eRep := r.GetRelayerExecReporter()
	pathName := r.GeneratePathName()
	T := r.T
	err := relayer.GeneratePath(ctx, eRep, chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, pathName)
	r.Require().NoErrorf(err, "Error on generating path, %s", err)
	err = relayer.CreateClients(ctx, eRep, pathName, ibc.CreateClientOptions{TrustingPeriod: "100000m"})
	r.Require().NoErrorf(err, "Error on creating client, %s", err)
	err = relayer.CreateConnections(ctx, eRep, pathName)
	r.Require().NoErrorf(err, "Error on creating connection, %s", err)
	err = r.StartRelayer(relayer, pathName)
	r.Require().NoErrorf(err, "Error on starting relayer, %s", err)

	portID := "transfer"
	err = r.SetupMockDApp(ctx, portID, ibc.Unordered)
	assert.NoErrorf(T, err, "Error on setting up mock dapp, %s", err)

	T.Run("should able to create a channel", func(t *testing.T) {
		err := r.CreateChannel(ctx, pathName, portID, ibc.Unordered)

		assert.NoErrorf(t, err, "Error on creating channel %v", err)

		res, err := r.GetChannel(ctx, chainA, 0, portID)
		assert.NoErrorf(t, err, "Error on getting channel %v", err)
		t.Log(res)
		res, err = r.GetChannel(ctx, chainB, 0, portID)
		assert.NoErrorf(t, err, "Error on getting channel %v", err)
		t.Log(res)

		seq, err := r.GetChannelSequence(ctx, chainA)
		assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
		assert.Equal(t, 1, seq)
		seq, err = r.GetChannelSequence(ctx, chainB)
		assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
		assert.Equal(t, 1, seq)
	})

	T.Run("single relay packet flow", func(t *testing.T) {
		r.Require().NoError(r.Ping(context.Background()))
	})

	T.Run("multi relay packet flow", func(t *testing.T) {
		r.Require().NoError(r.Ping(context.Background()))
	})

	T.Run("unordered packet test", func(t *testing.T) {

	})

	T.Run("crash and recover relay", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		r.Require().NoError(r.CrashTest(context.Background(), chainB, chainA, portID))
		//r.Require().NoError(r.CrashTest(context.Background(), chainB, chainA, portID))
	})

	_ = r.StopRelayer(ctx, relayer)
}

func (r *RelayerTestSuite) CrashTest(ctx context.Context, chainA, chainB chains.Chain, portID string) error {
	// crash relayer and write block height information for crashed node to file
	callbackA := r.WriteBlockHeight(ctx, chainA)
	callbackB := r.WriteBlockHeight(ctx, chainB)
	crashedAt, err := r.Crash(ctx, callbackA, callbackB)
	if err != nil {
		return err
	}
	r.T.Logf("crashed at: %s", crashedAt)
	currentHeight, err := chainB.(ibc.Chain).Height(ctx)
	if err != nil {
		return err
	}
	// send packet from chainA to chainB crashed node and check if it is received
	var msg = chainB.(ibc.Chain).Config().ChainID
	xcall, err := r.SendPacket(ctx, chainA, chainB, msg)
	if err != nil {
		return err
	}
	// recover relayer now
	recoveredAt, err := r.Recover(ctx, chainA.(ibc.Chain), currentHeight)
	if err != nil {
		return err
	}
	r.T.Logf("fully recovered at: %s", recoveredAt)
	// check if packet was sent in a recovered state
	res, err := r.FindPacketSent(xcall, chainA, chainB, currentHeight)
	if err != nil {
		return err
	}
	msg, err = r.ConvertToPlainString(res.Data)
	if err != nil {
		return err
	}
	if res.Data != msg {
		return fmt.Errorf("invalid packet: %s", msg)
	}
	channel, err := r.GetChannel(ctx, chainA, 0, portID)
	if err != nil {
		return err
	}
	if err := r.GetPacketReceipt(xcall, chainB, channel.Counterparty.ChannelId, channel.Counterparty.PortId); err != nil {
		return err
	}
	// check if relay is working as expected with ping pong to cross chain
	if err := r.Ping(ctx); err != nil {
		return err
	}
	r.T.Logf("relay recovered successfully")
	return nil
}
