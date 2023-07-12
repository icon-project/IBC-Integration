package tests

import (
	"context"
	"fmt"
	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/stretchr/testify/assert"
	"testing"
	"time"
)

type XCallTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (x *XCallTestSuite) TestDemo() {
	portId := "transfer"
	ctx := context.TODO()
	x.SetupXCall(ctx, portId)

	x.T.Run("xcall test", func(t *testing.T) {
		x.DeployMockApp(ctx, portId)
		chainA, chainB := x.GetChains()
		msg := "Hello"
		dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
		_, reqId, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
		x.Require().NoError(err)
		ctx, err = chainB.ExecuteCall(ctx, reqId)
		fmt.Println(ctx.Value("txResult"))

		msg = "rollback"
		rollback := "rollback data"
		sn, reqId, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))

		ctx, err = chainB.ExecuteCall(ctx, reqId)
		fmt.Println(ctx.Value("txResult"))
		time.Sleep(10 * time.Second)

		ctx, err = chainA.ExecuteRollback(ctx, sn)
		fmt.Println(ctx.Value("txResult"))
	})

	x.T.Run("another test", func(t *testing.T) {
		assertions := assert.New(t)
		assertions.Equal(123, 123, "they should be equal")
	})

}
