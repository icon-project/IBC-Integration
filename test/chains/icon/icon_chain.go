package icon

import (
	"fmt"

	"github.com/icon-project/ibc-integration/test/chains"
	"go.uber.org/zap"
)

func NewIconChain(environment string, chainConfig chains.ChainConfig, nid string, keystorePath string, keyPassword string, url string, scorePaths map[string]string, logger *zap.Logger) (chains.Chain, error) {
	switch environment {
	case "local", "localnet":
		// Start Docker
		chain := NewIconLocalnet("", logger, chainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes)
		return chain, nil
	case "testnet":
		return NewIconTestnet(chainConfig.Bin, nid, keystorePath, keyPassword, "5000000000", url, scorePaths), nil
	default:
		return nil, fmt.Errorf("unknown environment: %s", environment)
	}

	return nil, nil
}
