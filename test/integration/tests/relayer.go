package tests

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"
	"github.com/stretchr/testify/assert"

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

	r.T.Run("Un-order packet flow", func(t *testing.T) {
		ctx = context.WithValue(ctx, "testcase", "unordered")
		portID := "transfer-un-ordered"
		err = r.SetupMockDApp(ctx, portID)
		assert.NoErrorf(t, err, "Error on setting up mock dapp, %v", err)

		r.T.Run("should able to create Unordered channel", func(t *testing.T) {
			seqA, _ := r.GetChannelSequence(ctx, chainA)
			seqB, _ := r.GetChannelSequence(ctx, chainB)
			err := r.CreateChannel(ctx, pathName, portID, ibc.Unordered)

			assert.NoErrorf(t, err, "Error on creating channel %v", err)

			res, err := r.GetChannel(ctx, chainA, seqA, portID)
			assert.NoErrorf(t, err, "Error on getting channel %v", err)
			t.Log(res)
			res, err = r.GetChannel(ctx, chainB, seqB, portID)
			assert.NoErrorf(t, err, "Error on getting channel %v", err)
			t.Log(res)

			seq, err := r.GetChannelSequence(ctx, chainA)
			assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
			assert.Equal(t, seqA+1, seq)
			seq, err = r.GetChannelSequence(ctx, chainB)
			assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
			assert.Equal(t, seqB+1, seq)
		})

		r.T.Run("single relay packet flow chainA-chainB", func(t *testing.T) {
			response, err := r.SendPacket(ctx, chainA, chainB, "data", 1000, false)
			assert.NoErrorf(t, err, "Error while sending package from chainA-chainB")
			assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
			assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
		})
		r.T.Run("single relay packet flow chainB-chainA", func(t *testing.T) {
			response, err := r.SendPacket(ctx, chainB, chainA, "data", 1000, false)
			assert.NoErrorf(t, err, "Error while sending package from chainB-chainA")
			assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
			assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
		})

		r.T.Run("crash and recover relay chainA-chainB", func(t *testing.T) {
			r.RelayerCrashTest(ctx, chainA, chainB)
		})

		r.T.Run("crash and recover relay chainB-chainA", func(t *testing.T) {
			r.RelayerCrashTest(ctx, chainB, chainA)
		})
		r.T.Run("unordered packet test chainA-chainB", func(t *testing.T) {
			r.PacketFlowTest(ctx, t, chainA, chainB, ibc.Unordered)
		})

		r.T.Run("unordered packet test chainB-chainA", func(t *testing.T) {
			r.PacketFlowTest(ctx, t, chainB, chainA, ibc.Unordered)
		})
	})

	r.T.Run("Order packet flow", func(t *testing.T) {
		ctx = context.WithValue(ctx, "testcase", "ordered")
		portID := "transfer-ordered"
		//reset latest height of each chain on relayer
		callbackA := func() error {
			return r.WriteBlockHeight(ctx, chainA.(ibc.Chain).Config().ChainID, 0)
		}
		callbackB := func() error {
			return r.WriteBlockHeight(ctx, chainB.(ibc.Chain).Config().ChainID, 0)
		}
		err := r.CrashRelayer(ctx, callbackA, callbackB)
		if err != nil {
			return
		}

		err = r.Recover(ctx, time.Second*40)

		err = r.SetupMockDApp(ctx, portID)
		assert.NoErrorf(t, err, "Error on setting up mock dapp, %v", err)

		r.T.Run("should able to create Ordered channel", func(t *testing.T) {
			seqA, _ := r.GetChannelSequence(ctx, chainA)
			seqB, _ := r.GetChannelSequence(ctx, chainB)
			err := r.CreateChannel(ctx, pathName, portID, ibc.Ordered)

			assert.NoErrorf(t, err, "Error on creating channel %v", err)

			res, err := r.GetChannel(ctx, chainA, seqA, portID)
			assert.NoErrorf(t, err, "Error on getting channel %v", err)
			t.Log(res)
			res, err = r.GetChannel(ctx, chainB, seqB, portID)
			assert.NoErrorf(t, err, "Error on getting channel %v", err)
			t.Log(res)

			seq, err := r.GetChannelSequence(ctx, chainA)
			assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
			assert.Equal(t, seqA+1, seq)
			seq, err = r.GetChannelSequence(ctx, chainB)
			assert.NoErrorf(t, err, "Error on getting next channel sequence %v", err)
			assert.Equal(t, seqB+1, seq)
		})

		r.T.Run("single relay packet flow chainA-chainB", func(t *testing.T) {
			response, err := r.SendPacket(ctx, chainA, chainB, "data", 1000, false)
			assert.NoErrorf(t, err, "Error while sending package from chainA-chainB")
			assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
			assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
		})

		r.T.Run("single relay packet flow chainB-chainA", func(t *testing.T) {
			response, err := r.SendPacket(ctx, chainB, chainA, "data", 1000, false)
			assert.NoErrorf(t, err, "Error while sending package from chainB-chainA")
			assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")
			assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has not received on the target chain.")
		})

		r.T.Run("ordered packet test chainA-chainB", func(t *testing.T) {
			r.PacketFlowTest(ctx, t, chainA, chainB, ibc.Ordered)
		})

		r.T.Run("ordered packet test chainB-chainA", func(t *testing.T) {
			r.PacketFlowTest(ctx, t, chainB, chainA, ibc.Ordered)
		})
	})

	r.T.Run("send multiple packets on same ChainA height", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		height, err := chainA.(ibc.Chain).Height(ctx)
		r.Require().NoError(err)
		r.Require().NoError(r.multiplePacketsOnSameHeight(chainA, chainB, height+100, 5))
	})

	r.T.Run("send multiple packets on same ChainB height", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		height, err := chainB.(ibc.Chain).Height(ctx)
		r.Require().NoError(err)
		r.Require().NoError(r.multiplePacketsOnSameHeight(chainB, chainA, height+100, 5))
	})
}

