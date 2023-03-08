package cosmos

import (
	"fmt"
	"github.com/icon-project/ibc-integration/test/chains"
)

func NewCosmosChain(environment string, chainConfig chains.ChainConfig) (chains.Chain, error) {
	switch environment {
	case "local", "localnet":

	case "testnet":
	default:
		return nil, fmt.Errorf("unknown environment: %s", environment)
	}

	return nil, nil
}
