package e2e_demo

import (
	"context"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
	"testing"
)

func TestE2EDemoSetup(t *testing.T) {
	suite.Run(t, new(E2EDemoTest))
}

type E2EDemoTest struct {
	testsuite.E2ETestSuite
}

func (e *E2EDemoTest) Name() string {
	return "TestE2EDemo"
}

func (e *E2EDemoTest) TestSetupE2EDemo() {
	t := e.T()
	ctx := context.TODO()
	var err error

	e.Require().NoError(e.SetCfg())
	_ = e.SetupChainsAndRelayer(ctx)

	portId := "transfer"
	defer func(_err *error) {
		if _err != nil {
			interchaintest.CleanDockerSetup(e.T(), "TestE2EDemoSetup/TestSetupE2EDemo")
		}
	}(&err)
	ctx = context.WithValue(ctx, "testcase", "demo")
	err = e.SetupXCall(ctx, portId, 100)
	assert.NoErrorf(t, err, "fail to setup xcall -%w", err)
	err = e.DeployXCallMockApp(ctx, portId)
	assert.NoErrorf(t, err, "fail to deploy xcall dapp -%w", err)
	chainA, chainB := e.GetChains()

	err = interchaintest.BackupConfig(chainA)
	assert.NoErrorf(t, err, "fail to backup xcall config for chainA -%w", err)
	err = interchaintest.BackupConfig(chainB)
	assert.NoErrorf(t, err, "fail to backup xcall config for chainB -%w", err)

}

func (e *E2EDemoTest) TestCleanupE2EDemo() {
	interchaintest.CleanDockerSetup(e.T(), "TestE2EDemoSetup/TestSetupE2EDemo")
	interchaintest.CleanBackupConfig()
}
