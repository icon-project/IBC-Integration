package e2e_demo

import (
	"context"
	"fmt"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/suite"
	"testing"
)

func TestE2EDemo(t *testing.T) {
	suite.Run(t, new(E2EDemoSuite))
}

type E2EDemoSuite struct {
	testsuite.E2ETestSuite
}

func (e *E2EDemoSuite) TestSetup() {
	t := e.T()
	ctx := context.TODO()
	var err error

	e.Require().NoError(e.SetCfg())
	_ = e.SetupChainsAndRelayer(ctx)

	portId := "transfer"
	defer func(_err *error) {
		if *_err != nil {
			fmt.Println("cleaning up...")
			interchaintest.CleanDockerSetup(e.T(), "TestE2EDemo/TestSetup")
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

func (e *E2EDemoSuite) TestCleanup() {
	interchaintest.CleanDockerSetup(e.T(), "TestE2EDemo/TestSetup")
	interchaintest.CleanBackupConfig()
}
