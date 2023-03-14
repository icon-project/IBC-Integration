package integration_test

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/archway"
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

func (e *Executor) EnsureChainIsRunning() (context.Context, error) {
	var err error
	switch e.cfg.Chain.Name {
	case "icon":
		e.chain, err = icon.NewIconChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.Chain.NID, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger, e.cfg.InitMessage)
	case "archway":
		e.chain, err = archway.NewArchwayChain(e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig)
	case "cosmos":
		e.chain, err = cosmos.NewCosmosChain(e.T, e.ctx, e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger)
	default:
		err = fmt.Errorf("unknown chain: %s", e.cfg.Chain.Name)
	}

	if err != nil {
		return nil, err
	}

	return e.ctx, nil
}

func (e *Executor) chainRunning() error {
	// Wait for at least one block to complete
	time.Sleep(time.Second * 1)
	ctx, _ := e.chain.GetLastBlock(e.ctx)
	fmt.Printf("Chain is running. Current Chain height: %d", ctx.Value(chains.LastBlock{}).(uint64))

	ctx, _ = e.chain.DeployContract(e.ctx)
	ctxValue := ctx.Value(chains.ContractKey{}).(chains.ContractKey)
	fmt.Printf("\n Score Addresses: %s", ctxValue.ContractAddress)
	return nil
}
