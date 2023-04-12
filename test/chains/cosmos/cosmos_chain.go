package cosmos

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func NewCosmosChain(t *testing.T, ctx context.Context, environment string, chainConfig chains.ChainConfig, keystorePath string, keyPassword string, url string, contractPaths map[string]string, logger *zap.Logger) (chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		client, network := interchaintest.DockerSetup(t)
		localchain, _ := NewCosmosLocalnet(t, logger, chainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, keyPassword, contractPaths)
		ic := interchaintest.NewInterchain().
			AddChain(localchain.(ibc.Chain))
		// Log location
		f, _ := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
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
		return localchain, nil

	case "testnet":
		return NewCosmosTestnet(chainConfig.Bin, keystorePath, keyPassword, "5000000000", url, contractPaths), nil
	default:
		return nil, fmt.Errorf("unknown environment: %s", environment)
	}
}
