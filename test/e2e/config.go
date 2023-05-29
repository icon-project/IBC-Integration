package e2e

import (
	"fmt"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/icon-bridge/common/log"
	"github.com/spf13/viper"
	"os"
	"path/filepath"
)

type Config struct {
	Icon         Chain `mapstructure:"icon"`
	Counterparty Chain `mapstructure:"counterparty"`
}

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

func GetConfig() (*Config, error) {
	var config = new(Config)

	cwd, err := os.Getwd()
	basePath := filepath.Dir(fmt.Sprintf("%s/..%s..%s", cwd, string(os.PathSeparator), string(os.PathSeparator)))

	if err := os.Setenv("BASE_PATH", basePath); err != nil {
		log.Fatalf("Error setting BASE_PATH", err)
	}

	for _, v := range viper.AllKeys() {
		viper.Set(v, os.ExpandEnv(viper.GetString(v)))
	}
	return config, viper.Unmarshal(config)
}