package integration_test

import (
	"context"
	"testing"

	"github.com/cucumber/godog"
)

func TestSmartContract(t *testing.T) {
	executor := NewExecutor(t)
	suite := godog.TestSuite{
		Name: "TestSmartContract",
		ScenarioInitializer: func(ctx *godog.ScenarioContext) {
			ctx.Before(func(ctx context.Context, sc *godog.Scenario) (context.Context, error) {
				return executor.EnsureChainIsRunning()
			})

			// ctx.Step(`^Contract should be deployed on chain$`, executor.contractShouldBeDeployedOnChain)
			// ctx.Step(`^we Deploy SmartContract on chain$`, executor.weDeploySmartContractOnChain)
		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/smartcontract.feature"}, TestingT: t},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
