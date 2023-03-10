package cosmos

import (
	"context"
	"fmt"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/strangelove-ventures/interchaintest/v6/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"go.uber.org/zap"
)

type CosmosLocalnet struct {
	*cosmos.CosmosChain
}

func NewCosmosLocalnet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int) (chains.Chain, error) {
	chain := cosmos.NewCosmosChain(testName, chainConfig, numValidators, numFullNodes, log)
	return &CosmosLocalnet{
		CosmosChain: chain,
	}, nil
}

func (c *CosmosLocalnet) DeployContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	fmt.Println("**************************************************************************************************")
	panic("Unimplemented") // TODO: Implement
}
