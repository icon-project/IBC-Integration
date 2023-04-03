package integration

import (
	"testing"

	"github.com/cucumber/godog"
)

func TestAdmin(t *testing.T) {
	executor := NewExecutor(t)
	suite := godog.TestSuite{
		Name: "TestXcall",
		TestSuiteInitializer: func(sc *godog.TestSuiteContext) {
			sc.BeforeSuite(func() {
				executor.EnsureChainIsRunning()
			})
		},
		ScenarioInitializer: func(ctx *godog.ScenarioContext) {
			ctx.Step(`^"([^"]*)" executes add_admin in xcall with "([^"]*)" wallet address$`, executor.executesAdd_adminInXcallWithWalletAddress)
			ctx.Step(`^"([^"]*)" is an admin wallet who needs to be added to the list of xCall admins$`, executor.isAnAdminWalletWhoNeedsToBeAddedToTheListOfXCallAdmins)
			ctx.Step(`^"([^"]*)" is the "([^"]*)" contract owner$`, executor.isTheContractOwner)
			ctx.Step(`^"([^"]*)" wallet address should be added to the list of xCall admins$`, executor.walletAddressShouldBeAddedToTheListOfXCallAdmins)
		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/admin.feature"}, TestingT: t, StopOnFailure: false},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
