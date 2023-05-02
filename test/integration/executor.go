package integration

import (
	"context"
	"fmt"
	"strings"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/cosmos"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

type Executor struct {
	chain chains.Chain
	*testing.T
	ctx    context.Context
	cfg    *Config
	logger *zap.Logger
	error
}

const (
	SET_ADMIN    string = "set_admin"
	UPDATE_ADMIN string = "update_admin"
	REMOVE_ADMIN string = "remove_admin"
	GET_ADMIN    string = "get_admin"
)

func NewExecutor(t *testing.T) *Executor {
	cfg := GetConfig()

	return &Executor{
		T:      t,
		cfg:    cfg,
		ctx:    context.Background(),
		logger: zaptest.NewLogger(t),
	}
}

func (e *Executor) EnsureChainIsRunning() (context.Context, error) {
	var err error
	switch e.cfg.Chain.ChainConfig.Type {
	case "icon":
		e.chain, err = icon.NewIconChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.Chain.NID, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger)
	case "cosmos":
		e.chain, err = cosmos.NewCosmosChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger, e.cfg.Chain.NID)
	default:
		err = fmt.Errorf("unknown chain: %s", e.cfg.Chain.Name)
	}

	if err != nil {
		return nil, err
	}

	// To Make sure that chain is running
	ctx, err := e.chain.GetLastBlock(e.ctx)
	if err != nil {
		return nil, err
	}

	fmt.Printf("%s Chain is running. Current Chain height: %d \n", e.cfg.Chain.ChainConfig.Name, ctx.Value(chains.LastBlock{}).(uint64))
	return e.ctx, err
}

func (e *Executor) GetContractAddress(contractName string) string {
	ctxValue := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return ctxValue.ContractAddress[contractName]
}

func (e *Executor) walletAddressShouldBeAddedAsAdmin(admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, GET_ADMIN, "")
	if err != nil {
		return err
	}
	ctxValue := e.ctx.Value(chains.AdminKey("Admins")).(chains.Admins)

	// Test if the address of the given key is present in the response
	if strings.Contains(fmt.Sprint(chains.Response), ctxValue.Admin[admin]) {
		return nil
	} else {
		return fmt.Errorf("given key is not added to admin list")
	}
}

func (e *Executor) executesSet_adminInXcallWithWalletAddress(keyName, admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, SET_ADMIN, admin)
	return nil
}

func (e *Executor) isTheContractOwner(owner, contractName string) (err error) {
	// Get contract name from scenario and add to context
	e.ctx = context.WithValue(e.ctx, chains.ContractName{}, chains.ContractName{
		ContractName: contractName,
	})

	// Add init message from config to context
	contractName = strings.ToLower(contractName)
	initMsg := e.cfg.InitMessage[contractName]
	e.ctx = context.WithValue(e.ctx, chains.InitMessage{}, chains.InitMessage{
		InitMsg: initMsg,
	})

	// Deploy Contract for Testing
	e.ctx, err = e.chain.DeployContract(e.ctx, owner)
	ctxValue := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	fmt.Printf("\n Contract Addresses of %s : %s \n", contractName, ctxValue.ContractAddress[contractName])
	return err
}

func (e *Executor) walletAddressShouldNotBeAddedAsAdmin(admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, GET_ADMIN, "")
	if err != nil {
		return err
	}
	ctxValue := e.ctx.Value(chains.AdminKey("Admins")).(chains.Admins)
	if strings.Contains(fmt.Sprint(chains.Response), ctxValue.Admin[admin]) {
		return fmt.Errorf("given key is added to admin list, Non Owner should not be able to add admin")
	} else {
		return nil
	}
}

func (e *Executor) isAnAdminWalletWhoNeedsToBeAddedAsAdmin(keyName string) error {
	return e.chain.BuildWallets(e.ctx, keyName)
}

func (e *Executor) isNotTheContractOwnerOfTheXCallSmartContract(keyName string) error {
	return e.chain.BuildWallets(e.ctx, keyName)
}

func (e *Executor) xCallReturnsAnErrorMessageThatOnlyTheContractOwnerCanPerformThisAction() error {
	if e.error == nil {
		return fmt.Errorf("contract did not return an error message")
	}
	return nil
}

