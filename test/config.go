package interchaintest

import (
	"fmt"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/cosmos"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/spf13/viper"
	"go.uber.org/zap"
)

type Chain struct {
	Name             string             `mapstructure:"name"`
	version          string             `mapstructure:"version"`
	Environment      string             `mapstructure:"environment"`
	ChainConfig      chains.ChainConfig `mapstructure:"chain_config"`
	URL              string             `mapstructure:"url"`
	NID              string             `mapstructure:"nid"`
	KeystoreFile     string             `mapstructure:"keystore_file"`
	KeystorePassword string             `mapstructure:"keystore_password"`
	Contracts        map[string]string  `mapstructure:"contracts"`
}

type OuterConfig struct {
	ChainSpecs []*Chain `mapstructure:"chains"`
}

func GetConfig() (*OuterConfig, error) {
	var config = new(OuterConfig)
	return config, viper.Unmarshal(config)
}

func (o *OuterConfig) mapChains() map[string]*Chain {
	cfg := make(map[string]*Chain)
	for _, c := range o.ChainSpecs {
		cfg[c.Name] = c
	}
	return cfg
}

func (c *Chain) buildChain(testName string, log *zap.Logger) chains.Chain {
	switch c.ChainConfig.Type {
	case "icon":
		return icon.NewIconLocalnet(testName, log, c.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, c.KeystoreFile, c.KeystorePassword, c.Contracts)
	case "archway", "cosmos":
		cosmos, err := cosmos.NewCosmosLocalnet(testName, log, c.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, c.KeystorePassword, c.Contracts)
		if err != nil {
			panic(err)
		}
		return cosmos
	default:
		panic(fmt.Errorf("unexpected error, unknown chain type: %s for chain: %s", c.ChainConfig.Type, c.Name))
	}
}
