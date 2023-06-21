package interchaintest

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/cosmos"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/icon-project/ibc-integration/test/relayer"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"

	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

type server struct {
	logger *zap.Logger
	chains map[string]chains.Chain
	docker struct {
		networkID string
		client    *client.Client
	}
	interchain *Interchain
	ctx        context.Context
	t          *testing.T
	cfg        map[string]*Chain
}

func (s *server) registerChain(name string, chain chains.Chain) {
	s.chains[name] = chain
}

func (s *server) getChain(name string) chains.Chain {
	return s.chains[name]
}

func (s *server) linkRelayer(image, tag, gid string) {
	s.interchain.AddRelayer(s.setupRelayer(image, tag, gid).(ibc.Relayer), "relayer")
}

func (s *server) linkChain(chainName string) {
	s.interchain.AddChain(s.getChain(chainName).(ibc.Chain))
}

func (s *server) setUpIBC(chain, keyName string) (context.Context, error) {
	contracts1 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return s.getChain(chain).SetupIBC(s.ctx, keyName)
}

func (s *server) buildWallet(chain, keyName string) error {
	return s.getChain(chain).BuildWallets(s.ctx, keyName)
}

func (s *server) addLink(chain1, chain2 string) error {
	s.interchain.AddLink(InterchainLink{
		Chain1:            s.getChain(chain1).(ibc.Chain),
		Chain2:            s.getChain(chain2).(ibc.Chain),
		Relayer:           r,
		Path:              ibcPath,
		CreateChannelOpts: opts,
		CreateClientOpts: ibc.CreateClientOptions{
			TrustingPeriod: "100000m",
		},
	})
}

func (s *server) setupRelayer(image, tag, gid string) ibc.Relayer {
	option := relayer.CustomDockerImage(chains.GetEnvOrDefault(relayerImageEnv, relayerImage), chains.GetEnvOrDefault(relayerImageTagEnv, relayerImageTag), gid)
	return NewICONRelayerFactory(s.logger, option, relayer.ImagePull(false)).Build(s.t, s.docker.client, s.docker.networkID)
}

func (s *server) setDockerClient(client *client.Client, networkID string) {
	s.docker.client = client
	s.docker.networkID = networkID
}

func (s *server) setupConnection(chainName, keyName string) (context.Context, error) {
	chain := s.getChain(chainName)
	return chain.ConfigureBaseConnection(context.Background(), keyName, "channel-0", s.cfg[chainName].NID, contracts2.ContractAddress["connection"])
}

func (s *server) buildChain() {
	for name, cfg := range s.cfg {
		switch cfg.ChainConfig.Type {
		case "icon":
			s.chains[name] = icon.NewIconLocalnet(s.t.Name(), log, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystoreFile, cfg.KeystorePassword, cfg.Contracts)
		case "cosmos", "archway":

			cosmos.NewCosmosLocalnet(s.t.Name(), s.logger, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystorePassword, cfg.Contracts)
		}
	}
}

func setupIBCTest(t *testing.T) {

	cfg, err := GetConfig()
	if err != nil {
		t.Fatal(err)
	}

	var setup = &server{
		ctx:        context.Background(),
		t:          t,
		logger:     zaptest.NewLogger(t),
		chains:     make(map[string]chains.Chain),
		interchain: NewInterchain(),
		cfg:        cfg.mapChains(),
	}

	setup.setDockerClient(DockerSetup(t))

	// Log location
	f, err := CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	if err != nil {
		return
	}
	optionDocker := relayer.CustomDockerImage(chains.GetEnvOrDefault(relayerImageEnv, relayerImage), chains.GetEnvOrDefault(relayerImageTagEnv, relayerImageTag), "100:1000")

	r := NewICONRelayerFactory(logger, optionDocker, relayer.ImagePull(false)).Build(t, client, network)
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	eRep := rep.RelayerExecReporter(t)

	// Build interchain
	opts := ibc.CreateChannelOptions{
		SourcePortName: "mock",
		DestPortName:   "mock",
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}

	const ibcPath = "icon-cosmoshub"

	ic.BuildChains(ctx, eRep, InterchainBuildOptions{
		TestName:          t.Name(),
		Client:            client,
		NetworkID:         s.docker.NetworkID,
		BlockDatabaseFile: DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	)

	contracts1 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	ctx, err = chainB.SetupIBC(ctx, owner)

	contracts2 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	if chainA.(ibc.Chain).Config().Type == "icon" {
		chainA.OverrideConfig("archway-handler-address", contracts2.ContractAddress["ibc"])
	}

	if chainB.(ibc.Chain).Config().Type == "icon" {
		chainB.OverrideConfig("archway-handler-address", contracts1.ContractAddress["ibc"])
	}

	// Start the Relay
	ic.BuildRelayer(ctx, eRep, InterchainBuildOptions{
		TestName:          t.Name(),
		Client:            client,
		NetworkID:         network,
		BlockDatabaseFile: DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	)

	r.StartRelayer(ctx, eRep, ibcPath)
	nid1 := cfg.ChainSpecs[0].ChainConfig.ChainID
	nid2 := cfg.ChainSpecs[1].ChainConfig.ChainID

	// TODO get channel from relay
	_, err = chainA.ConfigureBaseConnection(context.Background(), owner, "channel-0", nid2, contracts2.ContractAddress["connection"])
	_, err = chainB.ConfigureBaseConnection(context.Background(), owner, "channel-0", nid1, contracts1.ContractAddress["connection"])
}

type BuiltinChainFactory struct {
	log *zap.Logger

	specs []*Chain
}

// NewBuiltinChainFactory returns a BuiltinChainFactory that returns chains defined by entries.
func NewBuiltinChainFactory(log *zap.Logger, specs []*Chain) *BuiltinChainFactory {
	return &BuiltinChainFactory{log: log, specs: specs}
}

func (f *BuiltinChainFactory) Count() int {
	return len(f.specs)
}

func (f *BuiltinChainFactory) Chains(testName string) (map[string]chains.Chain, error) {
	chains := make(map[string]chains.Chain, len(f.specs))
	for _, cfg := range f.specs {
		chain, err := buildChain(f.log, testName, *cfg)
		if err != nil {
			return nil, err
		}
		chains[cfg.Name] = chain
	}

	return chains, nil
}

func buildChain(log *zap.Logger, testName string, cfg Chain) (chains.Chain, error) {
	switch cfg.ChainConfig.Type {

	default:
		return nil, fmt.Errorf("unexpected error, unknown chain type: %s for chain: %s", cfg.ChainConfig.Type, cfg.Name)
	}
}
