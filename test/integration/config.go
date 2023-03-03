package integration_test

import (
	"fmt"
	"github.com/spf13/viper"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
)

type Config struct {
	Chain            Chain  `mapstructure:"chain"`
	Environment      string `mapstructure:"environment"`
	URL              string `mapstructure:"url"`
	KeystoreFile     string `mapstructure:"keystore_file"`
	KeystorePassword string `mapstructure:"keystore_password"`
	ChainId          string `mapstructure:"chain_id"`
	Bech32Prefix     string `mapstructure:"bech32_prefix"`
	GasAdjustment    string `mapstructure:"gas_adjustment"`
	GasPrice         string `mapstructure:"gas_price"`
	Bin              string `mapstructure:"bin"`
	TrustingPeriod   string `mapstructure:"trusting_period"`
	NID              string `mapstructure:"nid"`
}

type Chain struct {
	Name        string      `mapstructure:"name"`
	ChainConfig ChainConfig `mapstructure:"chain_config"`
}

type ChainConfig struct {
	Type           string      `mapstructure:"type"`
	Name           string      `mapstructure:"name"`
	ChainID        string      `mapstructure:"chain_id"`
	Images         DockerImage `mapstructure:"image"`
	Bin            string      `mapstructure:"bin"`
	Bech32Prefix   string      `mapstructure:"bech32_prefix"`
	Denom          string      `mapstructure:"denom"`
	CoinType       string      `mapstructure:"coin_type"`
	GasPrices      string      `mapstructure:"gas_prices"`
	GasAdjustment  float64     `mapstructure:"gas_adjustment"`
	TrustingPeriod string      `mapstructure:"trusting_period"`
	NoHostMount    bool        `mapstructure:"no_host_mount"`
}

type DockerImage struct {
	Repository string `mapstructure:"repository"`
	Version    string `mapstructure:"version"`
	UidGid     string `mapstructure:"uid_gid"`
}

func GetConfig() *Config {
	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		fmt.Println(err)
	}

	return &config
}
