package icon

import (
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os/exec"
	"path"
	"strings"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
)

type IconTestnet struct {
	bin              string
	nid              string
	keystorePath     string
	keyPassword      string
	scorePaths       map[string]string
	defaultStepLimit string
	url              string
}

func (it *IconTestnet) DeployXCallMockApp(ctx context.Context, connection chains.XCallConnection) error {
	//TODO implement me
	panic("implement me")
}

func (it *IconTestnet) GetIBCAddress(key string) string {
	//TODO implement me
	panic("implement me")
}

func (it *IconTestnet) SetupXCall(ctx context.Context, portId, keyName string) error {
	//TODO implement me
	panic("implement me")
}

type Block struct {
	Height int64
}

type Wallet struct {
	Address  string `json:"address"`
	ID       string `json:"id"`
	Version  int    `json:"version"`
	CoinType string `json:"coinType"`
	Crypto   struct {
		Cipher       string `json:"cipher"`
		Cipherparams struct {
			Iv string `json:"iv"`
		} `json:"cipherparams"`
		Ciphertext string `json:"ciphertext"`
		Kdf        string `json:"kdf"`
		Kdfparams  struct {
			Dklen int    `json:"dklen"`
			N     int    `json:"n"`
			R     int    `json:"r"`
			P     int    `json:"p"`
			Salt  string `json:"salt"`
		} `json:"kdfparams"`
		Mac string `json:"mac"`
	} `json:"crypto"`
}

func (it *IconTestnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	panic("unimplemented")
}

func (it *IconTestnet) OverrideConfig(keyName string, value any) {
	panic("unimplemented")
}
func (it *IconTestnet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	panic("unimplemented")
}
func (it *IconTestnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data, rollback []byte) (string, string, error) {
	panic("unimplemented")
}
func (it *IconTestnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, error) {
	panic("unimplemented")
}
func (it *IconTestnet) ExecuteCall(ctx context.Context, reqId string) (context.Context, error) {
	panic("unimplemented")
}

func (it *IconTestnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	panic("unimplemented")
}
func (it *IconTestnet) FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, error) {
	panic("unimplemented")
}

