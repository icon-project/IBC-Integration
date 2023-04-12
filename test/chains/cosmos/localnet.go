package cosmos

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
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

func (c *CosmosLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Fund user to deploy contract
	contractOwner, _ := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)

	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName
	codeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath[contractName])
	if err != nil {
		return ctx, err
	}

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg
	address, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, codeId, initMessage, true)
	if err != nil {
		return nil, err
	}

	var contracts chains.ContractKey
	contracts.ContractAddress = map[string]string{
		contractName: address,
	}

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   keyName,
	}), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	// wait for few blocks after executing before querying
	time.Sleep(2 * time.Second)

	// get query msg
	queryMsg := c.GetQueryParam(methodName)
	chains.Response = ""
	err := c.CosmosChain.QueryContract(ctx, contractAddress, queryMsg, &chains.Response)
	fmt.Printf("Response is : %s \n", chains.Response)
	return ctx, err
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodaName, param string) (context.Context, error) {
	// get param for executing a method in the contract
	ctx, params, err := c.GetExecuteParam(ctx, methodaName, param)
	if err != nil {
		return ctx, err
	}
	err = c.CosmosChain.ExecuteContract(ctx, keyName, contractAddress, params)
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
	return nil, nil
}

func (c *CosmosLocalnet) BuildWallets(ctx context.Context, keyName string) error {
	// Build Wallet and fund user
	_, err := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)
	return err
}
