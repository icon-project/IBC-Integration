package tests

import (
	"context"
	"errors"
	"fmt"
	"strconv"
	"strings"
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
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	_, err = chainB.ExecuteCall(ctx, reqId, data)
	x.Require().NoError(err)

	dataOutput, err := convertToPlainString(data)
	x.Require().NoError(err)

	assert.Equal(t, msg, dataOutput)
}

func (x *XCallTestSuite) TestRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "rollback"
	rollback := "RollbackDataTesting"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	sn, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
	x.Require().NoError(err)
	height, err := chainA.(ibc.Chain).Height(ctx)
	x.Require().NoError(err)
	_, err = chainB.ExecuteCall(ctx, reqId, data)
	x.Require().NoError(err)
	code, err := chainA.FindCallResponse(ctx, int64(height), sn)
	x.Require().NoError(err)
	assert.Equal(t, "0", code)
	_, err = chainA.ExecuteRollback(ctx, sn)
	x.Require().NoError(err)
}

func (x *XCallTestSuite) TestOneWayMessageWithSize(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, _msg, nil)
	assert.NoError(t, err)

	_, err = chainB.ExecuteCall(ctx, reqId, data)
	assert.NoError(t, err)
}

func (x *XCallTestSuite) TestOneWayMessageWithSizeExpectingError(ctx context.Context, t *testing.T, dataSize int, chainA, chainB chains.Chain) {
	_msg := make([]byte, dataSize)
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, _, _, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, _msg, nil)
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
