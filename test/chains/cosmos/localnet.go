package cosmos

import (
	"context"
	"fmt"
	"testing"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	interchaintest "github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"go.uber.org/zap"
)

type CosmosLocalnet struct {
	*cosmos.CosmosChain
	keyName  string
	filepath map[string]string
	t        *testing.T
}

func NewCosmosLocalnet(t *testing.T, log *zap.Logger, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int, keyPassword string, contracts map[string]string) (chains.Chain, error) {
	chain := cosmos.NewCosmosChain(t.Name(), chainConfig, numValidators, numFullNodes, log)
	return &CosmosLocalnet{
		CosmosChain: chain,
		keyName:     keyPassword,
		filepath:    contracts,
		t:           t,
	}, nil
}

func (c *CosmosLocalnet) DeployContract(ctx context.Context) (context.Context, error) {
	users := interchaintest.GetAndFundTestUsers(c.t, ctx, "default", int64(100_000_000), c.CosmosChain)
	destUser := users[0]
	c.keyName = destUser.KeyName()

	// TODO Init Message
	codeId, _ := c.CosmosChain.StoreContract(ctx, c.keyName, c.filepath["cosmos_contract"])
	address, err := c.CosmosChain.InstantiateContract(ctx, c.keyName, codeId, "Init Message", false)
	return context.WithValue(ctx, chains.ContractKey{}, chains.ContractKey{
		ContractAddress: address,
	}), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	h, err := c.CosmosChain.Height(ctx)
	return context.WithValue(ctx, chains.LastBlock{}, h), err
}

func (c *CosmosLocalnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	fmt.Println("**************************************************************************************************")
	panic("Unimplemented") // TODO: Implement
}
