package tests

import (
	"context"
	"fmt"
	"testing"
	"time"

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
	r.T.Run("test client state", func(t *testing.T) {
		r.Require().NoError(r.CreateClient(ctx))
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

	r.T.Run("test connection", func(t *testing.T) {
		portID := "transfer"
		r.Require().NoError(r.CreateConnection(ctx))
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

	r.T.Run("test single relay packet flow", func(t *testing.T) {
		r.Require().NoError(r.Ping(context.Background()))
	})

	r.T.Run("Crash chainA and relay", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		r.Require().NoError(r.CrashTest(context.Background(), chainA, chainB))
		r.Require().NoError(r.CrashTest(context.Background(), chainB, chainA))
	})
}

func (r *RelayerTestSuite) CrashTest(ctx context.Context, chainA, chainB chains.Chain) error {
	// crash Chain
	if err := r.CrashNode(ctx, chainB); err != nil {
		return err
	}

	// send packet from chainA to chainB crashed node and check if it is received
	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()
	var msg string = chainA.(ibc.Chain).Config().ChainID
	res, err := r.PacketFlow(ctx, chainA, chainB, msg, nil)
	if err != nil {
		return err
	}

	// crash relayer
	crashedAt, err := r.Crash(ctx)
	if err != nil {
		return err
	}
	r.T.Logf("crash took: %v", crashedAt)

	// recover chainB
	if err := r.ResumeNode(ctx, chainB); err != nil {
		return err
	}

	// recover relayer now
	recoverdDurarion, err := r.Recover(ctx, crashedAt)
	if err != nil {
		return err
	}
	r.T.Logf("probably recovered, took: %v", recoverdDurarion)

	// check if packet was received in a recovered state
	if err := r.QueryPacketCommitment(ctx, chainB, res.RequestID, res.Data); err != nil {
		return err
	}
	if msg != res.Data {
		return fmt.Errorf("expected packet data to be %s but got %s", msg, res.Data)
	}

	// check if relay is working as expected with ping pong to cross chain
	if err := r.Ping(context.Background()); err != nil {
		return err
	}
	r.T.Log("relay recovered successfully")
	return nil
}
