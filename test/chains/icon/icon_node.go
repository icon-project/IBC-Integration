package icon

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path"
	"path/filepath"
	"sync"
	"time"

	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/api/types/network"
	dockerclient "github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/icon-project/IBC-Integration/test/internal/blockdb"
	"github.com/icon-project/IBC-Integration/test/internal/dockerutil"
	iconclient "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon"
	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
	iconlog "github.com/icon-project/icon-bridge/common/log"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
	"go.uber.org/zap"
)

const (
	rpcPort = "9080/tcp"
)

type IconNode struct {
	VolumeName   string
	Index        int
	Chain        ibc.Chain
	NetworkID    string
	DockerClient *dockerclient.Client
	Client       iconclient.Client
	TestName     string
	Image        ibc.DockerImage
	log          *zap.Logger
	ContainerID  string
	// Ports set during StartContainer.
	HostRPCPort string
	Validator   bool
	lock        sync.Mutex
	Address     string
}

type IconNodes []*IconNode

// Name of the test node container
func (in *IconNode) Name() string {
	var nodeType string
	if in.Validator {
		nodeType = "val"
	} else {
		nodeType = "fn"
	}
	return fmt.Sprintf("%s-%s-%d-%s", in.Chain.Config().ChainID, nodeType, in.Index, dockerutil.SanitizeContainerName(in.TestName))
}

// Create Node Container with ports exposed and published for host to communicate with
func (in *IconNode) CreateNodeContainer(ctx context.Context) error {
	imageRef := in.Image.Ref()
	containerConfig := &types.ContainerCreateConfig{
		Config: &container.Config{
			Image: imageRef,
			ExposedPorts: nat.PortSet{
				"8080/tcp": {},
				"9080/tcp": {},
			},
			Hostname: in.HostName(),

			Labels: map[string]string{dockerutil.CleanupLabel: in.TestName},
		},
		HostConfig: &container.HostConfig{
			Binds:           in.Bind(),
			PublishAllPorts: true,
			AutoRemove:      false,
			DNS:             []string{},
			PortBindings: nat.PortMap{
				"9080/tcp": {
					nat.PortBinding{
						HostIP:   "172.17.0.1",
						HostPort: "9080",
					},
				},
			},
		},
		NetworkingConfig: &network.NetworkingConfig{
			EndpointsConfig: map[string]*network.EndpointSettings{
				in.NetworkID: {},
			},
		},
	}
	cc, err := in.DockerClient.ContainerCreate(ctx, containerConfig.Config, containerConfig.HostConfig, containerConfig.NetworkingConfig, nil, in.Name())
	if err != nil {
		panic(err)
	}
	if err != nil {
		return err
	}
	in.ContainerID = cc.ID
	return nil

}

func (in *IconNode) HostName() string {
	return dockerutil.CondenseHostName(in.Name())
}

func (in *IconNode) Bind() []string {
	return []string{fmt.Sprintf("%s:%s", in.VolumeName, in.HomeDir())}
}

func (in *IconNode) HomeDir() string {
	return path.Join("/var/icon-chain", in.Chain.Config().Name)
}

func (in *IconNode) StartContainer(ctx context.Context) error {
	if err := dockerutil.StartContainer(ctx, in.DockerClient, in.ContainerID); err != nil {
		return err
	}

	c, err := in.DockerClient.ContainerInspect(ctx, in.ContainerID)
	if err != nil {
		return err
	}
	in.HostRPCPort = dockerutil.GetHostPort(c, rpcPort)
	in.logger().Info("Icon chain node started", zap.String("container", in.Name()), zap.String("rpc_port", in.HostRPCPort))

	uri := "http://" + in.HostRPCPort + "/api/v3"
	var l iconlog.Logger
	in.Client = *iconclient.NewClient(uri, l)
	return nil
}

func (in *IconNode) logger() *zap.Logger {
	return in.log.With(
		zap.String("chain_id", in.Chain.Config().ChainID),
		zap.String("test", in.TestName),
	)
}

func (in *IconNode) Exec(ctx context.Context, cmd []string, env []string) ([]byte, []byte, error) {
	job := dockerutil.NewImage(in.logger(), in.DockerClient, in.NetworkID, in.TestName, in.Image.Repository, in.Image.Version)
	opts := dockerutil.ContainerOptions{
		Env:   env,
		Binds: in.Bind(),
	}
	res := job.Run(ctx, cmd, opts)
	return res.Stdout, res.Stderr, res.Err
}

func (in *IconNode) BinCommand(command ...string) []string {
	command = append([]string{in.Chain.Config().Bin}, command...)
	return command
}

func (in *IconNode) ExecBin(ctx context.Context, command ...string) ([]byte, []byte, error) {
	return in.Exec(ctx, in.BinCommand(command...), nil)
}

func (in *IconNode) GetBlockByHeight(ctx context.Context, height int64) (string, error) {
	in.lock.Lock()
	defer in.lock.Unlock()
	uri := "http://" + in.HostRPCPort + "/api/v3"
	block, _, err := in.ExecBin(ctx,
		"rpc", "blockbyheight", fmt.Sprint(height),
		"--uri", uri,
	)
	return string(block), err
}

