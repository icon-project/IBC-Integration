package tests

import (
	"context"
	"fmt"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestRelayer() {
	ctx := context.TODO()
	portID := "transfer"
	r.T.Run("client state", func(t *testing.T) {
		chainA, chainB := r.GetChains()
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

	r.T.Run("connection", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		stateA, err := r.GetConnectionState(ctx, chainA, 0)
		t.Log(stateA)
		r.Require().NoError(err)
		r.Require().Equal(stateA.GetState(), int32(3))
		stateB, err := r.GetConnectionState(ctx, chainB, 0)
		t.Log(stateB)
		r.Require().NoError(err)
		r.Require().Equal(stateB.GetState(), int32(3))
		seq, err := r.GetNextConnectionSequence(ctx, chainA)
		r.Require().NoError(err)
		r.Require().Equal(1, seq)
		seq, err = r.GetNextConnectionSequence(ctx, chainB)
		r.Require().NoError(err)
		r.Require().Equal(1, seq)
		r.Require().NoError(r.SetupXCall(ctx, portID))
		r.Require().NoError(r.CreateChannel(ctx, portID))
		r.Require().NoError(r.DeployMockApp(ctx, portID))
		res, err := r.GetChannel(ctx, chainA, 0, portID)
		r.Require().NoError(err)
		t.Log(res)
		res, err = r.GetChannel(ctx, chainB, 0, portID)
		r.Require().NoError(err)
		t.Log(res)

		seq, err = r.GetChannelSequence(ctx, chainA)
		r.Require().NoError(err)
		r.Require().Equal(1, seq)
		seq, err = r.GetChannelSequence(ctx, chainB)
		r.Require().NoError(err)
		r.Require().Equal(1, seq)
	})

	r.T.Run("single relay packet flow", func(t *testing.T) {
		r.Require().NoError(r.Ping(context.Background()))
	})

	r.T.Run("unordered packet test", func(t *testing.T) {

	})

	r.T.Run("crash and recover relay", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		r.Require().NoError(r.CrashTest(context.Background(), chainB, chainA, portID))
		r.Require().NoError(r.CrashTest(context.Background(), chainB, chainA, portID))
	})
}

func (r *RelayerTestSuite) CrashTest(ctx context.Context, chainA, chainB chains.Chain, portID string) error {

	// Get current height before crash
	height, err := chainB.(ibc.Chain).Height(ctx)
	if err != nil {
		return err
	}

	// crash Node
	if err := r.CrashNode(context.Background(), chainB); err != nil {
		return err
	}

	// send packet from chainA to chainB crashed node and check if it is received
	var msg = chainA.(ibc.Chain).Config().ChainID
	xcall, err := r.SendPacket(context.Background(), chainA, chainB, msg)
	if err != nil {
		return err
	}

	// crash relayer and write block height information for crashed node to file
	crashedAt, err := r.Crash(ctx, chainB.(ibc.Chain).Config().ChainID, height)
	if err != nil {
		return err
	}
	r.T.Logf("crash at: %v", crashedAt)

	// recover crashed node now
	if err := r.ResumeNode(ctx, chainB); err != nil {
		return err
	}
	// recover relayer now
	recoverdDurarion, err := r.Recover(ctx, crashedAt)
	if err != nil {
		return err
	}

	// check if packet was sent in a recovered state
	channel, err := r.GetChannel(ctx, chainB, 0, portID)
	if err != nil {
		return err
	}
	res, err := r.GetPacketReceipt(xcall, chainB, channel.Counterparty.ChannelId, portID)
	if err != nil {
		return err
	}

	// check if packet was received in a recovered state
	if err := r.QueryPacketCommitment(ctx, chainB, res.RequestID, res.Data); err != nil {
		return err
	}
	data, err := r.ConvertToPlainString(res.Data)
	if err != nil {
		return err
	}
	if msg != data {
		return fmt.Errorf("expected packet data to be %s but got %s", msg, res.Data)
	}

	// check if relay is working as expected with ping pong to cross chain
	if err := r.Ping(context.Background()); err != nil {
		return err
	}
	r.T.Logf("relay recovered, crashed: %v, recovered: %v", crashedAt, crashedAt.Add(recoverdDurarion))
	return nil
}
