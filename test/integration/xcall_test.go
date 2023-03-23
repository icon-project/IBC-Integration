package integration_test

import (
	"testing"

	"github.com/cucumber/godog"
)

func TestXcall(t *testing.T) {
	executor := NewExecutor(t)
	suite := godog.TestSuite{
		Name: "TestXcall",
		TestSuiteInitializer: func(sc *godog.TestSuiteContext) {
			sc.BeforeSuite(func() {
				executor.EnsureChainIsRunningAndContractIsDeployed()
			})
		},
		ScenarioInitializer: func(ctx *godog.ScenarioContext) {
			ctx.Step(`^Admin Address to be added$`, executor.adminAddressToBeAdded)
			ctx.Step(`^Owner adds admin$`, executor.ownerAddsAdmin)
			ctx.Step(`^Admin should be added successfully$`, executor.adminShouldBeAddedSuccessfully)
			ctx.Step(`^Admin should not be added successfully$`, executor.adminShouldNotBeAddedSuccessfully)
			ctx.Step(`^Non Owner adds admin$`, executor.nonOwnerAddsAdmin)
		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/xcall.feature"}, TestingT: t},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
