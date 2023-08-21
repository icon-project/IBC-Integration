package chains

import (
	"encoding/json"
	"fmt"
	"github.com/icza/dyno"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

type ChainConfig struct {
	Type           string      `mapstructure:"type"`
	Name           string      `mapstructure:"name"`
	ChainID        string      `mapstructure:"chain_id"`
	Images         DockerImage `mapstructure:"image"`
	Bin            string      `mapstructure:"bin"`
	Bech32Prefix   string      `mapstructure:"bech32_prefix"`
	Denom          string      `mapstructure:"denom"`
	SkipGenTx      bool        `mapstructure:"skip_gen_tx"`
	CoinType       string      `mapstructure:"coin_type"`
	GasPrices      string      `mapstructure:"gas_prices"`
	GasAdjustment  float64     `mapstructure:"gas_adjustment"`
	TrustingPeriod string      `mapstructure:"trusting_period"`
	NoHostMount    bool        `mapstructure:"no_host_mount"`
	BlockInterval  int         `mapstructure:"block_interval"`
}

type DockerImage struct {
	Repository string `mapstructure:"repository"`
	Version    string `mapstructure:"version"`
	UidGid     string `mapstructure:"uid_gid"`
}

func (c *ChainConfig) GetIBCChainConfig(chain *Chain) ibc.ChainConfig {
	return ibc.ChainConfig{
		Type:    c.Type,
		Name:    c.Name,
		ChainID: c.ChainID,
		Images: []ibc.DockerImage{{
			Repository: c.Images.Repository,
			Version:    c.Images.Version,
			UidGid:     c.Images.UidGid,
		}},
		Bin:          c.Bin,
		Bech32Prefix: c.Bech32Prefix,
		Denom:        c.Denom,
		CoinType:     c.CoinType,
		SkipGenTx:    c.SkipGenTx,
		GasPrices:    c.GasPrices,
		PreGenesis: func(config ibc.ChainConfig) error {
			if config.SkipGenTx {
				_chain := *chain
				return _chain.PreGenesis()
			}
			return nil
		},
		ModifyGenesis: func(config ibc.ChainConfig, bytes []byte) ([]byte, error) {
			if config.Type == "icon" {
				return bytes, nil
			}

			g := make(map[string]interface{})
			if err := json.Unmarshal(bytes, &g); err != nil {
				return nil, fmt.Errorf("failed to unmarshal genesis file: %w", err)
			}

			if config.SkipGenTx {
				//add minimum gas fee
				if err := modifyGenesisMinGasPrice(g, config); err != nil {
					return nil, err
				}
			}

			out, err := json.Marshal(&g)
			if err != nil {
				return nil, fmt.Errorf("failed to marshal genesis bytes to json: %w", err)
			}
			return out, nil

		},
		GasAdjustment:  c.GasAdjustment,
		TrustingPeriod: c.TrustingPeriod,
		NoHostMount:    c.NoHostMount,
	}
}

func modifyGenesisMinGasPrice(g map[string]interface{}, config ibc.ChainConfig) error {

	minGasPriceEntities := []MinimumGasPriceEntity{
		{
			Denom:  config.Denom,
			Amount: "0",
		},
	}
	if err := dyno.Set(g, minGasPriceEntities, "app_state", "globalfee", "params", "minimum_gas_prices"); err != nil {
		return fmt.Errorf("failed to set params minimum gas price in genesis json: %w", err)
	}

	return nil
}
