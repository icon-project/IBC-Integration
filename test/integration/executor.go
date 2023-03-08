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
	ibctest "github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

type Executor struct {
	chain chains.Chain
	*testing.T
	ctx context.Context
	cfg *Config
	logger *zap.Logger
}

func NewExecutor(t *testing.T) *Executor {
	cfg := GetConfig()

	return &Executor{
		T:   t,
		cfg: cfg,
		ctx: context.Background(),
		logger: zaptest.NewLogger(t),
	}
}

func (e *Executor) EnsureChainIsRunning(ctx context.Context) (context.Context, error) {
	var err error

	switch e.cfg.Chain.Name {
	case "icon":
		e.chain, err = icon.NewIconChain(e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig, e.cfg.Chain.NID, e.cfg.KeystoreFile, e.cfg.KeystorePassword, e.cfg.Chain.URL, e.cfg.Contracts, e.logger)
	case "archway":
		e.chain, err = archway.NewArchwayChain(e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig)
	case "cosmos":
		e.chain, err = cosmos.NewCosmosChain(e.cfg.Chain.Environment, e.cfg.Chain.ChainConfig)
	default:
		err = fmt.Errorf("unknown chain: %s", e.cfg.Chain.Name)
	}

	if err != nil {
		return nil, err
	}

	// Check wether chain is running by checking block height

	return ctx, nil
}

//

// "github.com/strangelove-ventures/interchaintest/v6/ibc"
// "go.uber.org/zap/zaptest"
// "github.com/icon-project/ibc-integration/test/chains/icon"

// ic      *ibctest.Interchain
// network string
// client  *client.Client

// "github.com/docker/docker/client"

// Local
/// Based on config create chain ibc.Chain
// cf := ibctest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*ibctest.ChainSpec{
// 	{Name: cfg.Chain.Name, ChainConfig: ibc.ChainConfig{
// 	}},
// })

// client, network := ibctest.DockerSetup(t)
// chains, _ := cf.Chains(t.Name())

// return &Executor{
// 	T:       t,
// 	chain:   chains[0],
// 	ctx:     context.Background(),
// 	Config:  cfg,
// 	ic:      ibctest.NewInterchain().AddChain(chains[0]),
// 	client:  client,
// 	network: network,
// }

// Test

// }

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
