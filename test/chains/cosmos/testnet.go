package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"strconv"
	"strings"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
)

func NewCosmosTestnet(bin, keystorePath, keyPassword, defaultStepLimit, url string, scorePaths map[string]string, chainID string) chains.Chain {
	return &CosmosTestnet{
		bin:              bin,
		keystorePath:     keystorePath,
		keyPassword:      keyPassword,
		scorePaths:       scorePaths,
		defaultStepLimit: defaultStepLimit,
		url:              url,
		Client:           nil,
		ChainID:          chainID,
	}
}

const amount string = "1000uconst"

func (c *CosmosTestnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	panic("unimplemented")
}

func (it *CosmosTestnet) OverrideConfig(keyName string, value any) {
	panic("unimplemented")
}

func (c *CosmosTestnet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	panic("unimplemented")
}

func (c *CosmosTestnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data, rollback []byte) (string, string, error) {
	panic("unimplemented")
}

func (c *CosmosTestnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, error) {
	panic("unimplemented")
}

func (c *CosmosTestnet) ExecuteCall(ctx context.Context, reqId string) (context.Context, error) {
	panic("unimplemented")
}

func (c *CosmosTestnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	panic("unimplemented")
}
func (c *CosmosTestnet) FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, error) {
	panic("unimplemented")
}

func (c *CosmosTestnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg

	// Store the contract
	Owneraddr, err := c.transferDenom(keyName, amount)
	if err != nil {
		return ctx, err
	}

	Res, err := exec.Command(c.bin, "tx", "wasm", "store", c.scorePaths[contractName], "--from", keyName, "--chain-id", c.ChainID, "--node",
		c.url, "--fees", "4000uconst", "--gas", "auto", "-y", "--output", "json", "-b", "block").Output()
	if err != nil {
		return ctx, err
	}

	// To get Code ID from response
	jqCmd := exec.Command("jq", "-r", ".logs[0].events[1].attributes[1].value")
	jqCmd.Stdin = strings.NewReader(string(Res))
	ID, _ := jqCmd.Output()
	CodeID := string(ID)
	CodeID = strings.ReplaceAll(CodeID, "\n", "")

	// wait for few blocks to complete
	time.Sleep(5 * time.Second)

	// Instantiate the Contract
	_, err = exec.Command(c.bin, "tx", "wasm", "instantiate", CodeID, initMessage, "--from", keyName, "--node",
		c.url, "--chain-id", c.ChainID, "--gas", "auto", "-y", "--label", "ics-20", "--fees", "264uconst", "--no-admin").Output()
	if err != nil {
		return ctx, err
	}
	time.Sleep(5 * time.Second)
	output, err := exec.Command(c.bin, "query", "wasm", "list-contract-by-code", CodeID, "--node",
		c.url, "--output", "json").Output()
	if err != nil {
		return ctx, err
	}
	jqCmd = exec.Command("jq", "-r", ".contracts[-1]")
	jqCmd.Stdin = strings.NewReader(string(output))
	addr, _ := jqCmd.Output()
	var contracts chains.ContractKey
	address := string(addr)
	address = strings.ReplaceAll(address, "\n", "")

	contracts.ContractAddress = map[string]string{
		contractName: address,
	}
	contracts.ContractOwner = map[string]string{
		keyName: Owneraddr,
	}

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

func (c *CosmosTestnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	// wait for few blocks after executing before querying
	time.Sleep(5 * time.Second)

	// get query msg
	queryMsg := c.GetQueryParam(methodName)
	queryJson, _ := json.Marshal(queryMsg)
	chains.Response = ""

	resp, err := exec.Command(c.bin, "query", "wasm", "contract-state", "smart", contractAddress, string(queryJson),
		"--chain-id", c.ChainID, "--node", c.url).Output()
	if err != nil {
		return ctx, err
	}
	chains.Response = string(resp)
	fmt.Printf("Response is : %s \n", chains.Response)
	return ctx, err
}

func (c *CosmosTestnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	ctx, params, err := c.GetExecuteParam(ctx, methodName, param)
	if err != nil {
		return ctx, err
	}
	c.transferDenom(keyName, amount)
	output, err := exec.Command(c.bin, "tx", "wasm", "execute", contractAddress, params, "--from", keyName, "--chain-id", c.ChainID, "--node",
		c.url, "--fees", "281uconst", "-y", "--output", "json", "-b", "block").Output()
	var result TxResul
	json.Unmarshal(output, &result)
	if result.Code != 0 {
		return ctx, fmt.Errorf("%v", result.RawLog)
	}
	return ctx, err
}

func (c *CosmosTestnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	var result Result
	hash, err := exec.Command(c.bin, "status", "--node", c.url).Output()
	if err != nil {
		fmt.Println(err)
	}
	err = json.Unmarshal(hash, &result)
	if err != nil {
		fmt.Println(err)
	}
	height, err := strconv.ParseUint(result.SyncInfo.LatestBlockHeight, 10, 64)
	return context.WithValue(ctx, chains.LastBlock{}, uint64(height)), err
}

func (c *CosmosTestnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosTestnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("not implemented") // TODO: Implement
}

// Height returns the current block height or an error if unable to get current height.
func (c *CosmosTestnet) Height(ctx context.Context) (uint64, error) {
	var result Result
	hash, err := exec.Command(c.bin, "status", "--node", c.url, "| jq -r '.SyncInfo.latest_block_height'").Output()
	if err != nil {
		fmt.Println(err)
	}
	err = json.Unmarshal(hash, &result)
	if err != nil {
		fmt.Println(err)
	}
	return uint64(0), err
}

func (c *CosmosTestnet) BuildWallets(ctx context.Context, keyName string) error {
	_, err := c.transferDenom(keyName, amount)
	return err
}

func (c *CosmosTestnet) GetExecuteParam(ctx context.Context, methodName, param string) (context.Context, string, error) {
	if strings.Contains(methodName, "set_admin") {
		return c.SetAdminParams(ctx, param)
	}
	return ctx, "", nil
}

func (c *CosmosTestnet) SetAdminParams(ctx context.Context, keyName string) (context.Context, string, error) {
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
		addr, err := c.transferDenom(keyName, amount)
		if err != nil {
			return ctx, "", err
		}
		admin.SetAdmin.Address = addr
		updatedJSON, _ := json.Marshal(admin)
		fmt.Println(string(updatedJSON))
		admins.Admin = map[string]string{
			keyName: addr,
		}
		return context.WithValue(ctx, chains.AdminKey("Admins"), chains.Admins{
			Admin: admins.Admin,
		}), string(updatedJSON), nil
	}
}

