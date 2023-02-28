package e2e_test

import (
	"flag"
	"fmt"
	"os"
	"testing"

	"github.com/spf13/viper"
)

func TestMain(m *testing.M) {
	var config string
	flag.StringVar(&config, "config", "", "path to config file")
	flag.Parse()
	
	if config == "" {
		fmt.Println("Config not provided")
		os.Exit(1)
	}

	viper.SetConfigFile(config)
	if err := viper.ReadInConfig(); err != nil {
		fmt.Printf("Error reading config file: %s\n", err)
		os.Exit(1)
	}

	os.Exit(m.Run())
}