func (r *RelayerTestSuite) PacketFlowTest(ctx context.Context, t *testing.T, src, target chains.Chain, order ibc.Order) {
	packet, crashHeight := r.handleCrashAndSendPacket(ctx, src, target)

	height, err := src.(ibc.Chain).Height(ctx)
	assert.NoErrorf(t, err, "Error while getting block height: %v", err)

	err = r.WriteBlockHeight(ctx, src.(ibc.Chain).Config().ChainID, height+1)
	assert.NoErrorf(t, err, "Error on setting block height (%d): %v", height+1, err)

	err = r.Recover(ctx, time.Second*30)
	r.Require().NoErrorf(err, "Unable to recover relayer %v", err)
	isPacketReceived := r.checkPacketReceipt(ctx, target, packet, order)
	assert.Falsef(t, isPacketReceived, "The packet event has received on the target chain.\n%v\n", packet)

	msg := "new-message"
	response, err := r.SendPacket(ctx, src, target, msg, 1000, false) //new packet
	if order == ibc.Ordered {
		assert.Errorf(t, err, "Error on sending packet (%s): %v", msg, err)
		assert.Falsef(t, response.IsPacketReceiptEventFound, "The packet event has been received on the target chain.")
	} else {
		assert.NoErrorf(t, err, "Error on sending packet (%s) %v", msg, err)
		assert.Truef(t, response.IsPacketReceiptEventFound, "The packet event has NOT been received on the target chain.")
	}
	assert.Truef(t, response.IsPacketSent, "The packet has not been sent to the target chain.")

	packetNew, _ := r.handleCrashAndSendPacket(ctx, src, target)

	err = r.WriteBlockHeight(ctx, src.(ibc.Chain).Config().ChainID, crashHeight-1)
	assert.NoErrorf(t, err, "Error on setting block height (%d): %v", crashHeight-1, err)

	err = r.Recover(ctx, time.Second*30)
	assert.NoErrorf(t, err, "Unable to recover relayer %v", err)
	isPacketReceived = r.checkPacketReceipt(ctx, target, packet, order)
	assert.Truef(t, isPacketReceived, "The packet event has NOT received on the target chain.\n%v\n", packet)

	params := map[string]interface{}{
		"sequence":   packetNew.Sequence,
		"port_id":    packetNew.SourcePort,
		"channel_id": packetNew.SourceChannel,
	}

	isPacketReceived = findPacket(ctx, target, params, order)

	assert.Truef(t, isPacketReceived, "The packet event has NOT received on the target chain.\n%v\n", packetNew)

	t.Logf("relay recovered successfully")
}

