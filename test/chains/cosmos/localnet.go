package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"testing"

	"github.com/cosmos/cosmos-sdk/types"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
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
	// get query msg
	queryMsg := c.GetQueryParam(methodName)
	chains.Response = ""
	err := c.CosmosChain.QueryContract(ctx, contractAddress, queryMsg, &chains.Response)
	fmt.Printf("Response is : %s \n", chains.Response)
	return ctx, err
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodaName, param string) (context.Context, error) {
	// get param for executing a method in the contract
	ctx, params := c.GetExecuteParam(ctx, methodaName, param)
	err := c.CosmosChain.ExecuteContract(ctx, keyName, contractAddress, params)
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
	var admin Admin
	var admins chains.Admins
	originalJSON := `{"set_admin":{"address":""}}`
	json.Unmarshal([]byte(originalJSON), &admin)

	// Check if an wallet is already created for given key
	addr, err := c.CosmosChain.GetAddress(ctx, keyName)
	adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
	if err != nil {
		adminWallet, _ := c.CosmosChain.BuildWallet(ctx, keyName, "")
		adminKey := adminWallet.FormattedAddress()
		// fund admin wallet
		c.BuildWallets(ctx, adminWallet.KeyName())
		// Update the value of the "address" key
		admin.SetAdmin.Address = adminKey
		// Marshal the struct back into JSON
		updatedJSON, _ := json.Marshal(admin)
		// Print the updated JSON string
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminKey,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON)
	} else {
		admin.SetAdmin.Address = adminAddr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminAddr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON)
	}

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

func (c *CosmosLocalnet) BuildWallets(ctx context.Context, keyName string) error {
	_, err := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)
	return err
}

func (c *CosmosLocalnet) GetAndFundTestUser(
	ctx context.Context,
	keyNamePrefix string,
	amount int64,
	chain ibc.Chain,
) (string, error) {
	addr, err := c.CosmosChain.GetAddress(ctx, keyNamePrefix)
	adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
	if err != nil {
		chainCfg := c.CosmosChain.Config()
		user, err := chain.BuildWallet(ctx, keyNamePrefix, "")
		if err != nil {
			return "", fmt.Errorf("failed to get source user wallet: %w", err)
		}

		err = chain.SendFunds(ctx, chains.FaucetAccountKeyName, ibc.WalletAmount{
			Address: user.FormattedAddress(),
			Amount:  amount,
			Denom:   chainCfg.Denom,
		})
		if err != nil {
			return "", fmt.Errorf("failed to get funds from faucet: %w", err)
		}
		return user.KeyName(), nil
	} else {
		err = chain.SendFunds(ctx, chains.FaucetAccountKeyName, ibc.WalletAmount{
			Address: adminAddr,
			Amount:  amount,
			Denom:   c.CosmosChain.Config().Denom,
		})
		return keyNamePrefix, err
	}
}
