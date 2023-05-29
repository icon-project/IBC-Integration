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
	pwd, err := os.Getwd()
	flag.StringVar(&config, "config", fmt.Sprintf("%s/config_.yaml", pwd), "path to config file")
	flag.Parse()

	if config == "" && err != nil {
		log.Fatal("Config not provided")
	}

	viper.SetConfigFile(config)
	viper.AutomaticEnv()
	if err := viper.ReadInConfig(); err != nil {
		log.Printf("Error reading config file: %s\n", err)
		os.Exit(1)
	}

	os.Exit(m.Run())
}