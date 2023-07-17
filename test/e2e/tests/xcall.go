package tests

import (
	"context"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/stretchr/testify/assert"
)

type XCallTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (x *XCallTestSuite) TestDemo() {
	portId := "transfer"
	ctx := context.TODO()
	x.SetupXCall(ctx, portId)
	x.DeployMockApp(ctx, portId)

	x.T.Run("xcall one way message", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessage(ctx, t, chainA, chainB)
		x.TestOneWayMessage(ctx, t, chainB, chainA)
	})

	x.T.Run("xcall test rollback", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestRollback(ctx, t, chainA, chainB)
		x.TestRollback(ctx, t, chainB, chainA)
	})
}

func (x *XCallTestSuite) TestOneWayMessage(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "Hello"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, reqId, data)
	x.Require().NoError(err)
}

func (x *XCallTestSuite) TestRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "rollback"
	rollback := "rollback data"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	sn, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
	x.Require().NoError(err)
	height, err := chainA.(ibc.Chain).Height(ctx)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, reqId, data)
	code, msg, err := chainA.FindCallResponse(ctx, int64(height), sn)
	x.Require().NoError(err)
	assert.Equal(t, "-1", code)
	ctx, err = chainA.ExecuteRollback(ctx, sn)
	x.Require().NoError(err)
}
