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
		e.chain, err = icon.NewIconChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.Chain.NID, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger, e.cfg.InitMessage)
	case "cosmos":
		e.chain, err = cosmos.NewCosmosChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger)
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

	fmt.Printf("Chain is running. Current Chain height: %d \n", ctx.Value(chains.LastBlock{}).(uint64))
	return e.ctx, err
}

func (e *Executor) GetContractAddress(contractName string) string {
	ctxValue := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return ctxValue.ContractAddress[contractName]
}

func (e *Executor) walletAddressShouldBeAddedAsAdmin(admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
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

func (e *Executor) executesAdd_adminInXcallWithWalletAddress(keyName, admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, e.error = e.chain.ExecuteContract(e.ctx, contractAddress, keyName, "set_admin", admin)
	return nil
}

func (e *Executor) isTheContractOwner(owner, contractName string) (err error) {
	// Get contract name from scenario and add to context
	e.ctx = context.WithValue(e.ctx, chains.ContractName{}, chains.ContractName{
		ContractName: contractName,
	})

	// Add init message to context
	initMsg := e.cfg.InitMessage
	e.ctx = context.WithValue(e.ctx, chains.InitMessage{}, chains.InitMessage{
		InitMsg: initMsg,
	})

	// Deploy Contract for Testing
	e.ctx, err = e.chain.DeployContract(e.ctx, owner)
	ctxValue := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	fmt.Printf("\n Contract Addresses of %s : %s \n", contractName, ctxValue.ContractAddress[contractName])
	return err
}

func (e *Executor) nonOwnerOfContractExecutesAdd_adminInXcallWithWalletAddress(nonOwner, admin string) (err error) {
	// Build a wallet for the non owner
	e.chain.BuildWallets(e.ctx, nonOwner)
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.ExecuteContract(e.ctx, contractAddress, nonOwner, "set_admin", admin)
	// Above call should return an error
	if err != nil {
		return nil
	}
	return fmt.Errorf("admin added when non owner executes transaction")
}

func (e *Executor) walletAddressShouldNotBeAddedAsAdmin(admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
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

func (e *Executor) anAdminExecutesAdd_adminInXcallWithWalletAddress(admin1, admin2 string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.ExecuteContract(e.ctx, contractAddress, admin1, "set_admin", admin2)
	if err != nil {
		return nil
	}
	return fmt.Errorf("admin added when another admin executes transaction")
}

func (e *Executor) isAnAdminWalletWhoNeedsToBeAddedToTheListOfXCallAdmins(keyName string) error {
	err := e.chain.BuildWallets(e.ctx, keyName)
	return err
}

func (e *Executor) isNotTheContractOwnerOfTheXCallSmartContract(keyName string) error {
	err := e.chain.BuildWallets(e.ctx, keyName)
	return err
}

func (e *Executor) xCallReturnsAnErrorMessageThatOnlyTheContractOwnerCanPerformThisAction() error {
	if e.error == nil {
		return fmt.Errorf("contract did not return an error message")
	}
	return nil
}

func (e *Executor) hasAlreadyAddedWalletAddressToTheListOfXCallAdmins(keyName, admin string) (err error) {
	err = e.executesAdd_adminInXcallWithWalletAddress(keyName, admin)
	return err
}

func (e *Executor) walletAddressShouldStillBeInTheListOfXCallAdmins(admin string) error {
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

func (e *Executor) noWalletAddressShouldBeInTheListOfXCallAdmins() (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
	return err
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
