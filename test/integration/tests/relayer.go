package tests

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"golang.org/x/sync/errgroup"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestRelayer() {
	r.T.Run("client state", func(t *testing.T) {
		r.TestClientState()
	})

	r.T.Run("connection", func(t *testing.T) {
		r.TestConnection()
	})

	r.T.Run("single relay packet flow", func(t *testing.T) {
		r.TestSingleRelayPacketFlow()
	})

	r.T.Run("multi relay packet flow", func(t *testing.T) {
		r.TestMultiRelayPacketFlow()
	})

	r.T.Run("unordered packet test", func(t *testing.T) {
		r.TestUnorderedPacket()
	})

	r.T.Run("crash and recover relay", func(t *testing.T) {
		r.TestCrashAndRecoverRelay()
	})
}

func (r *RelayerTestSuite) TestClientState() {
	ctx := context.TODO()
	chainA, chainB := r.GetChains()
	res, err := r.GetClientState(ctx, chainA, 0)
	r.T.Log(res)
	r.Require().NoError(err)
	res, err = r.GetClientState(ctx, chainB, 0)
	r.T.Log(res)
	r.Require().NoError(err)
	count, err := r.GetClientSequence(ctx, chainA)
	r.Require().NoError(err)
	r.Require().Equal(1, count)
	count, err = r.GetClientSequence(ctx, chainB)
	r.Require().NoError(err)
	r.Require().Equal(1, count)
}

func (r *RelayerTestSuite) TestConnection() {
	ctx := context.TODO()
	chainA, chainB := r.GetChains()
	stateA, err := r.GetConnectionState(ctx, chainA, 0)
	r.T.Log(stateA)
	r.Require().NoError(err)
	r.Require().Equal(stateA.GetState(), int32(3))
	stateB, err := r.GetConnectionState(ctx, chainB, 0)
	r.T.Log(stateB)
	r.Require().NoError(err)
	r.Require().Equal(stateB.GetState(), int32(3))
	seq, err := r.GetNextConnectionSequence(ctx, chainA)
	r.Require().NoError(err)
	r.Require().Equal(1, seq)
	seq, err = r.GetNextConnectionSequence(ctx, chainB)
	r.Require().NoError(err)
	r.Require().Equal(1, seq)
	portID := "transfer"
	r.Require().NoError(r.SetupXCall(ctx, portID))
	r.Require().NoError(r.CreateChannel(ctx, portID))
	r.Require().NoError(r.DeployMockApp(ctx, portID))
	res, err := r.GetChannel(ctx, chainA, 0, portID)
	r.Require().NoError(err)
	r.T.Log(res)
	res, err = r.GetChannel(ctx, chainB, 0, portID)
	r.Require().NoError(err)
	r.T.Log(res)

	seq, err = r.GetChannelSequence(ctx, chainA)
	r.Require().NoError(err)
	r.Require().Equal(1, seq)
	seq, err = r.GetChannelSequence(ctx, chainB)
	r.Require().NoError(err)
	r.Require().Equal(1, seq)
}

func (r *RelayerTestSuite) TestSingleRelayPacketFlow() {
	r.Require().NoError(r.Ping(context.Background()))
}

func (r *RelayerTestSuite) TestMultiRelayPacketFlow() {
	r.Require().NoError(r.Ping(context.Background()))
}

func (r *RelayerTestSuite) TestUnorderedPacket() {
	var (
		eg       errgroup.Group
		messages = []string{"1", "2", "3", "4", "5"}
		resChan  = make(chan string, len(messages))
		result   []string
	)
	chainA, chainB := r.GetChains()
	for i, msg := range messages {
		exec := i
		data := msg
		eg.Go(func() error {
			chainA, chainB := func(chainA, chainB chains.Chain) (chains.Chain, chains.Chain) {
				if exec%2 == 0 {
					return chainA, chainB
				}
				return chainB, chainA
			}(chainA, chainB)
			res, err := r.PacketFlow(context.Background(), chainA, chainB, data)
			if err != nil {
				return err
			}
			resChan <- res.SerialNo
			return nil
		})
	}
	go func() {
		if err := eg.Wait(); err != nil {
			r.Require().NoError(err)
		}
		close(resChan)
	}()
	for seq := range resChan {
		result = append(result, seq)
	}
	r.Require().NotEqual(result, messages, "packets were sent in order", result)
}

func (r *RelayerTestSuite) TestCrashAndRecoverRelay() {
	ctx := context.Background()
	chainA, chainB := r.GetChains()
	portID := "transfer"
	r.Require().NoError(r.CrashTest(ctx, chainA, chainB, portID))
	r.Require().NoError(r.CrashTest(ctx, chainB, chainA, portID))
}

func (r *RelayerTestSuite) CrashTest(ctx context.Context, chainA, chainB chains.Chain, portID string) error {
	// crash relayer and write block height information for crashed node to file
	callbackA := r.WriteBlockHeight(ctx, chainA)
	callbackB := r.WriteBlockHeight(ctx, chainB)
	crashedHeight, err := r.Crash(ctx, chainB.(ibc.Chain), callbackA, callbackB)
	if err != nil {
		return err
	}
	// send packet from chainA to chainB crashed node and check if it is received
	msg := chainB.(ibc.Chain).Config().ChainID
	xcall, err := r.SendPacket(ctx, chainA, chainB, msg)
	if err != nil {
		return err
	}
	// recover relayer now
	if err := r.Recover(ctx, time.Second*100); err != nil {
		return err
	}
	// check if packet was sent in a recovered state
	res, err := r.FindPacketSent(xcall, chainA, chainB, crashedHeight)
	if err != nil {
		return fmt.Errorf("%s packet not found: %w", msg, err)
	}
	data, err := r.ConvertToPlainString(res.Data)
	if err != nil {
		return err
	}
	if data != msg {
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
	return r.Ping(ctx)
}
