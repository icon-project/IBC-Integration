package cosmos

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"github.com/strangelove-ventures/interchaintest/v6/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func NewCosmosChain(t *testing.T, ctx context.Context, environment string, chainConfig chains.ChainConfig, keystorePath string, keyPassword string, url string, scorePaths map[string]string, logger *zap.Logger) (context.Context, chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		// cf := interchaintest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*interchaintest.ChainSpec{
		// 	// Source chain
		// 	{Name: "gaia", Version: "v7.0.0", ChainConfig: ibc.ChainConfig{
		// 		GasPrices: "0.0uatom",
		// 	},
		// 	},
		// },
		// )
		// chains, _ := cf.Chains(t.Name())
		// dest := chains[0]
		client, network := interchaintest.DockerSetup(t)
		chain, _ := NewCosmosLocalnet(t.Name(), logger, chainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes)
		ic := interchaintest.NewInterchain().
			AddChain(chain.(ibc.Chain))
		// Log location
		f, _ := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
		// if err != nil {
		// 	return ctx, chain, nil
		// }
		// Reporter/logs
		rep := testreporter.NewReporter(f)
		eRep := rep.RelayerExecReporter(t)

		// Build interchain
		require.NoError(t, ic.Build(ctx, eRep, interchaintest.InterchainBuildOptions{
			TestName:          t.Name(),
			Client:            client,
			NetworkID:         network,
			BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),

			SkipPathCreation: false},
		),
		)

		return context.WithValue(ctx, "ibc.chain", chain.(ibc.Chain)), chain, nil

	case "testnet":
	default:
		return nil, nil, fmt.Errorf("unknown environment: %s", environment)
	}

	return nil, nil, nil
}
