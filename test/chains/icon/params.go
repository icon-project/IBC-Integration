package icon

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
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
	//sequence, _ := strconv.Atoi(params["sequence"].(string))
	switch method {
	case chains.HasPacketReceipt:
		//_params := []string{
		//	fmt.Sprintf("portId=%v", params["port_id"]),
		//	fmt.Sprintf(`channelId=%v`, params["channel_id"]),
		//	fmt.Sprintf(`sequence=%v`, sequence),
		//}

		//params := fmt.Sprintf(`portId=%s,channelId=channel-%d`, portID, channelSuffix)
		//query = Query{
		//	"getPacketReceipt",
		//	strings.Join(_params, ","),
		//}
		query = Query{
			"hasPacketReceipt",
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

func (c *IconLocalnet) CheckForKeyStore(ctx context.Context, keyName string) string {
	// Check if keystore file exists for given keyname if not create a keystore file
	jsonFile := keyName + ".json"
	path := path.Join(c.HomeDir(), jsonFile)
	_, _, err := c.getFullNode().Exec(ctx, []string{"cat", path}, nil)
	if err == nil {
		c.keystorePath = path
		return ""
	}

	wallet, _ := c.BuildWallet(ctx, keyName, "")
	fmt.Printf("Address of %s is: %s\n", keyName, wallet.FormattedAddress())
	c.keystorePath = path
	return wallet.FormattedAddress()
}
