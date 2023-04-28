package icon

import (
	"context"
	"fmt"
	"path"
	"strings"

	"github.com/icon-project/ibc-integration/test/chains"
)

func (c *IconLocalnet) GetExecuteParam(ctx context.Context, methodName, params string) (context.Context, string, string) {
	if strings.Contains(methodName, "set_admin") {
		return c.SetAdminParams(ctx, methodName, params)
	} else if strings.Contains(methodName, "update_admin") {
		// TODO: update admin method is not found
		return c.UpdateAdminParams(ctx, "update_admin", params)
	} else if strings.Contains(methodName, "remove_admin") {
		// TODO: remove admin method is not found
		return ctx, "remove_admin", "_address='hjsdbjd'"
	}
	return ctx, "", ""
}

func (c *IconLocalnet) GetQueryParam(methodName string) string {
	if strings.Contains(methodName, "get_admin") {
		return "admin"
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
	if err != nil {
		wallet, _ := c.BuildWallet(ctx, keyName, "")
		fmt.Printf("Address of %s is: %s\n", keyName, wallet.FormattedAddress())
		c.keystorePath = path
		return wallet.FormattedAddress()
	} else {
		c.keystorePath = path
		return ""
	}
}
