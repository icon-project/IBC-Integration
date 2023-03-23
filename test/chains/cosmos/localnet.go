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

	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName
	codeId, _ := c.CosmosChain.StoreContract(ctx, c.keyName, c.filepath[contractName])

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg
	address, err := c.CosmosChain.InstantiateContract(ctx, c.keyName, codeId, initMessage, true)
	return context.WithValue(ctx, chains.ContractKey{}, chains.ContractKey{
		ContractAddress: address,
	}), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context) (context.Context, error) {
	ctxValue := ctx.Value(chains.ContractKey{}).(chains.ContractKey)
	contractAddress := ctxValue.ContractAddress
	var response interface{}
	var r Query
	ctxVal := ctx.Value(chains.Query{}).(chains.Query)
	if ctxVal.Query == "get_admin" {
		r = Query{GetAdmin: &GetAdmin{}}
	} else if ctxVal.Query == "get_protocol_fee" {
		r = Query{GetProtocolFee: &GetProtocolFee{}}
	}

	err := c.CosmosChain.QueryContract(ctx, contractAddress, r, &response)
	fmt.Printf("Response is : %s", response)
	return ctx, err
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context) (context.Context, error) {
	ctxValue := ctx.Value(chains.ContractKey{}).(chains.ContractKey)
	contractAddress := ctxValue.ContractAddress
	paramValue := ctx.Value(chains.Param{}).(chains.Param)
	err := c.CosmosChain.ExecuteContract(ctx, c.keyName, contractAddress, paramValue.Data)
	return ctx, err
}

func (c *CosmosLocalnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	h, err := c.CosmosChain.Height(ctx)
	return context.WithValue(ctx, chains.LastBlock{}, h), err
}

func (c *CosmosLocalnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("Unimplemented") // TODO: Implement
}
