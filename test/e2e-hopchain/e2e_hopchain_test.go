package e2e_hopchain

import (
	"context"
	"testing"

	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/stretchr/testify/suite"
)

func TestE2EHopchainTestSuite(t *testing.T) {
	suite.Run(t, new(E2EHopchainTest))
}

type E2EHopchainTest struct {
	testsuite.E2ETestSuite
}

func (s *E2EHopchainTest) TestE2E_hopchain() {

	t := s.T()
	ctx := context.TODO()
	s.Require().NoError(s.SetCfg())
	relayer := s.SetupICS20ChainsAndRelayer(ctx)
	hopchain := HopchainTestSuite{
		E2ETestSuite: &s.E2ETestSuite,
		T:            t,
	}
	t.Run("test hopchain", func(t *testing.T) {
		hopchain.TestICS20(relayer)
	})

}