func (c *CosmosTestnet) GetQueryParam(method string) Query {
	var queryMsg Query
	if strings.Contains(method, "admin") {
		queryMsg = Query{GetAdmin: &GetAdmin{}}
	} else if strings.Contains(method, "fee") {
		queryMsg = Query{GetProtocolFee: &GetProtocolFee{}}
	}
	return queryMsg
}

func (c *CosmosTestnet) transferDenom(keyName string, amount string) (string, error) {
	// Check if the key is already created
	_, err := exec.Command(c.bin, "keys", "show", keyName).Output()
	if err != nil {
		fmt.Printf("Creating address for %s\n", keyName)
		_, err := exec.Command(c.bin, "keys", "add", keyName).Output()
		if err != nil {
			return "", err
		}
	}
	time.Sleep(2 * time.Second)
	addr, err := exec.Command(c.bin, "keys", "show", keyName, "--address").Output()
	if err != nil {
		return "", err
	}
	address := strings.ReplaceAll(string(addr), "\n", "")
	_, err = exec.Command(c.bin, "tx", "bank", "send", c.keyPassword, address, amount, "--gas", "auto", "--node", c.url, "--chain-id", c.ChainID, "--fees", "200uconst", "-y").Output()
	if err != nil {
		return "", err
	}

	return address, nil
}
