package int_test

import (
	"context"
	"github.com/icon-project/ibc-integration/test/integration/tests"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"testing"

	"github.com/stretchr/testify/suite"
)

func TestIntegrationSuite(t *testing.T) {
	suite.Run(t, new(IntegrationTest))
}

type IntegrationTest struct {
	testsuite.E2ETestSuite
}

func (s *IntegrationTest) TestE2E_all() {
	t := s.T()
	ctx := context.TODO()

	t.Run("test relayer", func(t *testing.T) {
		ctx, rly, err := s.SetupRelayer(ctx)
		s.Require().NoError(err)
		s.StartRelayer(rly)
		relayer := tests.RelayerTestSuite{
			E2ETestSuite: &s.E2ETestSuite,
			T:            t,
		}
		relayer.TestRelayer(ctx)
	})
}
