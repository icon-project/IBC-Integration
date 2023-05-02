package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"

	"github.com/cosmos/cosmos-sdk/types"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
)

var packet = `{"ibc_packet_receive":{"msg":{"packet":{"data":"+FfBALhT+FHGhWFsaWNluEJhcmNod2F5MW45emhtaDY0YTJ2MmY5cDlwajh4MHU1YzQ5bHV3bWZrM2pxdmVqeGs5dHpxbXN1ajlsOXNhdnhydTMBgIMBAgM=",
"src":{"port_id":"our-port","channel_id":"channel-1"},"dest":{"port_id":"their-port","channel_id":"channel-3"},"sequence":0,"timeout":
{"block":{"revision":0,"height":0},"timestamp":null}},"relayer":"relay"}}}`

var args = `{"send_call_message":{"to":"hjhbd","data":[1,2,3],"rollback":[3,4,5]}}`

func (c *CosmosLocalnet) GetQueryParam(method string) Query {
	var queryMsg Query
	if strings.Contains(method, "admin") {
		queryMsg = Query{GetAdmin: &GetAdmin{}}
	} else if strings.Contains(method, "fee") {
		queryMsg = Query{GetProtocolFee: &GetProtocolFee{}}
	}
	return queryMsg
}

func (c *CosmosLocalnet) GetExecuteParam(ctx context.Context, methodName, param string) (context.Context, string, error) {
	if strings.Contains(methodName, "set_admin") {
		return c.SetAdminParams(ctx, param)
	} else if strings.Contains(methodName, "update_admin") {
		return c.UpdateAdminParams(ctx, param)
	} else if strings.Contains(methodName, "remove_admin") {
		originalJSON := `{"remove_admin":{}}`
		return ctx, string(originalJSON), nil
	} else if strings.Contains(methodName, "ibc_packet_receive") {
		packetData := packet
		return ctx, string(packetData), nil
	} else if strings.Contains(methodName, "send_call_message") {
		sendCall := args
		return ctx, string(sendCall), nil
	}
	return ctx, "", nil
}

func (c *CosmosLocalnet) GetAndFundTestUser(
	ctx context.Context,
	keyNamePrefix string,
	amount int64,
	chain ibc.Chain,
) (keyName string, address string, err error) {
	// Check if the address for the given key is already created
	addr, err := c.CosmosChain.GetAddress(ctx, keyNamePrefix)
	adminAddr, _ := types.Bech32ifyAddressBytes(c.CosmosChain.Config().Bech32Prefix, addr)
	if err != nil {
		chainCfg := c.CosmosChain.Config()
		user, err := chain.BuildWallet(ctx, keyNamePrefix, "")
		if err != nil {
			return "", "", fmt.Errorf("failed to get source user wallet: %w", err)
		}

		err = chain.SendFunds(ctx, chains.FaucetAccountKeyName, ibc.WalletAmount{
			Address: user.FormattedAddress(),
			Amount:  amount,
			Denom:   chainCfg.Denom,
		})

		if err != nil {
			return "", "", fmt.Errorf("failed to get funds from faucet: %w", err)
		}
		fmt.Printf("Address of %s is : %s \n", user.KeyName(), user.FormattedAddress())
		return user.KeyName(), user.FormattedAddress(), nil
	} else {
		return keyNamePrefix, adminAddr, err
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

func (c *CosmosLocalnet) getInitParams(ctx context.Context, contractName string) string {
	var xcallInit XcallInit
	var DappInit DappInit
	if contractName == "xcall" {
		originalJSON := `{"timeout_height":45, "ibc_host":""}`
		json.Unmarshal([]byte(originalJSON), &xcallInit)
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		coreAddr := ctxValue.ContractAddress["ibccore"]
		xcallInit.IbcHost = coreAddr
		updatedInit, _ := json.Marshal(xcallInit)
		fmt.Printf("Init msg for xCall is : %s", string(updatedInit))
		return string(updatedInit)
	} else if contractName == "dapp" {
		originalJSON := `{"address":""}`
		json.Unmarshal([]byte(originalJSON), &DappInit)
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		xcallAddr := ctxValue.ContractAddress["xcall"]
		DappInit.Address = xcallAddr
		updatedInit, _ := json.Marshal(DappInit)
		fmt.Printf("Init msg for Dapp is : %s", string(updatedInit))
		return string(updatedInit)
	}
	return ""
}
