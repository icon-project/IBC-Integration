package integration_test

import (
	"fmt"

	"github.com/spf13/viper"
)

type Config struct {
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

func GetConfig() *Config {
	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		fmt.Println(err)
	}

	return &config
}
