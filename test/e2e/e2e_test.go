package e2e_test

import (
	"bytes"
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
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
	basePath := filepath.Dir(fmt.Sprintf("%s/..%c..%c", cwd, os.PathSeparator, os.PathSeparator))
	if err := os.Setenv("BASE_PATH", basePath); err != nil {
		log.Fatalf("Error setting BASE_PATH: %s/n", err)
	}
	fmt.Println("Using config file:", config)
	viper.SetConfigFile(config)

	contents, err := os.ReadFile(config)
	if err != nil {
		log.Fatalf("Error opening config file: %s\n", err)
	}
	reader := bytes.NewBuffer([]byte(os.ExpandEnv(string(contents))))
	viper.AutomaticEnv()
	if err := viper.ReadConfig(reader); err != nil {
		log.Fatalf("Error reading config file: %s\n", err)
	}
	os.Exit(m.Run())
}
