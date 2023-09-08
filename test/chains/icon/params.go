package icon

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"path"
	"strings"

	"github.com/icon-project/ibc-integration/test/chains"
)

func (c *IconLocalnet) getExecuteParam(ctx context.Context, methodName string, params map[string]interface{}) (string, string) {
	if strings.Contains(methodName, chains.BindPort) {
		_params, _ := json.Marshal(map[string]interface{}{
			"portId":        params["port_id"],
			"moduleAddress": params["address"],
		})
		return "bindPort", string(_params)
	} else if strings.Contains(methodName, chains.SendMessage) {
		_params, _ := json.Marshal(map[string]interface{}{
			"data":          hex.EncodeToString(params["msg"].(chains.BufferArray)),
			"timeoutHeight": fmt.Sprintf("%d", params["timeout_height"]),
		})

		return "sendPacket", string(_params)
	}
	_params, _ := json.Marshal(params)

	return methodName, string(_params)
}

func (c *IconLocalnet) GetQueryParam(method string, params map[string]interface{}) Query {
	var query Query
	switch method {
	case chains.HasPacketReceipt:
		query = Query{
			"getPacketReceipt",
			Value{map[string]interface{}{
				"portId":    params["port_id"],
				"channelId": params["channel_id"],
				"sequence":  fmt.Sprintf("%d", params["sequence"]), //common.NewHexInt(int64(sequence)),
			}},
		}
		break
	case chains.GetClientState:
		query = Query{
			"getClientState",
			Value{map[string]interface{}{
				"clientId": params["client_id"],
			}},
		}
		break
	case chains.GetNextClientSequence:
		query = Query{
			"getNextClientSequence",
			Value{map[string]interface{}{}},
		}
		break
	case chains.GetNextConnectionSequence:
		query = Query{
			"getNextConnectionSequence",
			Value{map[string]interface{}{}},
		}
		break
	case chains.GetNextChannelSequence:
		query = Query{
			"getNextChannelSequence",
			Value{map[string]interface{}{}},
		}
		break
	case chains.GetConnection:
		query = Query{
			"getConnection",
			Value{map[string]interface{}{
				"connectionId": params["connection_id"],
			}},
		}
		break
	case chains.GetChannel:
		query = Query{
			"getChannel",
			Value{map[string]interface{}{
				"channelId": params["channel_id"],
				"portId":    params["port_id"],
			}},
		}
		break
	}
	return query
}

func (c *IconLocalnet) getInitParams(ctx context.Context, contractName string, initMsg map[string]interface{}) string {
	if contractName == "mockdapp" {
		updatedInit, _ := json.Marshal(map[string]string{
			"ibcHandler": initMsg["ibc_host"].(string),
		})
		fmt.Printf("Init msg for Dapp is : %s", string(updatedInit))
		return string(updatedInit)
	}
	return ""
}

func (c *IconLocalnet) SetAdminParams(ctx context.Context, methodaName, keyName string) (context.Context, string, string) {
	var admins chains.Admins
	executeMethodName := "setAdmin"
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, ""
	} else if strings.ToLower(keyName) == "junk" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, "$%$@&#6"
	} else {
		wallet, _ := c.BuildWallet(ctx, keyName, "")
		addr := wallet.FormattedAddress()
		admins.Admin = map[string]string{
			keyName: addr,
		}
		args := "_address=" + addr
		fmt.Printf("Address of %s is %s\n", keyName, addr)
		fmt.Println(args)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, args
	}

}

func (c *IconLocalnet) UpdateAdminParams(ctx context.Context, methodaName, keyName string) (context.Context, string, string) {
	var admins chains.Admins
	executeMethodName := "updateAdmin"
	if strings.ToLower(keyName) == "null" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, ""
	} else if strings.ToLower(keyName) == "junk" {
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, "$%$@&#6"
	} else {
		wallet, _ := c.BuildWallet(ctx, keyName, "")
		addr := wallet.FormattedAddress()
		admins.Admin = map[string]string{
			keyName: addr,
		}
		args := "_address=" + addr
		fmt.Printf("Address of %s is %s\n", keyName, addr)
		fmt.Println(args)
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), executeMethodName, args
	}

}

func (c *IconLocalnet) CheckForKeyStore(ctx context.Context, keyName string) ibc.Wallet {
	// Check if keystore file exists for given keyname if not create a keystore file
	jsonFile := keyName + ".json"
	ksPath := path.Join(c.HomeDir(), jsonFile)
	_, _, err := c.getFullNode().Exec(ctx, []string{"cat", ksPath}, nil)
	if err == nil {
		c.keystorePath = ksPath
		return nil
	}
	address, privateKey, _ := c.createKeystore(ctx, keyName)

	wallet := NewWallet(keyName, []byte(address), privateKey, c.cfg)
	c.Wallets[keyName] = wallet

	fmt.Printf("Address of %s is: %s\n", keyName, wallet.FormattedAddress())
	c.keystorePath = ksPath

	return wallet
}
