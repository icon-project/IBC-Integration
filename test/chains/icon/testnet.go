package icon

import (
	"os/exec"

	"github.com/icon-project/ibc-integration/test/integration"
	iconclient "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon"
)

type IconTestnet struct {
	Config integration.Config
	Client *iconclient.Client
}

func (it IconTestnet) DeployContract(scorePath, keystorePath, initMessage string) (string, error) {
	byt, err := exec.Command(it.Config.Bin, "rpc", "sendtx", "deploy", scorePath,
		"--key_store", keystorePath, "--key_password", "gochain", "--step_limit", "5000000000",
		"--content_type", "application/java", "--param", initMessage,
		"--uri", it.Config.URL, "--nid", it.Config.NID).Output()
	return string(byt), err
}
