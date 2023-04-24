package icon

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
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
	initMessage      string
}

type ContractInfo struct {
	name         string
	ScoreAddress string
	// any neccessary info
}

type Block struct {
	Height int64
}

// DeployContract implements chains.Chain
func (it *IconTestnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	var result *types.TransactionResult
	var output string

	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName

	hash, err := exec.Command(it.bin, "rpc", "sendtx", "deploy", it.scorePaths[contractName], "--param", it.initMessage,
		"--key_store", it.keystorePath, "--key_password", it.keyPassword, "--step_limit", it.defaultStepLimit,
		"--content_type", "application/java",
		"--uri", it.url, "--nid", it.nid).Output()
	if err != nil {
		fmt.Println(err)
	}
	json.Unmarshal(hash, &output)
	time.Sleep(3 * time.Second)

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
	contracts.ContractAddress = map[string]string{
		"keyname": "Address",
	}

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractAddress,
	}), err
}

// ExecuteContract implements chains.Chain
func (*IconTestnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	var hash string
	output, err := exec.Command("it.Config.Bin", "rpc", "sendtx", "call", "--to", "scoreAddress", "--method", methodName, "--key_store", "keystorePath",
		"--key_password", "gochain", "--step_limit", "5000000000", "--param", "params").Output()
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
	if params != "" {
		output, _ := exec.Command("it.Config.Bin", "rpc", "call", "--to", "scoreAddress", "--method", methodName, "--param", params, "--uri", "it.Config.URL").Output()
		fmt.Println(output)
		return ctx, nil
	} else {
		output, _ := exec.Command("it.Config.Bin", "rpc", "call", "--to", "scoreAddress", "--method", methodName, "--uri", "it.Config.URL").Output()
		fmt.Println(output)
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

func (it *IconTestnet) SetAdminParams(ctx context.Context) string {
	panic("unimplemented")
}

func (it *IconTestnet) CreateKey(ctx context.Context, keyName string) error {
	panic("unimplemented")
}

func (it *IconTestnet) BuildWallets(ctx context.Context, keyName string) error {
	panic("unimplemented")
}