func (in *IconNode) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	var flag = true
	if flag {
		time.Sleep(3 * time.Second)
		flag = false
	}

	time.Sleep(2 * time.Second)
	blockHeight := icontypes.BlockHeightParam{Height: icontypes.NewHexInt(int64(height))}
	res, _ := in.Client.GetBlockByHeight(&blockHeight)

	txs := make([]blockdb.Tx, 0, len(res.NormalTransactions)+2)
	var newTx blockdb.Tx
	for _, tx := range res.NormalTransactions {
		newTx.Data = []byte(fmt.Sprintf(`{"data":"%s"}`, tx.Data))
	}

	// ToDo Add events from block if any to newTx.Events.
	// Event is an alternative representation of tendermint/abci/types.Event
	return txs, nil
}

func (in *IconNode) Height(ctx context.Context) (uint64, error) {
	res, err := in.Client.GetLastBlock()
	return uint64(res.Height), err
}

func (in *IconNode) GetBalance(ctx context.Context, address string) (int64, error) {
	addr := icontypes.AddressParam{Address: icontypes.Address(address)}
	bal, _ := in.Client.GetBalance(&addr)
	return bal.Int64(), nil
}

func (in *IconNode) DeployContract(ctx context.Context, scorePath, keystorePath, initMessage string) (string, error) {
	var result icontypes.TransactionResult

	// Write Contract file to Docker volume
	_, score := filepath.Split(scorePath)
	err := in.CopyFile(ctx, scorePath, score)
	if err != nil {
		return "", fmt.Errorf("error copying keystore to Docker volume: %w", err)
	}

	// Deploy the contract
	hash, err := in.ExecTx(ctx, initMessage, path.Join(in.HomeDir(), score), keystorePath)
	if err != nil {
		return "", err
	}
	uri := "http://" + in.HostRPCPort + "/api/v3"

	//wait for few blocks
	time.Sleep(3 * time.Second)

	// Get Score Address
	out, _, err := in.ExecBin(ctx, "rpc", "txresult", hash, "--uri", uri)
	json.Unmarshal(out, &result)
	scoreAddress := result.SCOREAddress
	return string(scoreAddress), err

}

// ExecTx executes a transaction, waits for 2 blocks if successful, then returns the tx hash.
func (in *IconNode) ExecTx(ctx context.Context, initMessage string, filePath string, keystorePath string, command ...string) (string, error) {
	var output string
	in.lock.Lock()
	defer in.lock.Unlock()
	stdout, _, err := in.Exec(ctx, in.TxCommand(ctx, initMessage, filePath, keystorePath, command...), nil)
	if err != nil {
		return "", err
	}
	json.Unmarshal(stdout, &output)
	return output, nil
}

// TxCommand is a helper to retrieve a full command for broadcasting a tx
// with the chain node binary.
func (in *IconNode) TxCommand(ctx context.Context, initMessage, filePath, keystorePath string, command ...string) []string {
	// Write keystore file to Docker volume
	_, key := filepath.Split(keystorePath)
	err := in.CopyFile(ctx, keystorePath, key)
	if err != nil {
		return []string{"error copying keystore to Docker volume"}
	}
	keystore := path.Join(in.HomeDir(), key)

	command = append([]string{"rpc", "sendtx", "deploy", filePath}, command...)
	return in.NodeCommand(append(command,
		"--key_store", keystore,
		"--key_password", "gochain",
		"--step_limit", "5000000000",
		"--content_type", "application/java",
		"--param", initMessage,
	)...)
}

// NodeCommand is a helper to retrieve a full command for a chain node binary.
// when interactions with the RPC endpoint are necessary.
// For example, if chain node binary is `gaiad`, and desired command is `gaiad keys show key1`,
// pass ("keys", "show", "key1") for command to return the full command.
// Will include additional flags for node URL, home directory, and chain ID.
func (in *IconNode) NodeCommand(command ...string) []string {
	command = in.BinCommand(command...)
	return append(command,
		"--uri", fmt.Sprintf("http://%s/api/v3", in.HostRPCPort),
		"--nid", "0xc5addf",
	)
}

// CopyFile adds a file from the host filesystem to the docker filesystem
// relPath describes the location of the file in the docker volume relative to
// the home directory
func (tn *IconNode) CopyFile(ctx context.Context, srcPath, dstPath string) error {
	content, err := os.ReadFile(srcPath)
	if err != nil {
		return err
	}
	return tn.WriteFile(ctx, content, dstPath)
}

// WriteFile accepts file contents in a byte slice and writes the contents to
// the docker filesystem. relPath describes the location of the file in the
// docker volume relative to the home directory
func (tn *IconNode) WriteFile(ctx context.Context, content []byte, relPath string) error {
	fw := dockerutil.NewFileWriter(tn.logger(), tn.DockerClient, tn.TestName)
	return fw.WriteFile(ctx, tn.VolumeName, relPath, content)
}
