package chains

import (
	"context"
	"os"

	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

const (
	DefaultNumValidators = 1
	DefaultNumFullNodes  = 1
)

type Chain interface {
	DeployContract(ctx context.Context, keyName string) (context.Context, error)
	QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error)
	ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error)
	GetLastBlock(ctx context.Context) (context.Context, error)
	GetBlockByHeight(ctx context.Context) (context.Context, error)
	FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error)
	BuildWallets(ctx context.Context, keyName string) error
	SetupIBC(ctx context.Context, keyName string) (context.Context, error)
	SetupXCall(ctx context.Context, portId, keyName string) error
	ConfigureBaseConnection(ctx context.Context, connection XCallConnection) (context.Context, error)
	XCall(ctx context.Context, targetChain Chain, keyName, _to string, data, rollback []byte) (string, string, string, error)
	EOAXCall(ctx context.Context, targetChain Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error)
	ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error)
	ExecuteRollback(ctx context.Context, sn string) (context.Context, error)
	FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, string, error)
	FindCallResponse(ctx context.Context, startHeight int64, sn string) (string, error)
	OverrideConfig(key string, value any)
	GetIBCAddress(key string) string
	DeployXCallMockApp(ctx context.Context, connection XCallConnection) error
}

type ChainConfig struct {
	Type           string      `mapstructure:"type"`
	Name           string      `mapstructure:"name"`
	ChainID        string      `mapstructure:"chain_id"`
	Images         DockerImage `mapstructure:"image"`
	Bin            string      `mapstructure:"bin"`
	Bech32Prefix   string      `mapstructure:"bech32_prefix"`
	Denom          string      `mapstructure:"denom"`
	CoinType       string      `mapstructure:"coin_type"`
	GasPrices      string      `mapstructure:"gas_prices"`
	GasAdjustment  float64     `mapstructure:"gas_adjustment"`
	TrustingPeriod string      `mapstructure:"trusting_period"`
	NoHostMount    bool        `mapstructure:"no_host_mount"`
}

type DockerImage struct {
	Repository string `mapstructure:"repository"`
	Version    string `mapstructure:"version"`
	UidGid     string `mapstructure:"uid_gid"`
}

func (c *ChainConfig) GetIBCChainConfig() ibc.ChainConfig {
	return ibc.ChainConfig{
		Type:    c.Type,
		Name:    c.Name,
		ChainID: c.ChainID,
		Images: []ibc.DockerImage{{
			Repository: c.Images.Repository,
			Version:    c.Images.Version,
			UidGid:     c.Images.UidGid,
		}},
		Bin:            c.Bin,
		Bech32Prefix:   c.Bech32Prefix,
		Denom:          c.Denom,
		CoinType:       c.CoinType,
		GasPrices:      c.GasPrices,
		GasAdjustment:  c.GasAdjustment,
		TrustingPeriod: c.TrustingPeriod,
		NoHostMount:    c.NoHostMount,
	}
}

func GetEnvOrDefault(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}
