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
			ctx.Step(`^"([^"]*)" is the "([^"]*)" contract owner$`, executor.isTheContractOwner)
			ctx.Step(`^"([^"]*)" wallet address should be added as admin$`, executor.walletAddressShouldBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" non owner of contract executes add_admin in xcall with "([^"]*)" wallet address$`, executor.nonOwnerOfContractExecutesAdd_adminInXcallWithWalletAddress)
			ctx.Step(`^"([^"]*)" wallet address should not be added as admin$`, executor.walletAddressShouldNotBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" an admin executes add_admin in xcall with "([^"]*)" wallet address$`, executor.anAdminExecutesAdd_adminInXcallWithWalletAddress)
			ctx.Step(`^"([^"]*)" is an admin wallet who needs to be added to the list of xCall admins$`, executor.isAnAdminWalletWhoNeedsToBeAddedToTheListOfXCallAdmins)
			ctx.Step(`^"([^"]*)" is not the contract owner of the xCall smart contract$`, executor.isNotTheContractOwnerOfTheXCallSmartContract)
			ctx.Step(`^xCall returns an error message that only the contract owner can perform this action$`, executor.xCallReturnsAnErrorMessageThatOnlyTheContractOwnerCanPerformThisAction)
			ctx.Step(`^"([^"]*)" has already added "([^"]*)" wallet address to the list of xCall admins$`, executor.hasAlreadyAddedWalletAddressToTheListOfXCallAdmins)
			ctx.Step(`^"([^"]*)" wallet address should still be in the list of xCall admins$`, executor.walletAddressShouldStillBeInTheListOfXCallAdmins)
			ctx.Step(`^xCall returns an error message that the admin already exists$`, executor.xCallReturnsAnErrorMessageThatTheAdminAlreadyExists)
			ctx.Step(`^no wallet address should be in the list of xCall admins$`, executor.noWalletAddressShouldBeInTheListOfXCallAdmins)
			ctx.Step(`^xCall returns an error message that the null value cannot be added as admin$`, executor.xCallReturnsAnErrorMessageThatTheNullValueCannotBeAddedAsAdmin)
			ctx.Step(`^xCall returns an error message that  wallet address of the new admin is not a valid address$`, executor.xCallReturnsAnErrorMessageThatWalletAddressOfTheNewAdminIsNotAValidAddress)

			ctx.Step(`^"([^"]*)" executes update_admin in xcall with "([^"]*)" wallet address$`, executor.executesUpdate_adminInXcallWithWalletAddress)
			ctx.Step(`^xCall should update xCall admins, replacing Bob\'s address with "([^"]*)" address$`, executor.xCallShouldUpdateXCallAdminsReplacingBobsAddressWithAddress)

		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/admin.feature"}, TestingT: t, StopOnFailure: false},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
