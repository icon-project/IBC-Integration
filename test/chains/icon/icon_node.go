package icon

import (
	"context"
	"fmt"
	"path"
	"sync"

	"github.com/docker/docker/api/types"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/docker/api/types/network"
	dockerclient "github.com/docker/docker/client"
	"github.com/docker/go-connections/nat"
	"github.com/icon-project/IBC-Integration/test/internal/dockerutil"
	iconclient "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon"
	iconlog "github.com/icon-project/icon-bridge/common/log"
	"github.com/strangelove-ventures/ibctest/ibc"
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
						HostIP:   "127.0.0.1",
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
