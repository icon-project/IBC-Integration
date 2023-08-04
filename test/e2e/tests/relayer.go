package tests

import (
	"context"
	"fmt"
	"testing"

	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
)

type RelayerTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

func (r *RelayerTestSuite) TestRelayer(ctx context.Context) {
	clients, err := r.CreateClient(ctx)
	r.Require().NoError(err)
	fmt.Println("Clients created", clients)
}