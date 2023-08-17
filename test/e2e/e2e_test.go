package e2e_test

import (
	"context"
	"testing"

	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/stretchr/testify/suite"
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
	s.Require().NoError(s.SetCfg())
	// t.Run("test xcall", func(t *testing.T) {
	// 	rly := s.SetupChainsAndRelayer(ctx)
	// 	s.Require().NoError(s.StartRelayer(rly))
	// 	xcall := tests.XCallTestSuite{
	// 		E2ETestSuite: &s.E2ETestSuite,
	// 		T:            t,
	// 	}
	// 	xcall.TestDemo()
	// })

	t.Run("test relayer", func(t *testing.T) {
		rly, err := s.SetupRelayer(ctx)
		s.Require().NoError(err)
		s.Require().NoError(s.CreateClient(ctx))
		s.Require().NoError(s.CreateConnection(ctx))
		s.Require().NoError(s.StartRelayer(rly))
		relayer := testsuite.RelayerTestSuite{
			E2ETestSuite: &s.E2ETestSuite,
			T:            t,
		}
		relayer.TestRelayer()
	})
}
