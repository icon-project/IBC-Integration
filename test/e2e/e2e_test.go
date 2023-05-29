package e2e

import (
	"flag"
	"fmt"
	"log"
	"os"
	"testing"

	"github.com/spf13/viper"
)

func TestMain(m *testing.M) {
	var config string
	cwd, err := os.Getwd()
 defaultConfigPath := fmt.Sprintf("%s/config.yaml", pwd)
	flag.StringVar(&config, "config", defaultConfigPath, "path to config file")
	flag.Parse()

	if config == "" && err != nil {
  if _, err := os.Stat(defaultConfigPath) {
    config := defaultConfigPath
  } else {
    log.Fatal("Config not provided")
  }
	}

	viper.SetConfigFile(config)
	viper.AutomaticEnv()
	if err := viper.ReadInConfig(); err != nil {
		log.Fatalf("Error reading config file: %s\n", err)
	}
	os.Exit(m.Run())
}