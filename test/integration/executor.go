package integration_test

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/cucumber/godog"
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
	state  State
}

func NewExecutor(t *testing.T) *Executor {
	cfg := GetConfig()

	return &Executor{
		T:      t,
		cfg:    cfg,
		ctx:    context.Background(),
		logger: zaptest.NewLogger(t),
		state: NewState(),
	}
}

func (e *Executor) EnsureChainIsRunningAndContractIsDeployed() (context.Context, error) {
	var err error
	switch e.cfg.Chain.Name {
	case "icon":
		e.chain, err = icon.NewIconChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.Chain.NID, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger, e.cfg.InitMessage)
	case "cosmos", "archway":
		e.chain, err = cosmos.NewCosmosChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger)
	default:
		err = fmt.Errorf("unknown chain: %s", e.cfg.Chain.Name)
	}

	if err != nil {
		return nil, err
	}

	// Wait for at least one block to complete
	time.Sleep(time.Second * 1)
	ctx, _ := e.chain.GetLastBlock(e.ctx)
	fmt.Printf("Chain is running. Current Chain height: %d", ctx.Value(chains.LastBlock{}).(uint64))

	// Deploy Contract for Testing
	e.ctx, _ = e.chain.DeployContract(e.ctx)
	ctxValue := e.ctx.Value(chains.ContractKey{}).(chains.ContractKey)
	fmt.Printf("\n Contract Addresses: %s", ctxValue.ContractAddress)
	return e.ctx, nil
}

func (e *Executor) isTheContractOwner(user, contract string) error {
	owner := e.state.Get(fmt.Sprintf("%s.%s", CONTRACT_OWNERS, contract))
	if owner == nil {
		return fmt.Errorf("%s.%s not found in state", WALLETS, user)
	}

	e.state.Set(fmt.Sprintf("%s.%s", WALLETS, user), owner)

	return nil
}

func (e *Executor) contractIsDeployedAndInitialized(contract string) error {
	var err error

	e.ctx, err = e.chain.DeployContract(e.ctx, contract)
	if err != nil {
		return err
	}

	return nil
}

func (e *Executor) adminAddressToBeAdded(param *godog.DocString) error {
	e.ctx = context.WithValue(e.ctx, chains.Param{}, chains.Param{
		Data: param.Content,
	})
	return nil
}

func (e *Executor) ownerAddsAdmin() error {
	// Execute Contract
	var err error
	e.ctx, err = e.chain.ExecuteContract(e.ctx)
	return err
}

func (e *Executor) adminShouldBeAddedSuccessfully(param *godog.DocString) error {
	e.ctx = context.WithValue(e.ctx, chains.Query{}, chains.Query{
		Query: param.Content,
	})

	// Wait for few blocks and Query contract
	time.Sleep(time.Second * 2)
	var err error
	e.ctx, err = e.chain.QueryContract(e.ctx)
	return err
}

func (e *Executor) adminShouldNotBeAddedSuccessfully() error {
	return godog.ErrPending
}

func (e *Executor) nonOwnerAddsAdmin() error {
	/*
		Add non owner to context
		In execute contract method based on owner or non owner execute contract by passing respective keys
	*/
	return godog.ErrPending
}
