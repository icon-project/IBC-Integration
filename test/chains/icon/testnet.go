package icon

import (
	"encoding/json"
	"os/exec"
	"time"

	integration_test "github.com/icon-project/ibc-integration/test/integration"
	iconclient "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon"
	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
)

type IconTestnet struct {
	Config integration_test.Config
	Client *iconclient.Client
}

// This function takes initMessage, scorePath and keytorePath, Deploys contract to testnet and returns score address
func (it *IconTestnet) DeployContract(scorePath, keystorePath, initMessage string) (string, error) {
	var result *icontypes.TransactionResult
	var output string
	hash, _ := exec.Command(it.Config.Bin, "rpc", "sendtx", "deploy", scorePath,
		"--key_store", keystorePath, "--key_password", "gochain", "--step_limit", "5000000000",
		"--content_type", "application/java", "--param", initMessage,
		"--uri", it.Config.URL, "--nid", it.Config.NID).Output()

	json.Unmarshal(hash, &output)
	time.Sleep(3 * time.Second)

	// To get score address
	out, err := exec.Command(it.Config.Bin, "rpc", "txresult", output, "--uri", it.Config.URL).Output()
	json.Unmarshal(out, &result)
	return string(result.SCOREAddress), err
}

// This function queries any method in deployed smartcontract given score address, method name along with params if any, to return the result
func (it *IconTestnet) QueryContract(scoreAddress, methodName, params string) (string, error) {
	if params != "" {
		output, _ := exec.Command(it.Config.Bin, "rpc", "call", "--to", scoreAddress, "--method", methodName, "--param", params, "--uri", it.Config.URL).Output()
		return string(output), nil
	} else {
		output, _ := exec.Command(it.Config.Bin, "rpc", "call", "--to", scoreAddress, "--method", methodName, "--uri", it.Config.URL).Output()
		return string(output), nil
	}
}

// This function takes method name and params along with score address and keystore path to execute any method in contract that is already deployed
func (it *IconTestnet) ExecuteContract(scoreAddress, keystorePath, methodName, params string) (string, error) {
	var hash string
	output, err := exec.Command(it.Config.Bin, "rpc", "sendtx", "call", "--to", scoreAddress, "--method", methodName, "--key_store", keystorePath,
		"--key_password", "gochain", "--step_limit", "5000000000", "--param", params).Output()
	json.Unmarshal(output, &hash)
	return hash, err
}
