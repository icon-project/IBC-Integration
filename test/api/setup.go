package api

import (
	"context"
	"fmt"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"testing"
	"time"

	"github.com/docker/docker/client"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/relayer"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"golang.org/x/sync/errgroup"

	"go.uber.org/zap"
	"go.uber.org/zap/zaptest"
)

type Server struct {
	logger *zap.Logger
	ctx    context.Context
	chains map[string]chains.Chain
	docker struct {
		networkID string
		client    *client.Client
	}
	relayer struct {
		service  ibc.Relayer
		reporter *testreporter.RelayerExecReporter
	}
	interchain *interchaintest.Interchain

	t                     *testing.T
	cfg                   map[string]*Chain
	interchainLinkOption  ibc.CreateChannelOptions
	interchainBuildOption interchaintest.InterchainBuildOptions
	interchainLink        interchaintest.InterchainLink
	ibcPath               string
	contracts             map[string]chains.ContractKey
}

func (s *Server) SetIBCPath(path string) {
	s.ibcPath = path
}

func (s *Server) GetIBCPath(path string) string {
	return s.ibcPath
}

func (s *Server) getChain(name string) chains.Chain {
	return s.chains[name]
}

func NewServer(t *testing.T) *Server {
	cfg, err := GetConfig()
	if err != nil {
		t.Fatal(err)
	}
	logger := zaptest.NewLogger(t)
	interchainLinkOption := ibc.CreateChannelOptions{
		SourcePortName: "mock",
		DestPortName:   "mock",
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}
	cli, networkID := interchaintest.DockerSetup(t)
	docker := struct {
		networkID string
		client    *client.Client
	}{networkID, cli}
	ibcPath := chains.GetEnvOrDefault("IBC_PATH", "icon-cosmoshub")
	relayer := struct {
		service  ibc.Relayer
		reporter *testreporter.RelayerExecReporter
	}{reporter: configureLogReporter(t)}
	return &Server{
		ctx:                  context.Background(),
		t:                    t,
		logger:               logger,
		chains:               cfg.BuildChains(t.Name(), logger),
		interchain:           interchaintest.NewInterchain(),
		cfg:                  cfg.mapChains(),
		interchainLinkOption: interchainLinkOption,
		interchainBuildOption: interchaintest.InterchainBuildOptions{
			TestName:          t.Name(),
			Client:            cli,
			NetworkID:         networkID,
			BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),
			SkipPathCreation:  false,
		},
		interchainLink: interchaintest.InterchainLink{
			CreateChannelOpts: interchainLinkOption,
			Path:              ibcPath,
			Relayer:           relayer.service,
			CreateClientOpts: ibc.CreateClientOptions{
				TrustingPeriod: "100000m",
			},
		},
		docker:    docker,
		contracts: make(map[string]chains.ContractKey),
		ibcPath:   ibcPath,
		relayer:   relayer,
	}
}

func (s *Server) AddRelayer(image, tag, gid string) {
	s.interchain.AddRelayer(s.setupRelayer(image, tag, gid), "relayer")
}

func (s *Server) SetupIBC(chain, keyName string) (context.Context, error) {
	s.contracts[chain] = s.ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	ctx, err := s.getChain(chain).SetupIBC(s.ctx, keyName)
	if err != nil {
		return nil, err
	}
	return s.setupConnection(ctx, chain, keyName)
}

func (s *Server) BuildWallet(chain, keyName string) error {
	return s.getChain(chain).BuildWallets(s.ctx, keyName)
}

func (s *Server) LinkChain(chain1, chain2 string) error {
	chainA := s.getChain(chain1).(ibc.Chain)
	chainB := s.getChain(chain2).(ibc.Chain)
	s.interchain.AddChain(chainA).AddChain(chainB)
	s.interchainLink.Chain1 = chainA
	s.interchainLink.Chain2 = chainB
	s.interchain.AddLink(s.interchainLink)
	return s.interchain.BuildChains(s.ctx, s.relayer.reporter, s.interchainBuildOption)
}

func (s *Server) setupRelayer(image, tag, gid string) ibc.Relayer {
	option := relayer.CustomDockerImage(image, tag, gid)
	relay := interchaintest.NewICONRelayerFactory(s.logger, option, relayer.ImagePull(false)).Build(s.t, s.docker.client, s.docker.networkID)
	s.relayer.service = relay
	return relay
}

func (s *Server) getCounterContractKey(chainName string) chains.ContractKey {
	for key, v := range s.contracts {
		if key != chainName {
			return v
		}
	}
	panic("no counter contract key found")
}

func (s *Server) setupConnection(ctx context.Context, chainName, keyName string) (context.Context, error) {
	chain := s.getChain(chainName)
	return chain.ConfigureBaseConnection(s.ctx, keyName, "channel-0", s.cfg[chainName].NID, s.getCounterContractKey(chainName).ContractAddress["connection"])
}

func (s *Server) StartRelay() error {
	// Start the Relay
	s.interchain.BuildRelayer(s.ctx, s.relayer.reporter, s.interchainBuildOption)
	return s.relayer.service.StartRelayer(s.ctx, s.relayer.reporter, s.ibcPath)
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

func (s *Server) overrideConfig() {
	for chainName, chain := range s.chains {
		if chain.(ibc.Chain).Config().Type == "icon" {
			chain.OverrideConfig("archway-handler-address", s.getCounterContractKey(chainName).ContractAddress["ibc"])
		}
	}
}

func (s *Server) Serve(mux *http.ServeMux) func() error {
	ctx, cancel := context.WithCancel(s.ctx)
	go func() {
		c := make(chan os.Signal, syscall.SIGTERM)
		signal.Notify(c, os.Interrupt, syscall.SIGTERM)
		<-c
		cancel()
	}()
	g, gCtx := errgroup.WithContext(ctx)
	server := &http.Server{Addr: ":8080", Handler: mux, ReadTimeout: 10 * time.Second, WriteTimeout: 10 * time.Second}
	g.Go(func() error {
		<-gCtx.Done()
		return server.Shutdown(context.Background())
	})
	g.Go(func() error {
		return server.ListenAndServe()
	})
	return func() error {
		if err := g.Wait(); err != nil {
			s.relayer.service.StopRelayer(s.ctx, s.relayer.reporter)
			return fmt.Errorf("Server stopped, reason: %s", err)
		}
		return nil
	}
}
