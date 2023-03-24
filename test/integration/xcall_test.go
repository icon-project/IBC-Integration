package integration_test

import (
	"testing"

	"github.com/cucumber/godog"
)

const XCAll_ADMINS = string("XCAll_ADMINS")

func (e *Executor) isAUserWhoNeedsToBeAddedToTheListOfXCallAdmins(user string) error {
	// Create a wallet and set to state


	return nil
}

func (e *Executor) userExecutesAddAdminInXCallWithWalletAddress(user, admin string) error {


	return nil
}

func TestXCall(t *testing.T) {
	t.Run("Admin management", func(f *testing.T) {
		executor := NewExecutor(f)

		suite := godog.TestSuite{
			Name: "Admin management",
			TestSuiteInitializer: func(sc *godog.TestSuiteContext) {
				sc.BeforeSuite(func() {
					executor.EnsureChainIsRunningAndContractIsDeployed()
				})
			},
			ScenarioInitializer: func(ctx *godog.ScenarioContext) {
				ctx.Step(`^"(xCall)" contract is deployed and initialized$`, executor.contractIsDeployedAndInitialized)
				ctx.Step(`^(\w+) is the (xCall) contract owner$`, executor.isTheContractOwner)
				ctx.Step(`^(\w+) is a user who needs to be added to the list of xCall admins$`, executor.isAUserWhoNeedsToBeAddedToTheListOfXCallAdmins)
				ctx.Step(`^(\w+) executes add_admin in xCall with (\w+)'s wallet address$`, executor.userExecutesAddAdminInXCallWithWalletAddress)
				ctx.Step(`^(\w+)'s wallet address should be added to the list of xCall admins$`, executor.adminShouldNotBeAddedSuccessfully)
				ctx.Step(`^(\w+) is an existing admin wallet in the list of xCall admins$`, executor.adminShouldNotBeAddedSuccessfully)
				ctx.Step(`^(\w+)'s wallet address should still be in the list of xCall admins$`, executor.adminShouldNotBeAddedSuccessfully)
				ctx.Step(`^no new entry should be created in the list of xCall admins$`, executor.adminShouldNotBeAddedSuccessfully)
			},
			Options: &godog.Options{Format: "pretty", Paths: []string{"features/xCall/admin_management.feature"}, TestingT: f},
		}

		if suite.Run() != 0 {
			f.Fatal("non-zero status returned, failed to run admin management feature tests")
		}
	})
}
