package e2e_test

import (
	"context"
	"github.com/icon-project/ibc-integration/test/e2e/tests"
	"github.com/icon-project/ibc-integration/test/e2e/testsuite"
	"github.com/stretchr/testify/suite"
	"testing"
)

func TestE2ETestSuite(t *testing.T) {
	suite.Run(t, new(E2ETest))
}

type E2ETest struct {
	testsuite.E2ETestSuite
}

func (s *E2ETest) TestE2E_all() {

	t := s.T()
	ctx := context.TODO()
	relayer := s.SetupChainsAndRelayer(ctx)
	s.StartRelayer(relayer)

	t.Run("test xcall", func(t *testing.T) {
		xcall := tests.XCallTestSuite{
			E2ETestSuite: &s.E2ETestSuite,
			T:            t,
		}
		xcall.TestDemo()
	})
	s.StopRelayer(ctx, relayer)
}
