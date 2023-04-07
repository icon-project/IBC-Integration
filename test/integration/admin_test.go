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
			ctx.Step(`^"([^"]*)" wallet address should not be added as admin$`, executor.walletAddressShouldNotBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" is an admin wallet who needs to be added as admin$`, executor.isAnAdminWalletWhoNeedsToBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" is not the contract owner of the xCall smart contract$`, executor.isNotTheContractOwnerOfTheXCallSmartContract)
			ctx.Step(`^xCall returns an error message that only the contract owner can perform this action$`, executor.xCallReturnsAnErrorMessageThatOnlyTheContractOwnerCanPerformThisAction)
			ctx.Step(`^"([^"]*)" has already added "([^"]*)" wallet address as admin$`, executor.hasAlreadyAddedWalletAddressAsAdmin)
			ctx.Step(`^"([^"]*)" wallet address should still be as admin$`, executor.walletAddressShouldStillBeAsAdmin)
			ctx.Step(`^xCall returns an error message that the admin already exists$`, executor.xCallReturnsAnErrorMessageThatTheAdminAlreadyExists)
			ctx.Step(`^no wallet address should be as admin$`, executor.noWalletAddressShouldBeAsAdmin)
			ctx.Step(`^xCall returns an error message that the null value cannot be added as admin$`, executor.xCallReturnsAnErrorMessageThatTheNullValueCannotBeAddedAsAdmin)
			ctx.Step(`^xCall returns an error message that  wallet address of the new admin is not a valid address$`, executor.xCallReturnsAnErrorMessageThatWalletAddressOfTheNewAdminIsNotAValidAddress)
			ctx.Step(`^"([^"]*)" executes update_admin in xcall with "([^"]*)" wallet address$`, executor.executesUpdate_adminInXcallWithWalletAddress)
			ctx.Step(`^xCall should update xCall admin with "([^"]*)" address$`, executor.xCallShouldUpdateXCallAdminWithAddress)
			ctx.Step(`^"([^"]*)" executes remove_admin in xcall with "([^"]*)" wallet address$`, executor.executesRemove_adminInXcallWithWalletAddress)
			ctx.Step(`^xCall should remove "([^"]*)" wallet address as admin$`, executor.xCallShouldRemoveWalletAddressAsAdmin)

		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/admin.feature"}, TestingT: t, StopOnFailure: false},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
