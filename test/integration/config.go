package integration_test

import (
	"fmt"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/spf13/viper"
)

type Config struct {
	Chain            Chain             `mapstructure:"chain"`
	KeystoreFile     string            `mapstructure:"keystore_file"`
	KeystorePassword string            `mapstructure:"keystore_password"`
	Contracts        map[string]string `mapstructure:"contracts"`
}

type Chain struct {
	Name        string             `mapstructure:"name"`
	Environment string             `mapstructure:"environment"`
	ChainConfig chains.ChainConfig `mapstructure:"chain_config"`
	URL         string             `mapstructure:"url"`
	NID         string             `mapstructure:"nid"`
}

func GetConfig() *Config {
	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		fmt.Println(err)
	}

	return &config
}
