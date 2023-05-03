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
			ctx.Step(`^"([^"]*)" executes set_admin in xcall with "([^"]*)" wallet address$`, executor.executesSet_adminInXcallWithWalletAddress)
			ctx.Step(`^"([^"]*)" is the "([^"]*)" contract owner$`, executor.isTheContractOwner)
			ctx.Step(`^"([^"]*)" wallet address should be added as admin$`, executor.walletAddressShouldBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" wallet address should not be added as admin$`, executor.walletAddressShouldNotBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" is an admin wallet who needs to be added as admin$`, executor.isAnAdminWalletWhoNeedsToBeAddedAsAdmin)
			ctx.Step(`^"([^"]*)" is not the contract owner of the xCall smart contract$`, executor.isNotTheContractOwnerOfTheXCallSmartContract)
			ctx.Step(`^xCall returns an error message that only the contract owner can perform this action$`, executor.xCallReturnsAnErrorMessageThatOnlyTheContractOwnerCanPerformThisAction)
			ctx.Step(`^"([^"]*)" has already added "([^"]*)" wallet address as admin$`, executor.hasAlreadyAddedWalletAddressAsAdmin)
			ctx.Step(`^"([^"]*)" wallet address should still be as admin$`, executor.walletAddressShouldStillBeAsAdmin)
			ctx.Step(`^xCall returns an error message that the admin already exists$`, executor.xCallReturnsAnErrorMessageThatTheAdminAlreadyExists)
			ctx.Step(`^by default "([^"]*)" contract owner address should be as admin$`, executor.byDefaultContractOwnerAddressShouldBeAsAdmin)
			ctx.Step(`^xCall returns an error message that the null value cannot be added as admin$`, executor.xCallReturnsAnErrorMessageThatTheNullValueCannotBeAddedAsAdmin)
			ctx.Step(`^xCall returns an error message that  wallet address of the new admin is not a valid address$`, executor.xCallReturnsAnErrorMessageThatWalletAddressOfTheNewAdminIsNotAValidAddress)
			ctx.Step(`^"([^"]*)" executes update_admin in xcall with "([^"]*)" wallet address$`, executor.executesUpdate_adminInXcallWithWalletAddress)
			ctx.Step(`^xCall should update xCall admin with "([^"]*)" address$`, executor.xCallShouldUpdateXCallAdminWithAddress)
			ctx.Step(`^"([^"]*)" executes remove_admin in xcall$`, executor.executesRemove_adminInXcall)
			ctx.Step(`^xCall should remove "([^"]*)" wallet address as admin$`, executor.xCallShouldRemoveWalletAddressAsAdmin)
			ctx.Step(`^xCall returns an error message that admin is already set$`, executor.xCallReturnsAnErrorMessageThatAdminIsAlreadySet)
			ctx.Step(`^there are no admin wallets added as admin$`, executor.thereAreNoAdminWalletsAddedAsAdmin)
			ctx.Step(`^xCall returns an error message that there are no admin wallets added to the xCall smart contract$`, executor.xCallReturnsAnErrorMessageThatThereAreNoAdminWalletsAddedToTheXCallSmartContract)
			ctx.Step(`^"([^"]*)" contract deployed by "([^"]*)" only when the chain is "([^"]*)"$`, executor.contractDeployedByOnlyWhenTheChainIs)
			ctx.Step(`^a user query for admin$`, executor.aUserQueryForAdmin)
			ctx.Step(`^"([^"]*)" wallet address should be as admin$`, executor.walletAddressShouldBeAsAdmin)
			ctx.Step(`^"([^"]*)" should open channel to send and receive messages$`, executor.shouldOpenChannelToSendAndReceiveMessages)
			ctx.Step(`^"([^"]*)" contract throws an error that only the contract can perform this action$`, executor.contractThrowsAnErrorThatOnlyTheContractCanPerformThisAction)
			ctx.Step(`^"([^"]*)" non contract executes "([^"]*)" in xcall$`, executor.nonContractExecutesInXcall)
			ctx.Step(`^"([^"]*)" executes "([^"]*)" in dapp with "([^"]*)" than limit$`, executor.executesInDappWithThanLimit)
			ctx.Step(`^xcall contract panic with an error MaxDataSizeExceeded$`, executor.xcallContractPanicWithAnErrorMaxDataSizeExceeded)
			ctx.Step(`^"([^"]*)" executes "([^"]*)" in dapp with "([^"]*)" to limit$`, executor.executesInDappWithToLimit)
			ctx.Step(`^"([^"]*)" executes "([^"]*)" in "([^"]*)" with "([^"]*)" request ID$`, executor.executesInWithRequestID)
			ctx.Step(`^xcall contract panic with an error RequestNotFound$`, executor.xcallContractPanicWithAnErrorRequestNotFound)
			ctx.Step(`^xcall should execute call message successfully$`, executor.xcallShouldExecuteCallMessageSuccessfully)
			ctx.Step(`^xcall contract should emit a event with sequence id and request id$`, executor.xcallContractShouldEmitAEventWithSequenceIdAndRequestId)

		},
		Options: &godog.Options{Format: "pretty", Paths: []string{"features/xcall/messaging.feature"}, TestingT: t, StopOnFailure: false},
	}

	if suite.Run() != 0 {
		t.Fatal("non-zero status returned, failed to run feature tests")
	}
}
