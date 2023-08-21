package tests

import (
	"context"
	"errors"
	"fmt"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/stretchr/testify/assert"
	"strconv"
	"strings"
	"testing"
)

type XCallTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (x *XCallTestSuite) TextXCall() {
	testcase := "xcall"
	portId := "transfer"
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	x.Require().NoError(x.SetupXCall(ctx, portId, 100), "fail to setup xcall")
	x.Require().NoError(x.DeployXCallMockApp(ctx, portId), "fail to deploy xcall dapp")
	chainA, chainB := x.GetChains()
	x.T.Run("xcall one way message chainA-chainB", func(t *testing.T) {

		x.testOneWayMessage(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall one way message chainB-chainA", func(t *testing.T) {

		x.testOneWayMessage(ctx, t, chainB, chainA)
	})

	x.T.Run("xcall test rollback chainA-chainB", func(t *testing.T) {
		x.testRollback(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall test rollback chainB-chainA", func(t *testing.T) {
		x.testRollback(ctx, t, chainB, chainA)
	})

	x.T.Run("xcall test send maxSize Data: 2048 bytes", func(t *testing.T) {
		x.testOneWayMessageWithSize(ctx, t, 1300, chainA, chainB)
		x.testOneWayMessageWithSize(ctx, t, 1300, chainB, chainA)
	})

	x.T.Run("xcall test send maxSize Data: 2049bytes", func(t *testing.T) {
		x.testOneWayMessageWithSizeExpectingError(ctx, t, 2000, chainB, chainA)
		x.testOneWayMessageWithSizeExpectingError(ctx, t, 2000, chainA, chainB)
	})

}

func (x *XCallTestSuite) TestXCallPacketDrop() {
	testcase := "packet-drop"
	portId := "transfer-1"
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	x.Require().NoError(x.SetupXCall(ctx, portId, 1), "fail to setup xcall")
	x.Require().NoError(x.DeployXCallMockApp(ctx, portId), "fail to deploy xcall dapp")
	chainA, chainB := x.GetChains()
	x.T.Run("xcall packet drop chainA-chainB", func(t *testing.T) {
		x.testPacketDrop(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall packet drop chainB-chainA", func(t *testing.T) {
		x.testPacketDrop(ctx, t, chainB, chainA)
	})
}

func (x *XCallTestSuite) testPacketDrop(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "drop-msg"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	//height, _ := chainA.(ibc.Chain).Height(ctx)
	listener := chainA.InitEventListener(ctx, "ibc")
	defer listener.Stop()
	res, err := chainA.XCall(ctx, chainB, testsuite.User, dst, []byte(msg), nil)

	x.Require().Errorf(err, "failed to find eventlog")
	sn := res.SerialNo
	snInt, _ := strconv.Atoi(res.SerialNo)
	params := map[string]interface{}{
		"port_id":    "transfer-1",
		"channel_id": "channel-0",
		"sequence":   uint64(snInt),
	}

	ctx, err = chainB.CheckForTimeout(ctx, params, listener)
	response := ctx.Value("timeout-response").(*chains.TimeoutResponse)
	assert.Truef(t, response.HasTimeout, "timeout event not found - %s", sn)
	assert.Falsef(t, response.IsPacketFound, "packet found on target chain - %s", sn)
}

func (x *XCallTestSuite) testOneWayMessage(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "MessageTransferTestingWithoutRollback"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	res, err := chainA.XCall(ctx, chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	x.Require().NoError(err)
	dataOutput, err := x.ConvertToPlainString(res.Data)
	x.Require().NoError(err)

	assert.Equal(t, msg, dataOutput)
	fmt.Println("Data Transfer Testing Without Rollback from " + chainA.(ibc.Chain).Config().ChainID + " to " + chainB.(ibc.Chain).Config().ChainID + " with data " + msg + " and Received:" + dataOutput + " PASSED")
}

func (x *XCallTestSuite) testRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "rollback"
	rollback := "RollbackDataTesting"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	res, err := chainA.XCall(ctx, chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
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

func (x *XCallTestSuite) testOneWayMessageWithSize(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	res, err := chainA.XCall(ctx, chainB, testsuite.User, dst, _msg, nil)
	assert.NoError(t, err)

	_, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) testOneWayMessageWithSizeExpectingError(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	_, err := chainA.XCall(ctx, chainB, testsuite.User, dst, _msg, nil)
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
