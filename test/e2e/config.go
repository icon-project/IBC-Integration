package e2e_test

import (
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/spf13/viper"
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
