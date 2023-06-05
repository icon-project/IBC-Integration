package e2e_test

import (
	_ "embed"
	"fmt"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/cosmos"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"go.uber.org/zap"
)

// ChainFactory describes how to get chains for tests.
// This type currently supports a Pair method,
// but it may be expanded to a Triplet method in the future.
type ChainFactory interface {
	// Count reports how many chains this factory will produce from its Chains method.
	Count() int

	// Chains returns a set of chains.
	Chains(testName string) ([]ibc.Chain, error)

	// Name returns a descriptive name of the factory,
	// indicating all of its chains.
	// Depending on how the factory was configured,
	// this may report more than two chains.
	// Name() string
}

// BuiltinChainFactory implements ChainFactory to return a fixed set of chains.
// Use NewBuiltinChainFactory to create an instance.
type BuiltinChainFactory struct {
	log *zap.Logger

	specs []*Chain
}

// //  go:embed configuredChains.yaml
// var embeddedConfiguredChains []byte

// var logConfiguredChainsSourceOnce sync.Once

// // initBuiltinChainConfig returns an ibc.ChainConfig mapping all configured chains
// func initBuiltinChainConfig(log *zap.Logger) (map[string]ibc.ChainConfig, error) {
// 	var dat []byte
// 	var err error

// 	// checks if IBCTEST_CONFIGURED_CHAINS environment variable is set with a path,
// 	// otherwise, ./configuredChains.yaml gets embedded and used.
// 	val := os.Getenv("IBCTEST_CONFIGURED_CHAINS")

// 	if val != "" {
// 		dat, err = os.ReadFile(val)
// 		if err != nil {
// 			return nil, err
// 		}
// 	} else {
// 		dat = embeddedConfiguredChains
// 	}

// 	builtinChainConfigs := make(map[string]ibc.ChainConfig)

// 	err = yaml.Unmarshal(dat, &builtinChainConfigs)
// 	if err != nil {
// 		return nil, fmt.Errorf("error unmarshalling pre-configured chains: %w", err)
// 	}

// 	logConfiguredChainsSourceOnce.Do(func() {
// 		if val != "" {
// 			log.Info("Using user specified configured chains", zap.String("file", val))
// 		} else {
// 			log.Info("Using embedded configured chains")
// 		}
// 	})

// 	return builtinChainConfigs, nil
// }

// NewBuiltinChainFactory returns a BuiltinChainFactory that returns chains defined by entries.
func NewBuiltinChainFactory(log *zap.Logger, specs []*Chain) *BuiltinChainFactory {
	return &BuiltinChainFactory{log: log, specs: specs}
}

func (f *BuiltinChainFactory) Count() int {
	return len(f.specs)
}

func (f *BuiltinChainFactory) Chains(testName string) ([]chains.Chain, error) {
	chains := make([]chains.Chain, len(f.specs))
	for i, cfg := range f.specs {
		chain, err := buildChain(f.log, testName, *cfg)
		if err != nil {
			return nil, err
		}
		chains[i] = chain
	}

	return chains, nil
}

func buildChain(log *zap.Logger, testName string, cfg Chain) (chains.Chain, error) {
	switch cfg.ChainConfig.Type {
	case "icon":
		return icon.NewIconLocalnet(testName, log, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystoreFile, cfg.KeystorePassword, cfg.Contracts), nil
	case "cosmos":
		return cosmos.NewCosmosLocalnet(testName, log, cfg.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.KeystorePassword, cfg.Contracts)
	default:
		return nil, fmt.Errorf("unexpected error, unknown chain type: %s for chain: %s", cfg.ChainConfig.Type, cfg.Name)
	}
}

// func (f *BuiltinChainFactory) Name() string {
// 	parts := make([]string, len(f.specs))
// 	for i, s := range f.specs {
// 		// Ignoring error here because if we fail to generate the config,
// 		// another part of the factory stack should have failed properly before we got here.
// 		cfg, _ := s.Config(f.log)

// 		v := s.Version
// 		if v == "" {
// 			v = cfg.Images[0].Version
// 		}

// 		parts[i] = cfg.Name + "@" + v
// 	}
// 	return strings.Join(parts, "+")
// }
