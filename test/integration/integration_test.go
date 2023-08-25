package int_test

import (
	"context"
	"github.com/icon-project/ibc-integration/test/integration/tests"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/stretchr/testify/assert"
	"testing"

	"github.com/stretchr/testify/suite"
)

func TestIntegrationSuite(t *testing.T) {
	suite.Run(t, new(IntegrationTest))
}

type IntegrationTest struct {
	testsuite.E2ETestSuite
}

func (s *IntegrationTest) TestIntegration_all() {
	t := s.T()
	ctx := context.TODO()
	s.Require().NoError(s.SetCfg())
	rly, err := s.SetupRelayer(ctx)

	assert.NoErrorf(t, err, "Error while setting up relayer, %v", err)

	test := tests.RelayerTestSuite{
		E2ETestSuite: &s.E2ETestSuite,
		T:            t,
	}
	t.Run("test client creation", func(t *testing.T) {
		ctx = context.WithValue(ctx, "testcase", "client")
		test.TestClientCreation(ctx, rly)
	})

	t.Run("test connection", func(t *testing.T) {
		ctx = context.WithValue(ctx, "testcase", "connection")
		test.TestConnection(ctx, rly)
	})

	t.Run("test relayer", func(t *testing.T) {
		test.TestRelayer(ctx, rly)
	})
}
