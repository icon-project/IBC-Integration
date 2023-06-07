package e2e_test

import (
	"bytes"
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"testing"

	_ "github.com/spf13/cobra"
	"github.com/spf13/viper"
)

func TestMain(m *testing.M) {
	var config string
	cwd, err := os.Getwd()
	flag.StringVar(&config, "config", "config.yaml", "config file name")
	flag.Parse()
	if config == "" && err != nil {
		log.Fatal("Config not provided")
	}
	basePath := filepath.Dir(fmt.Sprintf("%s/..%c..%c", cwd, os.PathSeparator, os.PathSeparator))
	if err := os.Setenv("BASE_PATH", basePath); err != nil {
		log.Fatal("Error setting BASE_PATH:", err)
	}
	fmt.Println("BASE_PATH:", basePath)
	contents, err := os.ReadFile(fmt.Sprintf("%s%c%s", cwd, os.PathSeparator, config))
	if err != nil {
		log.Fatal("error opening config file:", err)
	}
	reader := bytes.NewBuffer([]byte(os.ExpandEnv(string(contents))))
	viper.AutomaticEnv()
	viper.SetConfigType(filepath.Ext(config)[1:])
	if err := viper.ReadConfig(reader); err != nil {
		log.Fatal("Error reading config file:", err)
	}
	os.Exit(m.Run())
}
