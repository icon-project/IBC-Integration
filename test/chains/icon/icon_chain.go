package icon

import (
	"context"
	"fmt"
	"testing"
	"time"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"github.com/strangelove-ventures/interchaintest/v6/testreporter"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func NewIconChain(t *testing.T, ctx context.Context, environment string, chainConfig chains.ChainConfig, nid string, keystorePath string, keyPassword string, url string, scorePaths map[string]string, logger *zap.Logger) (context.Context, chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		// Start Docker
		// cf := interchaintest.NewBuiltinChainFactory(zaptest.NewLogger(t), []*interchaintest.ChainSpec{
		// 	// Source chain
		// 	{Name: "icon", ChainConfig: ibc.ChainConfig{
		// 		Type:    "icon",
		// 		Name:    "icon",
		// 		ChainID: "icon-1",
		// 		Images: []ibc.DockerImage{
		// 			{
		// 				Repository: "hemz1012/goloop", // FOR LOCAL IMAGE USE: Docker Image Name
		// 				Version:    "latest",          // FOR LOCAL IMAGE USE: Docker Image Tag
		// 			},
		// 		},
		// 		Bin:            "goloop",
		// 		Bech32Prefix:   "icon",
		// 		Denom:          "icx",
		// 		GasPrices:      "0.001icx",
		// 		GasAdjustment:  1.3,
		// 		TrustingPeriod: "508h",
		// 		NoHostMount:    false},
		// 	},
		// },
		// )
		// chains, _ := cf.Chains(t.Name())
		// dest := chains[0]
		client, network := interchaintest.DockerSetup(t)
		localchain := NewIconLocalnet(t.Name(), logger, chainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes)
		ic := interchaintest.NewInterchain().
			AddChain(localchain.(ibc.Chain))
		// Log location
		f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
		if err != nil {
			return ctx, nil, err
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

		return context.WithValue(ctx, "ibc.chain", localchain.(ibc.Chain)), localchain, nil
	case "testnet":
		return ctx, NewIconTestnet(chainConfig.Bin, nid, keystorePath, keyPassword, "5000000000", url, scorePaths), nil
	default:
		return ctx, nil, fmt.Errorf("unknown environment: %s", environment)
	}
}