func (r *RelayerTestSuite) RelayerCrashTest(ctx context.Context, chainA, chainB chains.Chain) {
	// crash relayer and write block height information for crashed node to file
	packet, _ := r.handleCrashAndSendPacket(ctx, chainA, chainB)

	// recover relayer now
	err := r.Recover(ctx, time.Second*30)
	r.Require().NoErrorf(err, "Unable to recover relayer %v", err)
	isPacketReceived := r.checkPacketReceipt(ctx, chainB, packet, ibc.Unordered)
	assert.Truef(r.T, isPacketReceived, "The packet event has NOT received on the target chain.\n%v\n", packet)

	// check if relay is working as expected with ping pong to cross chain
	msg := "new-message"
	response, err := r.SendPacket(ctx, chainA, chainB, msg, 1000, false)
	assert.NoErrorf(r.T, err, "Error on sending packet (%s) %v", msg, err)
	assert.Truef(r.T, response.IsPacketSent, "The packet has not been sent to the target chain.")
	assert.Truef(r.T, response.IsPacketReceiptEventFound, "The packet event has NOT been received on the target chain.")

	r.T.Logf("relay recovered successfully")
}

func (r *RelayerTestSuite) checkPacketReceipt(ctx context.Context, targetChain chains.Chain, packet types.Packet, order ibc.Order) bool {
	params := map[string]interface{}{
		"sequence":   packet.Sequence,
		"port_id":    packet.SourcePort,
		"channel_id": packet.SourceChannel,
	}
	isPacketReceived := findPacket(ctx, targetChain, params, order)
	return isPacketReceived
}

func (r *RelayerTestSuite) handleCrashAndSendPacket(ctx context.Context, src chains.Chain, target chains.Chain) (types.Packet, uint64) {
	callbackA := r.WriteCurrentBlockHeight(ctx, src)
	callbackB := r.WriteCurrentBlockHeight(ctx, target)
	crashedHeight, err := src.(ibc.Chain).Height(ctx)
	err = r.CrashRelayer(ctx, callbackA, callbackB)
	assert.NoErrorf(r.T, err, "Error on relayer crash %v", err)
	chainID := src.(ibc.Chain).Config().ChainID
	r.T.Logf("crashed at: %s %d", chainID, crashedHeight)
	// send packet from src to target crashed node and check if it is received
	var msg = fmt.Sprintf("data-%s", chainID)
	response, _ := r.SendPacket(ctx, src, target, msg, 1000000, false)
	packet := response.Packet
	assert.NotEqualf(r.T, types.Packet{}, packet, "packet is empty")
	assert.Truef(r.T, response.IsPacketSent, "The packet has not been sent to the target chain.")
	assert.Falsef(r.T, response.IsPacketReceiptEventFound, "The packet event has already been received on the target chain.")
	return packet, crashedHeight
}

func findPacket(ctx context.Context, chain chains.Chain, params map[string]interface{}, order ibc.Order) bool {
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
			isPacketReceived = chain.IsPacketReceived(ctx, params, order)
			if isPacketReceived {
				return isPacketReceived
			}
			time.Sleep(interval)
		}
	}
}

func (r *RelayerTestSuite) multiplePacketsOnSameHeight(src, dst chains.Chain, height uint64, numPackets uint) error {
	ctx := context.Background()
	res, err := r.SendPacket(ctx, src, dst, fmt.Sprintf("test-%d", numPackets), height, true)
	if err != nil {
		return err
	}
	if !res.IsPacketSent {
		return fmt.Errorf("packet not sent")
	}
	numPackets -= 1
	if numPackets == 0 {
		return nil
	}
	return r.multiplePacketsOnSameHeight(src, dst, height, numPackets)
}
