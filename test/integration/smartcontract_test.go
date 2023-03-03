package integration_test

import (
	"testing"

	"github.com/cucumber/godog"
)

func TestSmartContract(t *testing.T) {
	executor := NewExecutor(t)
	suite := godog.TestSuite{
		Name: "TestSmartContract",
		ScenarioInitializer: func(ctx *godog.ScenarioContext) {
			ctx.Step(`^Contract should be deployed on chain$`, executor.contractShouldBeDeployedOnChain)
			ctx.Step(`^Chain running$`, executor.ChainRunning)
			ctx.Step(`^we Deploy SmartContract on chain$`, executor.weDeploySmartContractOnChain)
		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/smartcontract.feature"}, TestingT: t},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
