package tests

import (
	"context"
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
	msg := "MessageTransferTestingWithoutRollback"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	_, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), nil)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, reqId, data)
	x.Require().NoError(err)

	dataOutput, err := convertToPlainString(data)
	x.Require().NoError(err)

	assert.Equal(t, msg, dataOutput)
	fmt.Println("Data Transfer Testing Without Rollback from " + chainA.(ibc.Chain).Config().ChainID + " to " + chainB.(ibc.Chain).Config().ChainID + " with data " + msg + " and Received:" + dataOutput + " PASSED")
}

func (x *XCallTestSuite) TestRollback(ctx context.Context, t *testing.T, chainA, chainB chains.Chain) {
	msg := "rollback"
	rollback := "RollbackDataTesting"
	dst := chainB.(ibc.Chain).Config().ChainID + "/" + chainB.GetIBCAddress("dapp")
	sn, reqId, data, err := chainA.XCall(context.Background(), chainB, testsuite.User, dst, []byte(msg), []byte(rollback))
	x.Require().NoError(err)
	height, err := chainA.(ibc.Chain).Height(ctx)
	x.Require().NoError(err)
	ctx, err = chainB.ExecuteCall(ctx, reqId, data)
	code, err := chainA.FindCallResponse(ctx, int64(height), sn)
	x.Require().NoError(err)
	assert.Equal(t, "0", code)
	ctx, err = chainA.ExecuteRollback(ctx, sn)
	x.Require().NoError(err)
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
