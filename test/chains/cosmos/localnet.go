package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
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

func (c *CosmosLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	users := interchaintest.GetAndFundTestUsers(c.t, ctx, keyName, int64(100_000_000), c.CosmosChain)
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
	var contracts chains.ContractKey

	contracts.ContractAddress = map[string]string{
		contractName: address,
	}

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   c.keyName,
	}), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	queryMsg := c.GetQueryParam(methodName)
	err := c.CosmosChain.QueryContract(ctx, contractAddress, queryMsg, &chains.Response)
	fmt.Printf("Response is : %s \n", chains.Response)
	return ctx, err
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, methodaName, param string) (context.Context, error) {
	ctx, params := c.GetExecuteParam(ctx, methodaName, param)
	err := c.CosmosChain.ExecuteContract(ctx, c.keyName, contractAddress, params)
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

func (c *CosmosLocalnet) SetAdminParams(ctx context.Context, keyName string) (context.Context, string) {
	originalJSON := `{"set_admin":{"address":""}}`
	var admin Admin
	json.Unmarshal([]byte(originalJSON), &admin)

	adminWallet, _ := c.CosmosChain.BuildWallet(ctx, keyName, "")
	adminKey := adminWallet.FormattedAddress()
	// Update the value of the "address" key
	admin.SetAdmin.Address = adminKey

	// Marshal the struct back into JSON
	updatedJSON, _ := json.Marshal(admin)

	// Print the updated JSON string
	fmt.Println(string(updatedJSON))
	var admins chains.Admins

	admins.Admin = map[string]string{
		keyName: adminKey,
	}

	return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
		Admin: admins.Admin,
	}), string(updatedJSON)
}

func (c *CosmosLocalnet) GetQueryParam(method string) Query {
	var queryMsg Query
	if strings.Contains(method, "admin") {
		queryMsg = Query{GetAdmin: &GetAdmin{}}
	} else if strings.Contains(method, "fee") {
		queryMsg = Query{GetProtocolFee: &GetProtocolFee{}}
	}
	return queryMsg
}

func (c *CosmosLocalnet) GetExecuteParam(ctx context.Context, methodaName, param string) (context.Context, string) {
	if strings.Contains(methodaName, "admin") {
		return c.SetAdminParams(ctx, param)
	}
	return ctx, ""
}

func (c *CosmosLocalnet) CreateKey(ctx context.Context, keyName string) error {
	return c.CosmosChain.CreateKey(ctx, keyName)
}
