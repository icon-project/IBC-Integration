package e2e_test

import (
	"fmt"
	"github.com/spf13/viper"
)

type Config struct {

}

func GetConfig() *Config {
	var config Config
	if err := viper.Unmarshal(&config); err != nil {
		fmt.Println(err)
	}

	return &config
}