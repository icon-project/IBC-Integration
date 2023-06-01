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
	flag.StringVar(&config, "config", fmt.Sprintf("%s%cconfig.yaml", cwd, os.PathSeparator), "path to config file")
	flag.Parse()

	if config == "" && err != nil {
		log.Fatal("Config not provided")
	}
	fmt.Println("Using config file:", config)
	viper.SetConfigFile(config)
	viper.AutomaticEnv()
	if err := viper.ReadInConfig(); err != nil {
		log.Fatalf("Error reading config file: %s\n", err)
	}
	os.Exit(m.Run())
}
