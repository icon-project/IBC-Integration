package tests

import (
	"context"
	"errors"
	"fmt"
	"math/big"
	"strconv"
	"strings"
	"testing"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/stretchr/testify/assert"
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
	x.T.Run("test xcall", func(t *testing.T) {
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
	})
	// new send message tests
	x.T.Run("test New xcall", func(t *testing.T) {
		x.T.Run("xcall new one way message chainA-chainB", func(t *testing.T) {

			x.testOneWayMessage(ctx, t, chainA, chainB, true)
		})

		x.T.Run("xcall new one way message chainB-chainA", func(t *testing.T) {

			x.testOneWayMessage(ctx, t, chainB, chainA, true)
		})

		x.T.Run("xcall new test rollback chainA-chainB", func(t *testing.T) {
			x.testRollback(ctx, t, chainA, chainB, true)
		})

		x.T.Run("xcall new test rollback chainB-chainA", func(t *testing.T) {
			x.testRollback(ctx, t, chainB, chainA, true)
		})

		x.T.Run("xcall new test send maxSize Data: 2048 bytes", func(t *testing.T) {
			x.testOneWayMessageWithSize(ctx, t, 1300, chainA, chainB, true)
			x.testOneWayMessageWithSize(ctx, t, 1300, chainB, chainA, true)
		})

		x.T.Run("xcall new test send maxSize Data: 2049bytes", func(t *testing.T) {
			x.testOneWayMessageWithSizeExpectingError(ctx, t, 2000, chainB, chainA, true)
			x.testOneWayMessageWithSizeExpectingError(ctx, t, 2000, chainA, chainB, true)
		})
	})

}

func (x *XCallTestSuite) TestXCallPacketDrop() {
	testcase := "packet-drop"
	portId := "transfer-1"
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	x.Require().NoError(x.SetupXCall(ctx, portId, 10), "fail to setup xcall")
	x.Require().NoError(x.DeployXCallMockApp(ctx, portId), "fail to deploy xcall dapp")
	chainA, chainB := x.GetChains()
	x.T.Run("test xcall packet drop", func(t *testing.T) {
		x.T.Run("xcall packet drop chainA-chainB", func(t *testing.T) {
			x.testPacketDrop(ctx, t, chainA, chainB)
		})

		x.T.Run("xcall packet drop chainB-chainA", func(t *testing.T) {
			x.testPacketDrop(ctx, t, chainB, chainA)
		})
	})

	// new send message related test
	x.T.Run("test new xcall packet drop", func(t *testing.T) {
		x.T.Run("xcall new packet drop chainA-chainB", func(t *testing.T) {
			x.testPacketDrop(ctx, t, chainA, chainB, true)
		})

		x.T.Run("xcall new packet drop chainB-chainA", func(t *testing.T) {
			x.testPacketDrop(ctx, t, chainB, chainA, true)
		})
	})
}

func (x *XCallTestSuite) testPacketDrop(ctx context.Context, t *testing.T, chainA, chainB chains.Chain, newFunctionCall ...bool) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "drop-msg"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	//height, _ := chainA.(ibc.Chain).Height(ctx)
	listener := chainA.InitEventListener(ctx, "ibc")
	defer listener.Stop()
	// res, err := chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), []byte("rollback-data"))
	var res *chains.XCallResponse
	var err error
	if len(newFunctionCall) > 0 && newFunctionCall[0] {
		msgType := big.NewInt(2)
		res, err = chainA.NewXCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), msgType, []byte("rollback-data"))
	} else {
		res, err = chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), []byte("rollback-data"))
	}
	assert.Errorf(t, err, "failed to find eventlog - %w", err)
	sn := res.SerialNo
	snInt, _ := strconv.Atoi(res.SerialNo)
	params := map[string]interface{}{
		"port_id":    "transfer-1",
		"channel_id": "channel-0",
		"sequence":   uint64(snInt),
	}

	ctx, err = chainA.CheckForTimeout(ctx, chainB, params, listener)
	response := ctx.Value("timeout-response").(*chains.TimeoutResponse)
	assert.Truef(t, response.HasTimeout, "timeout event not found - %s", sn)
	assert.Falsef(t, response.IsPacketFound, "packet found on target chain - %s", sn)
	assert.Truef(t, response.HasRollbackCalled, "failed to call rollback  - %s", sn)
}

func (x *XCallTestSuite) testOneWayMessage(ctx context.Context, t *testing.T, chainA, chainB chains.Chain, newFunctionCall ...bool) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "MessageTransferTestingWithoutRollback"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	var res *chains.XCallResponse
	var err error
	if len(newFunctionCall) > 0 && newFunctionCall[0] {
		msgType := big.NewInt(1)
		res, err = chainA.NewXCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), msgType, nil)
	} else {
		res, err = chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), nil)
	}
	assert.NoErrorf(t, err, "error on sending packet- %w", err)
	ctx, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	assert.NoErrorf(t, err, "error on execute call packet- %w", err)
	dataOutput, err := x.ConvertToPlainString(res.Data)
	assert.NoErrorf(t, err, "error on converting res data as msg- %w", err)
	assert.Equal(t, msg, dataOutput)
	fmt.Println("Data Transfer Testing Without Rollback from " + chainA.(ibc.Chain).Config().ChainID + " to " + chainB.(ibc.Chain).Config().ChainID + " with data " + msg + " and Received:" + dataOutput + " PASSED")
}

func (x *XCallTestSuite) testRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain, newFunctionCall ...bool) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	msg := "rollback"
	rollback := "RollbackDataTesting"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	var res *chains.XCallResponse
	var err error
	if len(newFunctionCall) > 0 && newFunctionCall[0] {
		msgType := big.NewInt(2)
		res, err = chainA.NewXCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), msgType, []byte(rollback))
	} else {
		res, err = chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, []byte(msg), []byte(rollback))
	}
	assert.NoErrorf(t, err, "error on sending packet- %w", err)
	height, err := chainA.(ibc.Chain).Height(ctx)
	_, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	code, err := chainA.FindCallResponse(ctx, height, res.SerialNo)
	assert.NoErrorf(t, err, "no call response found %w", err)
	assert.Equal(t, "0", code)
	ctx, err = chainA.ExecuteRollback(ctx, res.SerialNo)
	assert.NoErrorf(t, err, "error on excute rollback- %w", err)
}

func (x *XCallTestSuite) testOneWayMessageWithSize(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain, newFunctionCall ...bool) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	var res *chains.XCallResponse
	var err error
	if len(newFunctionCall) > 0 && newFunctionCall[0] {
		msgType := big.NewInt(1)
		res, err = chainA.NewXCall(ctx, chainB, interchaintest.UserAccount, dst, _msg, msgType, nil)
	} else {
		res, err = chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, _msg, nil)
	}
	assert.NoError(t, err)

	_, err = chainB.ExecuteCall(ctx, res.RequestID, res.Data)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) testOneWayMessageWithSizeExpectingError(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain, newFunctionCall ...bool) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress(dappKey)
	var err error
	if len(newFunctionCall) > 0 && newFunctionCall[0] {
		msgType := big.NewInt(1)
		_, err = chainA.NewXCall(ctx, chainB, interchaintest.UserAccount, dst, _msg, msgType, nil)
	} else {
		_, err = chainA.XCall(ctx, chainB, interchaintest.UserAccount, dst, _msg, nil)
	}
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
