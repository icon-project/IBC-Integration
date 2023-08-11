package tests

import (
	"context"
	"testing"

	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestRelayer(ctx context.Context) {
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
		portID := "transfer"
		r.SetupXCall(ctx, portID)
		r.DeployMockApp(ctx, portID)
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

	r.T.Run("test packet flow", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		r.PacketFlow(ctx, chainA, chainB, "test")
	})

	r.T.Run("crash nodes", func(t *testing.T) {
		chainA, chainB := r.GetChains()
		r.Require().NoError(r.CrashNode(ctx, chainA))
		r.Require().NoError(r.CrashNode(ctx, chainB))
	})

	r.T.Run("test crash and recover relay", func(t *testing.T) {
		crashedAt, err := r.Crash(ctx)
		r.Require().NoError(err)
		t.Logf("crash took: %v", crashedAt)
		pong, err := r.Ping(context.Background())
		r.Require().Error(err)
		r.Require().NotEqual("pong", pong)
		staredAt, err := r.Recover(ctx, crashedAt)
		r.Require().NoError(err)
		t.Logf("recover took: %v", staredAt)
		pong, err = r.Ping(context.Background())
		r.Require().NoError(err)
		r.Require().Equal("pong", pong)
	})
}
