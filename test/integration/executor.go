package integration_test

import (
	"context"
	"testing"

	"github.com/docker/docker/client"
	ibctest "github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"go.uber.org/zap/zaptest"
)

type Executor struct {
	chain ibc.Chain
	*testing.T
	ic      *ibctest.Interchain
	network string
	client  *client.Client
	ctx     context.Context
	*Config
}

func NewExecutor(t *testing.T) *Executor {
	cfg := GetConfig()

	
	/// Based on config create chain ibc.Chain
	cf := ibctest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*ibctest.ChainSpec{
		{Name: cfg.Chain.Name, ChainConfig: ibc.ChainConfig{}},
	})

	chains, _ := cf.Chains(t.Name())

	return &Executor{
		T:      t,
		chain:  chains[0],
		ctx:    context.Background(),
		Config: cfg,
	}
}

func (e *Executor) ChainRunning() {

}

func (e *Executor) contractShouldBeDeployedOnChain() {

}

func (e *Executor) weDeploySmartContractOnChain() {

}
