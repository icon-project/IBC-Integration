package icon

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
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

type ContractInfo struct {
	name         string
	scoreAddress string
	// any neccessary info
}

// DeployContract implements chains.Chain
func (it *IconTestnet) DeployContract(ctx context.Context) (context.Context, error) {
	var result *types.TransactionResult
	var output string

	// Build Params
	// "--param", initMessage

	contract := ctx.Value(chains.ContractKey{}).(string)
	contractPath := it.scorePaths[contract]
	if contract == "" {
		return nil, fmt.Errorf("cannot find contract %v in config", contract)
	}

	hash, _ := exec.Command(it.bin, "rpc", "sendtx", "deploy", contractPath,
		"--key_store", it.keystorePath, "--key_password", it.keyPassword, "--step_limit", it.defaultStepLimit,
		"--content_type", "application/java",
		"--uri", it.url, "--nid", it.nid).Output()

	json.Unmarshal(hash, &output)
	time.Sleep(3 * time.Second)

	out, err := exec.Command(it.bin, "rpc", "txresult", output, "--uri", it.url).Output()
	if err != nil {
		return nil, err
	}

	json.Unmarshal(out, &result)

	return context.WithValue(ctx, ContractInfo{}, ContractInfo{
		scoreAddress: string(result.SCOREAddress),
	}), nil
}

// ExecuteContract implements chains.Chain
func (*IconTestnet) ExecuteContract(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetBalance implements chains.Chain
func (*IconTestnet) GetBalance(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetBlockByHeight implements chains.Chain
func (*IconTestnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetLastBlock implements chains.Chain
func (*IconTestnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// QueryContract implements chains.Chain
func (*IconTestnet) QueryContract(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

func NewIconTestnet(bin, nid, keystorePath, keyPassword, defaultStepLimit, url string, scorePaths map[string]string) chains.Chain {
	return &IconTestnet{
		bin,
		nid,
		keystorePath,
		keyPassword,
		scorePaths,
		defaultStepLimit,
		url,
	}
}

// // This function queries any method in deployed smartcontract given score address, method name along with params if any, to return the result
// func (it *Testnet) QueryContract(scoreAddress, methodName, params string) (string, error) {
// 	if params != "" {
// 		output, _ := exec.Command(it.Config.Bin, "rpc", "call", "--to", scoreAddress, "--method", methodName, "--param", params, "--uri", it.Config.URL).Output()
// 		return string(output), nil
// 	} else {
// 		output, _ := exec.Command(it.Config.Bin, "rpc", "call", "--to", scoreAddress, "--method", methodName, "--uri", it.Config.URL).Output()
// 		return string(output), nil
// 	}
// }

// // This function takes method name and params along with score address and keystore path to execute any method in contract that is already deployed
// func (it *Testnet) ExecuteContract(scoreAddress, keystorePath, methodName, params string) (string, error) {
// 	var hash string
// 	output, err := exec.Command(it.Config.Bin, "rpc", "sendtx", "call", "--to", scoreAddress, "--method", methodName, "--key_store", keystorePath,
// 		"--key_password", "gochain", "--step_limit", "5000000000", "--param", params).Output()
// 	json.Unmarshal(output, &hash)
// 	return hash, err
// }
