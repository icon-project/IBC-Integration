package tests

import (
	"context"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"golang.org/x/sync/errgroup"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestRelayer(ctx context.Context) {
	chainA, chainB := r.GetChains()

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

	r.T.Run("test single relay packet flow", func(t *testing.T) {
		pong, err := r.Ping(context.Background())
		r.Require().NoError(err)
		r.Require().Equal("pong", pong)
	})

	r.T.Run("crash nodes", func(t *testing.T) {
		// // crash chainA
		// r.Require().NoError(r.CrashNode(ctx, chainA))
		// // send packet from chainB to chainA crashed node and check if it is received
		// ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
		// defer cancel()
		// res, err := r.PacketFlow(ctx, chainB, chainA, "crash-chainA")
		// r.Require().Errorf(err, "crashed node should not receive packet")
		// // recover crashed node
		// r.Require().NoError(r.ResumeNode(ctx, chainA))
		// res, err := r.Ping(context.Background())
		// r.Require().NoError(err)
		// r.Require().Equal("pong", res)
	})

	r.T.Run("test crash and recover relay", func(t *testing.T) {
		var eg errgroup.Group

		eg.Go(func() error {
			return r.CrashNode(ctx, chainA)
		})
		eg.Go(func() error {
			return r.CrashNode(ctx, chainB)
		})
		r.Require().NoError(eg.Wait())

		// send packet from chainA to chainB crashed node and check if it is received
		ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
		defer cancel()
		commitmenIdA, chainAdata, err := r.PacketFlow(ctx, chainA, chainB, "crash-chainA")
		r.Require().NoError(err)

		// send packet from chainB to chainA crashed node and check if it is received
		commitmenIdB, chainBdata, err := r.PacketFlow(ctx, chainB, chainA, "crash-chainA")
		r.Require().NoError(err)

		// crash relayer
		crashedAt, err := r.Crash(ctx)
		r.Require().NoError(err)
		t.Logf("crash took: %v", crashedAt)

		// recover chainA
		eg.Go(func() error {
			return r.ResumeNode(ctx, chainA)
		})
		// recover chainB
		eg.Go(func() error {
			return r.ResumeNode(ctx, chainB)
		})
		r.Require().NoError(eg.Wait())

		// recover relayer now
		recoverdDurarion, err := r.Recover(ctx, crashedAt)
		r.Require().NoError(err)
		t.Logf("probably recovered, took: %v", recoverdDurarion)

		// check if packet was received in a recovered state
		chainAres, err := r.QueryPacketCommitment(ctx, chainB, commitmenIdA, chainAdata)
		r.Require().NoError(err)
		r.Require().Equal("crash-chainA", chainAres)

		// check if packet was received in a recovered state
		chainBres, err := r.QueryPacketCommitment(ctx, chainA, commitmenIdB, chainBdata)
		r.Require().NoError(err)
		r.Require().Equal("crash-chainB", chainBres)

		// check if relay is working with ping pong to cross chain
		pong, err := r.Ping(context.Background())
		r.Require().Error(err)
		r.Require().NotEqual("pong", pong)
		t.Log("relay recovered")
	})
}
