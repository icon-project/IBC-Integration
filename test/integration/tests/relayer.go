package tests

import (
	"context"
	"fmt"
	"github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"
	"github.com/stretchr/testify/assert"
	"testing"
	"time"

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
	r.Require().NoErrorf(err, "Error on setting up mock dapp, %s", err)

	r.T.Run("should able to create a channel", func(t *testing.T) {
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

	r.T.Run("single relay packet flow chainA-chainB", func(t *testing.T) {
		response, err := r.SendPacket(ctx, chainA, chainB, "data", 1000)
		assert.NoErrorf(t, err, "Error while sending package from chainA-chainB")
		assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
		assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
	})
	r.T.Run("single relay packet flow chainB-chainA", func(t *testing.T) {
		response, err := r.SendPacket(ctx, chainB, chainA, "data", 1000)
		assert.NoErrorf(t, err, "Error while sending package from chainB-chainA")
		assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
		assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
	})
	//
	//r.T.Run("multi relay packet flow", func(t *testing.T) {
	//	//r.Require().NoError(r.Ping(ctx))
	//})
	//
	r.T.Run("unordered packet test chainA-chainB", func(t *testing.T) {
		r.UnOrderPacketTest(ctx, chainA, chainB)
	})

	r.T.Run("unordered packet test chainB-chainA", func(t *testing.T) {
		r.UnOrderPacketTest(ctx, chainB, chainA)
	})

	r.T.Run("crash and recover relay chainA-chainB", func(t *testing.T) {
		r.RelayerCrashTest(ctx, chainA, chainB)
	})

	r.T.Run("crash and recover relay chainB-chainA", func(t *testing.T) {
		r.RelayerCrashTest(ctx, chainB, chainA)
	})

	//defer func(r *RelayerTestSuite, ctx context.Context, relayer ibc.Relayer) {
	//	err := r.StopRelayer(ctx, relayer)
	//	if err != nil {
	//		fmt.Println(err)
	//	}
	//}(r, ctx, relayer)
}

func (r *RelayerTestSuite) UnOrderPacketTest(ctx context.Context, chainA, chainB chains.Chain) {
	// crash relayer and write block height information for crashed node to file
	ctx = r.crashAndSendPacket(ctx, chainA, chainB)

	cResponse := ctx.Value("crash-response").(map[string]interface{})
	packet := cResponse["packet"].(types.Packet)
	crashHeight := cResponse["height"].(uint64)
	height, err := chainA.(ibc.Chain).Height(ctx)
	// recover relayer now
	err = r.WriteBlockHeight(ctx, chainA.(ibc.Chain).Config().ChainID, height+1)
	assert.NoErrorf(r.T, err, "Error on while setting block height (%d) %v ", height+1, err)

	// recover relayer now
	isPacketReceived := r.recoverAndCheckPacket(ctx, chainB, packet)

	assert.Falsef(r.T, isPacketReceived, "The packet event has received on the target chain.")

	// check if relay is working as expected with ping pong to cross chain
	msg := "new-message"
	response, err := r.SendPacket(ctx, chainA, chainB, msg, 1000)
	assert.NoErrorf(r.T, err, "Error on sending packet (%s) %v", msg, err)
	assert.Truef(r.T, response.IsPacketSent, "The packet has not been sent to the target chain.")
	assert.Truef(r.T, response.IsPacketReceiptEventFound, "The packet event has NOT been received on the target chain.")

	ctx = r.crashAndSendPacket(ctx, chainA, chainB)

	err = r.WriteBlockHeight(ctx, chainA.(ibc.Chain).Config().ChainID, crashHeight-1)
	assert.NoErrorf(r.T, err, "Error on while setting block height (%d) %v ", crashHeight-1, err)

	isPacketReceived = r.recoverAndCheckPacket(ctx, chainB, packet)

	assert.Truef(r.T, isPacketReceived, "The packet event has NOT received on the target chain.")

	r.T.Logf("relay recovered successfully")
}

func (r *RelayerTestSuite) RelayerCrashTest(ctx context.Context, chainA, chainB chains.Chain) {
	// crash relayer and write block height information for crashed node to file
	ctx = r.crashAndSendPacket(ctx, chainA, chainB)

	cResponse := ctx.Value("crash-response").(map[string]interface{})
	packet := cResponse["packet"].(types.Packet)
	// recover relayer now
	isPacketReceived := r.recoverAndCheckPacket(ctx, chainB, packet)
	assert.Truef(r.T, isPacketReceived, "The packet event has NOT received on the target chain.")

	// check if relay is working as expected with ping pong to cross chain
	msg := "new-message"
	response, err := r.SendPacket(ctx, chainA, chainB, msg, 1000)
	assert.NoErrorf(r.T, err, "Error on sending packet (%s) %v", msg, err)
	assert.Truef(r.T, response.IsPacketSent, "The packet has not been sent to the target chain.")
	assert.Truef(r.T, response.IsPacketReceiptEventFound, "The packet event has NOT been received on the target chain.")

	r.T.Logf("relay recovered successfully")
}

func (r *RelayerTestSuite) recoverAndCheckPacket(ctx context.Context, targetChain chains.Chain, packet types.Packet) bool {
	err := r.Recover(ctx, time.Second*30)
	assert.NoErrorf(r.T, err, "Error on relayer recover %v", err)

	params := map[string]interface{}{
		"sequence":   packet.Sequence,
		"port_id":    packet.SourcePort,
		"channel_id": packet.SourceChannel,
	}
	isPacketReceived := findPacket(ctx, targetChain, params)
	return isPacketReceived
}

func (r *RelayerTestSuite) crashAndSendPacket(ctx context.Context, chainA chains.Chain, chainB chains.Chain) context.Context {
	callbackA := r.WriteCurrentBlockHeight(ctx, chainA)
	callbackB := r.WriteCurrentBlockHeight(ctx, chainB)
	crashedHeight, err := r.CrashRelayer(ctx, chainB.(ibc.Chain), callbackA, callbackB)
	assert.NoErrorf(r.T, err, "Error on relayer crash %v", err)

	r.T.Logf("crashed at: %d", crashedHeight)
	// send packet from chainA to chainB crashed node and check if it is received
	var msg = fmt.Sprintf("data-%s", chainB.(ibc.Chain).Config().ChainID)
	response, _ := r.SendPacket(ctx, chainA, chainB, msg, 1000000)
	packet := response.Packet
	assert.NotNilf(r.T, packet, "packet is null")
	assert.Truef(r.T, response.IsPacketSent, "The packet has not been sent to the target chain.")
	assert.Falsef(r.T, response.IsPacketReceiptEventFound, "The packet event has already been received on the target chain.")
	return context.WithValue(ctx, "crash-response", map[string]interface{}{
		"packet":  packet,
		"height":  crashedHeight,
		"message": msg,
	})
}

func findPacket(ctx context.Context, chain chains.Chain, params map[string]interface{}) bool {
	duration := 30 * time.Second
	interval := 2 * time.Second

	_ctx, cancel := context.WithTimeout(context.Background(), duration)
	defer cancel()

	var isPacketReceived bool

	for {
		select {
		case <-_ctx.Done():
			fmt.Println("Loop finished")
			return isPacketReceived
		default:
			isPacketReceived = chain.IsPacketReceived(ctx, params)
			if isPacketReceived {
				return isPacketReceived
			}
			time.Sleep(interval)
		}
	}
}
