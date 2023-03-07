package integration_test

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/docker/docker/client"
	ibctest "github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"github.com/strangelove-ventures/interchaintest/v6/testreporter"
	"github.com/stretchr/testify/require"
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

func (e *Executor) ChainRunning() error {
	exec := NewExecutor(e.T)
	// cf := ibctest.NewBuiltinChainFactory(zaptest.NewLogger(e.T), chains)
	// if cf == nil {
	// 	return fmt.Errorf("chain factory failed")
	// }
	// chains, _ := cf.Chains(e.T.Name())
	cut := exec.chain
	// client, network := ibctest.DockerSetup(e.T)
	ic := ibctest.NewInterchain().
		AddChain(cut)
	// Log location
	f, err := ibctest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	require.NoError(e.T, err)
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	eRep := rep.RelayerExecReporter(e.T)

	// Build interchain
	require.NoError(e.T, ic.Build(e.ctx, eRep, ibctest.InterchainBuildOptions{
		TestName:          e.T.Name(),
		Client:            exec.client,
		NetworkID:         exec.network,
		BlockDatabaseFile: ibctest.DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	),
	)
	return nil

}

func (e *Executor) weDeploySmartContractOnChain() error {
	users := ibctest.GetAndFundTestUsers(e.T, e.ctx, "default", int64(100_000_000), e.chain)
	cut := users[0]
	balance, _ := e.chain.GetBalance(e.ctx, cut.FormattedAddress(), e.chain.Config().Denom)
	fmt.Println(balance, e.chain.Config().Denom)
	/*
		Next step, Need to be generic
		May be we can use switch case based on cofig we can do type conversion ?
	*/
	osmosis := e.chain.(*cosmos.CosmosChain)
	keyName := cut.KeyName()
	codeId, err := osmosis.StoreContract(e.ctx, keyName, "/home/dell/practice/ibc-bdd/ibctest/godogs/cw_tpl_osmosis.wasm")
	if err != nil {
		return fmt.Errorf("error storing: %v", err)
	}
	fmt.Println(codeId)
	return nil
}

func (e *Executor) contractShouldBeDeployedOnChain() {

}
