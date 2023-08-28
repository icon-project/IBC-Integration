package cosmos

import (
	"context"
	"fmt"
	"testing"
	"time"

	chains "github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func NewCosmosChain(t *testing.T, ctx context.Context, environment string, chainConfig chains.ChainConfig, keystorePath string, keyPassword string, url string, contractPaths map[string]string, logger *zap.Logger, chainID string) (chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		client, network := interchaintest.DockerSetup(t)
		var localchain chains.Chain
		localchain, _ = NewCosmosLocalnet(t.Name(), logger, chainConfig.GetIBCChainConfig(&localchain), chains.DefaultNumValidators, chains.DefaultNumFullNodes, keyPassword, contractPaths)
		ic := interchaintest.NewInterchain().AddChain(localchain.(ibc.Chain))
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
	default:
		return nil, fmt.Errorf("unknown environment: %s", environment)
	}
}
