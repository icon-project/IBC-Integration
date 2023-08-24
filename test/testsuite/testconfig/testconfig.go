package testconfig

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"path"
	"path/filepath"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite/relayer"
	"github.com/spf13/viper"
)

const (
	TestConfigFilePathEnv = "TEST_CONFIG_PATH"
	defaultConfigFileName = "sample-config-archway.yaml"
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

type DebugConfig struct {
	// DumpLogs forces the logs to be collected before removing test containers.
	DumpLogs bool `yaml:"dumpLogs"`
}

type TestConfig struct {
	// ChainConfigs holds configuration values related to the chains used in the tests.
	ChainConfigs []Chain `mapstructure:"chains"`
	// RelayerConfig holds configuration for the relayer to be used.
	RelayerConfig relayer.Config `mapstructure:"relayer"`
	// DebugConfig holds configuration for miscellaneous options.
	//DebugConfig DebugConfig `yaml:"debug"`
}

type ChainOptions struct {
	ChainAConfig *Chain
	ChainBConfig *Chain
}

// ChainOptionConfiguration enables arbitrary configuration of ChainOptions.
type ChainOptionConfiguration func(options *ChainOptions)

// DefaultChainOptions returns the default configuration for the chains.
// These options can be configured by passing configuration functions to E2ETestSuite.GetChains.
func DefaultChainOptions() (*ChainOptions, error) {
	tc, err := New()
	if err != nil {
		return nil, err
	}
	return &ChainOptions{
		ChainAConfig: &tc.ChainConfigs[0],
		ChainBConfig: &tc.ChainConfigs[1],
	}, nil
}

func New() (*TestConfig, error) {
	return fromFile()
}

func fromFile() (*TestConfig, error) {

	cwd, _ := os.Getwd()
	basePath := filepath.Dir(fmt.Sprintf("%s/..%c..%c", cwd, os.PathSeparator, os.PathSeparator))

	if err := os.Setenv(chains.BASE_PATH, basePath); err != nil {
		log.Fatal("Error setting BASE_PATH:", err)
	}

	configFile := getConfigFilePath(basePath)
	confContent, err := os.ReadFile(configFile)
	if err != nil {
		return nil, err
	}
	reader := bytes.NewBuffer([]byte(os.ExpandEnv(string(confContent))))
	viper.AutomaticEnv()
	viper.SetConfigType(filepath.Ext(configFile)[1:])
	if err := viper.ReadConfig(reader); err != nil {
		return nil, err
	}
	var tc = new(TestConfig)
	return tc, viper.Unmarshal(tc)
}

// getConfigFilePath returns the absolute path where the e2e config file should be.
func getConfigFilePath(srcPath string) string {
	if absoluteConfigPath := os.Getenv(TestConfigFilePathEnv); absoluteConfigPath != "" {
		return absoluteConfigPath
	}
	return path.Join(srcPath, "test", "testsuite", defaultConfigFileName)
}
