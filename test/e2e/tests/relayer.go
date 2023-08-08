package tests

import (
	"context"
	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"testing"
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
		r.Require().NoError(err, res)
		res, err = r.GetClientState(ctx, chainB, 0)
		r.Require().NoError(err, res)
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
		seq, err := r.GetConnectionState(ctx, chainA, 0)
		r.Require().NoError(err)
		t.Log(seq)
		seq, err = r.GetConnectionState(ctx, chainB, 0)
		r.Require().NoError(err)
		t.Log(seq)
	})

	r.T.Run("test crash and recover", func(t *testing.T) {
		duration, err := r.CrashAndRecover(ctx)
		r.Require().NoError(err)
		t.Logf("recover took: %v", duration)
	})
}
