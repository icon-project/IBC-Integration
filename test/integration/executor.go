package integration
package integration

import (
	"context"
	"fmt"
	"strings"
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

func (e *Executor) walletAddressShouldBeAddedToTheListOfXCallAdmins(admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, _ = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
	ctxValue := e.ctx.Value(chains.AdminKey("Admins")).(chains.Admins)

	// Test if the address of the given key is present in the response
	if strings.Contains(fmt.Sprint(chains.Response), ctxValue.Admin[admin]) {
		return nil
	} else {
		return fmt.Errorf("given key is not added to admin list")
	}
}

func (e *Executor) executesAdd_adminInXcallWithWalletAddress(owner, admin string) (err error) {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, err = e.chain.ExecuteContract(e.ctx, contractAddress, owner, "set_admin", admin)
	return err
}

func (e *Executor) isAnAdminWalletWhoNeedsToBeAddedToTheListOfXCallAdmins(keyName string) error {
	return nil
}

func (e *Executor) isTheContractOwner(owner, contractName string) (err error) {
	// Check if there is already an existing contract owner
	ctxValue, ok := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	if !ok {
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
		fmt.Printf("\n Contract Addresses of %s : %s \n", contractName, ctxValue.ContractAddress[contractName])
		return err
	}
	return nil
}

func (e *Executor) EnsureChainIsRunning() (context.Context, error) {
	var err error
	switch e.cfg.Chain.ChainConfig.Type {
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
	ctx, _ := e.chain.GetLastBlock(e.ctx)
	fmt.Printf("Chain is running. Current Chain height: %d \n", ctx.Value(chains.LastBlock{}).(uint64))
	// To Make sure that chain is running
	ctx, _ := e.chain.GetLastBlock(e.ctx)
	fmt.Printf("Chain is running. Current Chain height: %d \n", ctx.Value(chains.LastBlock{}).(uint64))
	return e.ctx, nil
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

func (e *Executor) walletAddressShouldNotBeAddedToTheListOfXCallAdmins(admin string) error {
	contractAddress := e.GetContractAddress("xcall")
	e.ctx, _ = e.chain.QueryContract(e.ctx, contractAddress, "get_admin", "")
	ctxValue := e.ctx.Value(chains.AdminKey("Admins")).(chains.Admins)
	if strings.Contains(fmt.Sprint(chains.Response), ctxValue.Admin[admin]) {
		return fmt.Errorf("given key is added to admin list, Non Owner should not be able to add admin")
	} else {
		return nil
	}
}

func (e *Executor) GetContractAddress(contractName string) string {
	ctxValue := e.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return ctxValue.ContractAddress[contractName]
}
