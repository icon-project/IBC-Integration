package relayer

import (
	dockerclient "github.com/docker/docker/client"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/strangelove-ventures/interchaintest/v8/ibc"

	"testing"

	"github.com/icon-project/ibc-integration/test/relayer"
	"go.uber.org/zap"
)

const (
	rlyRelayerUser = "100:1000"
)

// Config holds configuration values for the relayer used in the tests.
type Config struct {
	// Tag is the tag used for the relayer image.
	Tag string `mapstructure:"tag"`
	// Image is the image that should be used for the relayer.
	Image string `mapstructure:"image"`
}

// New returns an implementation of ibc.Relayer depending on the provided RelayerType.
func New(t *testing.T, cfg Config, logger *zap.Logger, dockerClient *dockerclient.Client, network string) ibc.Relayer {
	optionDocker := relayer.CustomDockerImage(cfg.Image, cfg.Tag, rlyRelayerUser)
	flagOptions := relayer.StartupFlags("-p", "events") // relayer processes via events
	imageOptions := relayer.ImagePull(false)

	relayerFactory := interchaintest.NewICONRelayerFactory(logger, optionDocker, flagOptions, imageOptions)
	return relayerFactory.Build(t, dockerClient, network)
}

// RelayerMap is a mapping from test names to a relayer set for that test.
type RelayerMap map[string]map[ibc.Wallet]bool

// AddRelayer adds the given relayer to the relayer set for the given test name.
func (r RelayerMap) AddRelayer(testName string, relayer ibc.Relayer, chainID string) {
	if _, ok := r[testName]; !ok {
		r[testName] = make(map[ibc.Wallet]bool)
	}
	wallet, exists := relayer.GetWallet(chainID)
	r[testName][wallet] = exists
}

// containsRelayer returns true if the given relayer is in the relayer set for the given test name.
func (r RelayerMap) ContainsRelayer(testName string, wallet ibc.Wallet) bool {
	_, ok := r[testName]
	return ok
}
