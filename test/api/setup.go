package api

import (
	"context"
	"fmt"
	"testing"
	"time"

	"github.com/docker/docker/client"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
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
	relayer    struct{ relay ibc.Relayer; reporter ibc.RelayerExecReporter }
	interchain *interchaintest.Interchain
	ctx        context.Context
	t          *testing.T
	cfg        map[string]*Chain
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
	contracts1 := s.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return s.getChain(chain).SetupIBC(s.ctx, keyName)
}

func (s *server) buildWallet(chain, keyName string) error {
	return s.getChain(chain).BuildWallets(s.ctx, keyName)
}

func (s *server) addLink(chain1, chain2, ibcPath string) *interchaintest.Interchain {
	opts := ibc.CreateChannelOptions{
		SourcePortName: "mock",
		DestPortName:   "mock",
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}
	return s.interchain.AddLink(interchaintest.InterchainLink{
		Chain1:            s.getChain(chain1).(ibc.Chain),
		Chain2:            s.getChain(chain2).(ibc.Chain),
		Relayer:           s.relayer,
		Path:              ibcPath,
		CreateChannelOpts: opts,
		CreateClientOpts: ibc.CreateClientOptions{
			TrustingPeriod: "100000m",
		},
	})
}

func (s *server) setupRelayer(image, tag, gid string) {
	option := relayer.CustomDockerImage(image, tag, gid)
	s.relayer = interchaintest.NewICONRelayerFactory(s.logger, option, relayer.ImagePull(false)).Build(s.t, s.docker.client, s.docker.networkID)
}

func (s *server) setDockerClient(client *client.Client, networkID string) {
	s.docker.client = client
	s.docker.networkID = networkID
}

func (s *server) setupConnection(chainName, keyName string) (context.Context, error) {
	chain := s.getChain(chainName)
	return chain.ConfigureBaseConnection(context.Background(), keyName, "channel-0", s.cfg[chainName].NID, contracts2.ContractAddress["connection"])
}

func (s *server) startRelay() {
	// Start the Relay
	s.interchain.BuildRelayer(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:          s.t.Name(),
		Client:            s.docker.client,
		NetworkID:         s.docker.networkID,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
		SkipPathCreation:  false},
	)
	s.relayer.StartRelayer(sctx, eRep, ibcPath)
}

func (s *server) configureLogReporter() *testreporter.RelayerExecReporter {
	f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	if err != nil {
		s.t.Fatal(err)
	}
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	return rep.RelayerExecReporter(s.t)
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
		interchain: interchaintest.NewInterchain(),
		cfg:        cfg.mapChains(),
		relayer: struct{relay ibc.Relayer; reporter ibc.RelayerExecReporter}{ reporter:  },
	}

	setup.setDockerClient(interchaintest.DockerSetup(t))
	setup.setupRelayer("", "", "")

	// Log location

	// Build interchain

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
	ic.BuildRelayer(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:          t.Name(),
		Client:            client,
		NetworkID:         network,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	)

	r.StartRelayer(ctx, eRep, ibcPath)

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
