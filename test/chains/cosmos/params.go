package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/cosmos/cosmos-sdk/types"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
)

func (c *CosmosLocalnet) GetQueryParam(method string) Query {
	var queryMsg Query
	if strings.Contains(method, "admin") {
		queryMsg = Query{GetAdmin: &GetAdmin{}}
	} else if strings.Contains(method, "fee") {
		queryMsg = Query{GetProtocolFee: &GetProtocolFee{}}
	}
	return queryMsg
}

func (c *CosmosLocalnet) GetExecuteParam(ctx context.Context, methodaName, param string) (context.Context, string, error) {
	if strings.Contains(methodaName, "set_admin") {
		return c.SetAdminParams(ctx, param)
	} else if strings.Contains(methodaName, "update_admin") {
		return c.UpdateAdminParams(ctx, param)
	} else if strings.Contains(methodaName, "remove_admin") {
		originalJSON := `{"remove_admin":{}}`
		return ctx, string(originalJSON), nil
	}
	return ctx, "", nil
}

func (c *CosmosLocalnet) GetAndFundTestUser(
	ctx context.Context,
	keyNamePrefix string,
	amount int64,
	chain ibc.Chain,
) (string, error) {
	// Check if the address for the given key is already created
	_, err := c.CosmosChain.GetAddress(ctx, keyNamePrefix)
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
		fmt.Printf("Address of %s is : %s \n", user.KeyName(), user.FormattedAddress())
		return user.KeyName(), nil
	} else {
		return keyNamePrefix, err
	}
}

func (c *CosmosLocalnet) SetAdminParams(ctx context.Context, keyName string) (context.Context, string, error) {
	var admin SetAdmin
	var admins chains.Admins
	originalJSON := `{"set_admin":{"address":""}}`
	json.Unmarshal([]byte(originalJSON), &admin)
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(originalJSON), nil
	} else if strings.ToLower(keyName) == "junk" {
		admin.SetAdmin.Address = "$%#^!(&^%^)"
		updatedJSON, _ := json.Marshal(admin)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	} else {
		// Check if the given wallet exists if not create a wallet
		addr, err := c.CosmosChain.GetAddress(ctx, keyName)
		if err != nil {
			c.BuildWallets(ctx, keyName)
			addr, _ = c.CosmosChain.GetAddress(ctx, keyName)
		}
		adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
		admin.SetAdmin.Address = adminAddr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminAddr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	}
}

func (c *CosmosLocalnet) UpdateAdminParams(ctx context.Context, keyName string) (context.Context, string, error) {
	var admin UpdateAdmin
	var admins chains.Admins
	originalJSON := `{"update_admin":{"address":""}}`
	json.Unmarshal([]byte(originalJSON), &admin)
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(originalJSON), nil
	} else if strings.ToLower(keyName) == "junk" {
		admin.UpdateAdmin.Address = "$%#^!(&^%^)"
		updatedJSON, _ := json.Marshal(admin)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	} else {
		// Check if the given wallet exists if not create a wallet
		addr, err := c.CosmosChain.GetAddress(ctx, keyName)
		if err != nil {
			c.BuildWallets(ctx, keyName)
			addr, _ = c.CosmosChain.GetAddress(ctx, keyName)
		}
		adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
		admin.UpdateAdmin.Address = adminAddr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: adminAddr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	}
}
