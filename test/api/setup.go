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
	relayer struct {
		relay    ibc.Relayer
		reporter *testreporter.RelayerExecReporter
	}
	interchain *interchaintest.Interchain
	ctx        context.Context
	t          *testing.T
	cfg        map[string]*Chain
	ibcPath    string
	contracts  map[string]chains.ContractKey
}

func (s *server) getChain(name string) chains.Chain {
	return s.chains[name]
}

var setup *server

func NewServer(t *testing.T) *server {
	cfg, err := GetConfig()
	if err != nil {
		t.Fatal(err)
	}

	return &server{
		ctx:        context.Background(),
		t:          t,
		logger:     zaptest.NewLogger(t),
		chains:     make(map[string]chains.Chain),
		interchain: interchaintest.NewInterchain(),
		cfg:        cfg.mapChains(),
		contracts:  make(map[string]chains.ContractKey),
		ibcPath:    "icon-cosmoshub",
		relayer: struct {
			relay    ibc.Relayer
			reporter *testreporter.RelayerExecReporter
		}{reporter: configureLogReporter(t)},
	}
}

func (s *server) linkRelayer(image, tag, gid string) {
	s.interchain.AddRelayer(s.setupRelayer(image, tag, gid).(ibc.Relayer), "relayer")
}

func (s *server) linkChain(chainName string) {
	s.interchain.AddChain(s.getChain(chainName).(ibc.Chain))
}

func (s *server) setUpIBC(chain, keyName string) (context.Context, error) {
	s.contracts[chain] = s.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	return s.getChain(chain).SetupIBC(s.ctx, keyName)
}

func (s *server) buildWallet(chain, keyName string) error {
	return s.getChain(chain).BuildWallets(s.ctx, keyName)
}

func (s *server) addLink(chain1, chain2 string) *interchaintest.Interchain {
	opts := ibc.CreateChannelOptions{
		SourcePortName: "mock",
		DestPortName:   "mock",
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}
	return s.interchain.AddLink(interchaintest.InterchainLink{
		Chain1:            s.getChain(chain1).(ibc.Chain),
		Chain2:            s.getChain(chain2).(ibc.Chain),
		Relayer:           s.relayer.relay,
		Path:              s.ibcPath,
		CreateChannelOpts: opts,
		CreateClientOpts: ibc.CreateClientOptions{
			TrustingPeriod: "100000m",
		},
	})
}

func (s *server) setupRelayer(image, tag, gid string) ibc.Relayer {
	option := relayer.CustomDockerImage(image, tag, gid)
	relay := interchaintest.NewICONRelayerFactory(s.logger, option, relayer.ImagePull(false)).Build(s.t, s.docker.client, s.docker.networkID)
	s.relayer.relay = relay
	return relay
}

func (s *server) setDockerClient(client *client.Client, networkID string) {
	s.docker.client = client
	s.docker.networkID = networkID
}

func (s *server) getCounterContractKey(chainName string) chains.ContractKey {
	for key, v := range s.contracts {
		if key != chainName {
			return v
		}
	}
	panic("no counter contract key found")
}

func (s *server) setupConnection(chainName, keyName string) (context.Context, error) {
	chain := s.getChain(chainName)
	return chain.ConfigureBaseConnection(s.ctx, keyName, "channel-0", s.cfg[chainName].NID, s.getCounterContractKey(chainName).ContractAddress["connection"])
}

func (s *server) startRelay() error {
	// Start the Relay
	s.interchain.BuildRelayer(s.ctx, s.relayer.reporter, interchaintest.InterchainBuildOptions{
		TestName:          s.t.Name(),
		Client:            s.docker.client,
		NetworkID:         s.docker.networkID,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
		SkipPathCreation:  false},
	)
	return s.relayer.relay.StartRelayer(s.ctx, s.relayer.reporter, s.ibcPath)
}

func configureLogReporter(t *testing.T) *testreporter.RelayerExecReporter {
	f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	if err != nil {
		t.Fatal(err)
	}
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	return rep.RelayerExecReporter(t)
}

func (s *server) overrideConfig() {
	for chainName, chain := range s.chains {
		if chain.(ibc.Chain).Config().Type == "icon" {
			chain.OverrideConfig("archway-handler-address", s.getCounterContractKey(chainName).ContractAddress["ibc"])
		}
	}
}

func setupIBCTest(t *testing.T) {

	setup.setDockerClient(interchaintest.DockerSetup(t))
	setup.setupRelayer("", "", "")
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
