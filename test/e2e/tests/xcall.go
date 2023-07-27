package tests

import (
	"context"
	"fmt"
	"strconv"
	"strings"
	"testing"
	"time"

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
		x.TestOneWayMessage(ctx, t, chainB, chainA)
		// x.TestOneWayMessage(ctx, t, chainA, chainB)
	})

	x.T.Run("xcall one way message with given size", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestOneWayMessageWithSize(ctx, t, 1868, chainB, chainA)
		// x.TestOneWayMessageWithSize(ctx, t, 1869, chainB, chainA) Fails
		// x.TestOneWayMessageWithSize(ctx, t, 1000, chainA, chainB)
		// x.TestOneWayMessageWithSize(ctx, t, 1866, chainA, chainB) Fail
	})

	x.T.Run("xcall test rollback", func(t *testing.T) {
		chainA, chainB := x.GetChains()
		x.TestRollback(ctx, t, chainB, chainA)
		// x.TestRollback(ctx, t, chainA, chainB)
	})
}

func (x *XCallTestSuite) TestOneWayMessage(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "MessageTransferTestingWithoutRollback"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	_, _msgData, err := chainB.ExecuteCall(ctx, chainB, reqId, data)
	x.Require().NoError(err)

	fmt.Println("msgData from executeCall MessageReceived: ", chainB.(ibc.Chain).Config().ChainID, _msgData)

	dataOutput, err := convertToPlainString(_msgData)
	x.Require().NoError(err)

	assert.Equal(t, msg, dataOutput)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) TestOneWayMessageWithSize(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, _msg, nil)
	if err != nil {
		t.Fatalf("Error in TestOneWayMessageWithSize Xcall transfer: %v with %v size", err, len(_msg))
	}
	ctx, data, err = chainB.ExecuteCall(ctx, chainB, reqId, data)
	if err != nil {
		t.Fatalf("Error in TestOneWayMessageWithSize ExecuteCall transfer: %v with %v size", err, len(_msg))
	}
	fmt.Println("msgData from executeCall MessageReceived: ", chainB.(ibc.Chain).Config().ChainID, data)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) TestRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "MessageTransferTestingWithRollback"
	rollback := "RollbackMessageData"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	fmt.Println("testing TestRollback from " + chainA.(ibc.Chain).Config().ChainID + " to " + chainB.(ibc.Chain).Config().ChainID)
	sn, reqId, _data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
	x.Require().NoError(err)

	fmt.Println("Calling ExecuteCall")
	ctx, msg, err = chainB.ExecuteCall(ctx, chainB, reqId, _data)
	assert.NoError(t, err)
	fmt.Println("msgData from executeCall MessageReceived: ", chainB.(ibc.Chain).Config().ChainID, msg)
	time.Sleep(10 * time.Second)

	heightB, err := chainB.(ibc.Chain).Height(ctx)
	assert.NoError(t, err)

	code, msg, err := chainB.FindCallResponse(ctx, int64(heightB), sn)
	fmt.Printf("Checking Error Message on Chain B Response EventLogs: %v --> %v --> %v\n", err, msg, code)

	heightA, err := chainA.(ibc.Chain).Height(ctx)
	// assert.NoError(t, err)
	fmt.Println("heightA: ", heightA)

	rollbackSn, _msg, err := chainA.FindCallResponse(ctx, int64(heightA), sn)
	fmt.Println("rollbackSn: ", rollbackSn)
	fmt.Println("msg: ", _msg)

	fmt.Printf("Checking Error Message on Rollback EventLogs: %v --> %v\n", err, rollbackSn)
	// assert.Equal(t, err, "failed to find eventLog")

	// ctx, err = chainA.ExecuteRollback(ctx, sn)
	// time.Sleep(10 * time.Second)
	// fmt.Println("TXResult: ", ctx.Value("txResult"))

	// fmt.Println("ERROR: ", err)

	// if err != nil {
	// 	t.Fatalf("Error in TestRollback ExecuteRollback from %v to %v: %v", chainA.(ibc.Chain).Config().ChainID, chainB.(ibc.Chain).Config().ChainID, err)
	// }

	// x.Require().NoError(err)
}

func convertToPlainString(input string) (string, error) {
	if strings.HasPrefix(input, "0x") {
		input = input[2:]
	}

	if strings.HasPrefix(input, "[") && strings.HasSuffix(input, "]") {
		input = input[1 : len(input)-1]

		parts := strings.Split(input, ", ")
		var plainString strings.Builder
		for _, part := range parts {
			value, err := strconv.Atoi(part)
			if err != nil {
				return "", err
			}
			plainString.WriteByte(byte(value))
		}

		return plainString.String(), nil
	}

	if len(input)%2 != 0 {
		return "", fmt.Errorf("invalid input length")
	}

	var plainString strings.Builder
	for i := 0; i < len(input); i += 2 {
		value, err := strconv.ParseUint(input[i:i+2], 16, 8)
		if err != nil {
			return "", err
		}
		plainString.WriteByte(byte(value))
	}

	return plainString.String(), nil
}
