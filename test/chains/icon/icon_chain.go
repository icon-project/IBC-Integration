package icon

import (
	"context"
	"fmt"
	"testing"
	"time"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func NewIconChain(t *testing.T, ctx context.Context, environment string, chainConfig chains.ChainConfig, nid string, keystorePath string, keyPassword string, url string, scorePaths map[string]string, logger *zap.Logger) (chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		// Start Docker
		client, network := interchaintest.DockerSetup(t)
		var localchain chains.Chain
		localchain = NewIconLocalnet(t.Name(), logger, chainConfig.GetIBCChainConfig(&localchain), chains.DefaultNumValidators, chains.DefaultNumFullNodes, keystorePath, keyPassword, scorePaths)
		ic := interchaintest.NewInterchain().
			AddChain(localchain.(ibc.Chain))
		// Log location
		f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
		if err != nil {
			return nil, err
		}
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
