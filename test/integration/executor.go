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

	// Local
	/// Based on config create chain ibc.Chain
	cf := ibctest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*ibctest.ChainSpec{
		{Name: cfg.Chain.Name, ChainConfig: ibc.ChainConfig{
			Type:    cfg.Chain.ChainConfig.Type,
			Name:    cfg.Chain.ChainConfig.Name,
			ChainID: cfg.Chain.ChainConfig.ChainID,
			Images: []ibc.DockerImage{{
				Repository: cfg.Chain.ChainConfig.Images.Repository,
				Version:    cfg.Chain.ChainConfig.Images.Version,
				UidGid:     cfg.Chain.ChainConfig.Images.UidGid,
			}},
			Bin:            cfg.Chain.ChainConfig.Bin,
			Bech32Prefix:   cfg.Chain.ChainConfig.Bech32Prefix,
			Denom:          cfg.Chain.ChainConfig.Denom,
			CoinType:       cfg.Chain.ChainConfig.CoinType,
			GasPrices:      cfg.Chain.ChainConfig.GasPrices,
			GasAdjustment:  cfg.Chain.ChainConfig.GasAdjustment,
			TrustingPeriod: cfg.Chain.ChainConfig.TrustingPeriod,
			NoHostMount:    cfg.Chain.ChainConfig.NoHostMount,
		}},
	})

	client, network := ibctest.DockerSetup(t)
	chains, _ := cf.Chains(t.Name())

	return &Executor{
		T:       t,
		chain:   chains[0],
		ctx:     context.Background(),
		Config:  cfg,
		ic:      ibctest.NewInterchain().AddChain(chains[0]),
		client:  client,
		network: network,
	}


	// Test

}

func (e *Executor) ChainRunning() {
	
}

func (e *Executor) contractShouldBeDeployedOnChain() {

}

func (e *Executor) weDeploySmartContractOnChain() {

}