func (e *Executor) hasAlreadyAddedWalletAddressAsAdmin(keyName, admin string) (err error) {
	return e.executesSet_adminInXcallWithWalletAddress(keyName, admin)
}

func (e *Executor) walletAddressShouldStillBeAsAdmin(admin string) error {
	err := e.walletAddressShouldBeAddedAsAdmin(admin)
	if err != nil {
		return fmt.Errorf("existing admin list is modified")
	}
	return nil
}

func (e *Executor) xCallReturnsAnErrorMessageThatTheAdminAlreadyExists() error {
	if e.error == nil {
		return fmt.Errorf("contract did not return an error message that admin already exists")
	}
	return nil
}

func (e *Executor) byDefaultContractOwnerAddressShouldBeAsAdmin(owner string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
	if err != nil {
		return err
	}
	ctxVal := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	if strings.Contains(fmt.Sprint(chains.Response), ctxVal.ContractOwner[owner]) {
		return nil
	}
	return fmt.Errorf("by Default owner of contract address is not set as admin")
}

func (e *Executor) xCallReturnsAnErrorMessageThatTheNullValueCannotBeAddedAsAdmin() error {
	if e.error == nil {
		return fmt.Errorf("no Error message was returned when adding null value as admin")
	}
	return nil
}

func (e *Executor) xCallReturnsAnErrorMessageThatWalletAddressOfTheNewAdminIsNotAValidAddress() error {
	if e.error == nil {
		return fmt.Errorf("no Error message was returned when adding junk address as admin")
	}
	return nil
}

func (e *Executor) executesUpdate_adminInXcallWithWalletAddress(keyName, admin string) error {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, UPDATE_ADMIN, admin)
	return nil
}

func (e *Executor) xCallShouldUpdateXCallAdminWithAddress(admin string) error {
	if e.error != nil {
		return e.error
	} else {
		return e.walletAddressShouldBeAddedAsAdmin(admin)
	}

}

func (e *Executor) executesRemove_adminInXcall(keyName string) error {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, REMOVE_ADMIN, "")
	return nil
}

func (e *Executor) xCallShouldRemoveWalletAddressAsAdmin(admin string) error {
	if e.error != nil {
		return e.error
	} else {
		err := e.walletAddressShouldNotBeAddedAsAdmin(admin)
		if err == nil {
			return fmt.Errorf("admin is not removed ")
		}
		return nil
	}
}

func (e *Executor) xCallReturnsAnErrorMessageThatAdminIsAlreadySet() error {
	if e.error == nil {
		return fmt.Errorf("owner was able to set admin twice")
	}
	return nil
}

func (e *Executor) thereAreNoAdminWalletsAddedAsAdmin() (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, GET_ADMIN, "")
	if err != nil {
		return nil
	}
	return err
}

func (e *Executor) xCallReturnsAnErrorMessageThatThereAreNoAdminWalletsAddedToTheXCallSmartContract() error {
	if e.error == nil {
		return fmt.Errorf("owner was able to update admin even though admin was not set initially")
	}
	return nil
}

func (e *Executor) contractDeployedByOnlyWhenTheChainIs(contractName, owner, chainName string) error {
	if e.cfg.Chain.ChainConfig.Type == chainName {
		return e.isTheContractOwner(owner, strings.ToLower(contractName))
	}
	fmt.Println("Given chain is not Icon, so deploying BMC contract is not required")
	return nil
}

func (e *Executor) aUserQueryForAdmin() error {
	return nil
}

func (e *Executor) walletAddressShouldBeAsAdmin(admin string) error {
	return e.walletAddressShouldBeAddedAsAdmin(admin)
}

func (e *Executor) shouldOpenChannelToSendAndReceiveMessages(keyName string) error {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, "ibc_packet_receive", "")
	return nil
}

func (e *Executor) contractThrowsAnErrorThatOnlyTheContractCanPerformThisAction(arg1 string) error {
	if e.error == nil {
		return fmt.Errorf("Non contract was able to perform send call message when roll back was not null")
	}
	return nil
}

func (e *Executor) nonContractExecutesInXcall(keyName, methodaName string) error {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, methodaName, "")
	return nil
}
