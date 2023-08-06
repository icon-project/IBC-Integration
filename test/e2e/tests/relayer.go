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
	r.Require().NoError(r.CreateClient(ctx))
	chainA, chainB := r.GetChains()
	res, err := r.GetClientState(ctx, chainA, 0)
	r.Require().NoError(err, res)
	res, err = r.GetClientState(ctx, chainB, 0)
	r.Require().NoError(err, res)
	r.Require().NoError(r.CreateClient(ctx))
}