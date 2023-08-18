package tests

import (
	"context"
	"errors"
	"fmt"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"strings"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
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
	x.DeployXCallMockApp(ctx, portId)

	x.T.Run("xcall one way message chainA-chainB", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessage(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall one way message chainB-chainA", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessage(ctx, t, chainB, chainA)
	})

	x.T.Run("xcall test rollback chainA-chainB", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestRollback(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall test rollback chainB-chainA", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestRollback(ctx, t, chainB, chainA)
	})

	x.T.Run("xcall test send maxSize Data: 2048 bytes", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessageWithSize(ctx, t, 1300, chainA, chainB)
		x.TestOneWayMessageWithSize(ctx, t, 1868, chainB, chainA)
	})

	x.T.Run("xcall test send maxSize Data: 2049bytes", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessageWithSizeExpectingError(ctx, t, 1869, chainB, chainA)
		x.TestOneWayMessageWithSizeExpectingError(ctx, t, 2000, chainA, chainB)
	})

}

func (x *XCallTestSuite) TestOneWayMessage(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "MessageTransferTestingWithoutRollback"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	res, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	x.Require().NoError(err)
	dataOutput, err := x.ConvertToPlainString(res.Data)
	x.Require().NoError(err)

	assert.Equal(t, msg, dataOutput)
	fmt.Println("Data Transfer Testing Without Rollback from " + chainA.(ibc.Chain).Config().ChainID + " to " + chainB.(ibc.Chain).Config().ChainID + " with data " + msg + " and Received:" + dataOutput + " PASSED")
}

func (x *XCallTestSuite) TestRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "rollback"
	rollback := "RollbackDataTesting"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	res, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
	x.Require().NoError(err)
	height, err := chainA.(ibc.Chain).Height(ctx)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	code, err := chainA.FindCallResponse(ctx, height, res.SerialNo)
	x.Require().NoError(err)
	assert.Equal(t, "0", code)
	ctx, err = chainA.ExecuteRollback(ctx, res.SerialNo)
	x.Require().NoError(err)
}

func (x *XCallTestSuite) TestOneWayMessageWithSize(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	res, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, _msg, nil)
	assert.NoError(t, err)

	_, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) TestOneWayMessageWithSizeExpectingError(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, _msg, nil)
	if err != nil {
		if strings.Contains(err.Error(), "submessages:") {
			subStart := strings.Index(err.Error(), "submessages:") + len("submessages:")
			subEnd := strings.Index(err.Error(), ": execute")
			subMsg := err.Error()[subStart:subEnd]
			errorMessage := strings.TrimSpace(subMsg)
			assert.Equal(t, errorMessage, "MaxDataSizeExceeded")
		} else {
			assert.Equal(t, err, errors.New("UnknownFailure"))
		}
	}

}