// DeployContract implements chains.Chain
func (it *IconTestnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	var result *types.TransactionResult
	var output string

	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg

	if contractName == "xcall" {
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		bmcAddr := ctxValue.ContractAddress["bmc"]
		initMessage = initMessage + bmcAddr
	}

	keyStorePath, address := it.GetKeyStorePathAndAddress(ctx, keyName)

	// Deploy Contract
	hash, err := exec.Command(it.bin, "rpc", "sendtx", "deploy", it.scorePaths[contractName], "--param", initMessage,
		"--key_store", keyStorePath, "--key_password", keyName, "--step_limit", it.defaultStepLimit,
		"--content_type", "application/java",
		"--uri", it.url, "--nid", it.nid).Output()
	if err != nil {
		return nil, err
	}
	json.Unmarshal(hash, &output)
	time.Sleep(5 * time.Second)

	out, err := exec.Command(it.bin, "rpc", "txresult", output, "--uri", it.url).Output()
	if err != nil {
		return nil, err
	}
	json.Unmarshal(out, &result)
	var contracts chains.ContractKey

	contracts.ContractAddress = map[string]string{
		contractName: string(result.SCOREAddress),
	}

	// TODO: map keyname to their address
	contracts.ContractOwner = map[string]string{
		keyName: address,
	}

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

// ExecuteContract implements chains.Chain
func (it *IconTestnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	var hash string
	keyStorePath, addr := it.GetKeyStorePathAndAddress(ctx, keyName)
	ctx, methodName, param = it.GetExecuteParam(ctx, methodName, param, addr)
	output, err := exec.Command(it.bin, "rpc", "sendtx", "call", "--to", contractAddress, "--method", methodName, "--param", param, "--key_store", keyStorePath,
		"--key_password", keyName, "--step_limit", "5000000000", "--uri", it.url, "--nid", it.nid).Output()
	json.Unmarshal(output, &hash)
	return ctx, err
}

// GetBalance implements chains.Chain
func (*IconTestnet) GetBalance(ctx context.Context, address string, denom string) (int64, error) {
	panic("unimplemented")
}

// GetBlockByHeight implements chains.Chain
func (*IconTestnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetLastBlock implements chains.Chain
func (it *IconTestnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	var res icontypes.Block
	out, err := exec.Command(it.bin, "rpc", "lastblock", "--uri", it.url).Output()
	json.Unmarshal(out, &res)
	return context.WithValue(ctx, chains.LastBlock{}, uint64(res.Height)), err
}

// QueryContract implements chains.Chain
func (it *IconTestnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	time.Sleep(2 * time.Second)

	queryMethodName, queryParam := it.GetQueryParam(methodName)
	if params != "" {
		output, _ := exec.Command(it.bin, "rpc", "call", "--to", contractAddress, "--method", queryMethodName, "--param", queryParam, "--uri", it.url).Output()
		fmt.Println(output)
		chains.Response = output
		return ctx, nil
	} else {
		output, _ := exec.Command(it.bin, "rpc", "call", "--to", contractAddress, "--method", queryMethodName, "--uri", it.url).Output()
		fmt.Println(string(output))
		chains.Response = string(output)
		return ctx, nil
	}
}

func (*IconTestnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("unimplemented")
}

func NewIconTestnet(bin, nid, keystorePath, keyPassword, defaultStepLimit, url string, scorePaths map[string]string) chains.Chain {
	return &IconTestnet{
		bin:              bin,
		nid:              nid,
		keystorePath:     keystorePath,
		keyPassword:      keyPassword,
		scorePaths:       scorePaths,
		defaultStepLimit: defaultStepLimit,
		url:              url,
	}
}

func KeyStorePath(keyName string) string {
	home, _ := exec.Command("sh", "-c", "echo $HOME").Output()
	homedir := strings.ReplaceAll(string(home), "\n", "")
	jsonFile := keyName + ".json"
	keyStorePath := path.Join(homedir, "icontestnet", jsonFile)
	return keyStorePath
}

func (it *IconTestnet) CreateKey(ctx context.Context, keyName string) error {
	keyStorePath := KeyStorePath(keyName)
	_, err := exec.Command(it.bin, "ks", "gen", "--password", keyName, "--out", keyStorePath).Output()
	return err
}

func (it *IconTestnet) BuildWallets(ctx context.Context, keyName string) error {
	return nil
}

func (it *IconTestnet) GetKeyStorePathAndAddress(ctx context.Context, keyName string) (keyStorePath, address string) {
	// check if keystore already exists
	path := KeyStorePath(keyName)
	it.checkIfKeyExists(ctx, path, keyName)
	addr := GetWalletAddress(path)
	// Transfer some funds
	_, err := exec.Command(it.bin, "rpc", "sendtx", "transfer", "--key_store", it.keystorePath, "--key_password", it.keyPassword, "--to", addr, "--value", "100000000000000000000", "--nid", it.nid,
		"--uri", it.url, "--step_limit", it.defaultStepLimit).Output()
	if err != nil {
		fmt.Println(err)
		return "", ""
	}
	return path, addr
}

func (it *IconTestnet) checkIfKeyExists(ctx context.Context, keyStorePath, keyName string) error {
	_, err := exec.Command("cat", keyStorePath).Output()
	if err != nil {
		return it.CreateKey(ctx, keyName)
	}
	return nil
}
func GetWalletAddress(keyStorePath string) string {
	var walletInfo Wallet
	wallet, _ := ioutil.ReadFile(keyStorePath)
	json.Unmarshal(wallet, &walletInfo)
	addr := walletInfo.Address
	return addr
}

func (it *IconTestnet) GetExecuteParam(ctx context.Context, methodName, params, addr string) (context.Context, string, string) {
	if strings.Contains(methodName, "set_admin") {
		return it.SetAdminParams(ctx, methodName, params, addr)
	} else if strings.Contains(methodName, "update_admin") {
		// TODO: update admin method is not found
		return ctx, "update_admin", ""
	} else if strings.Contains(methodName, "remove_admin") {
		// TODO: remove admin method is not found
		return ctx, "remove_admin", "_address='hjsdbjd'"
	}
	return ctx, methodName, params
}

func (c *IconTestnet) SetAdminParams(ctx context.Context, methodaName, keyName, addr string) (context.Context, string, string) {
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

func (it *IconTestnet) GetQueryParam(method string) (methodName, params string) {
	if strings.Contains(method, "get_admin") {
		return "admin", ""
	}
	return "", ""
}
