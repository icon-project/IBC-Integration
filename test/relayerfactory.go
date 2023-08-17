package interchaintest

import (
	"context"
	"testing"

	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/relayer"
	iconRelayer "github.com/icon-project/ibc-integration/test/relayer/icon"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"go.uber.org/zap"
)

// RelayerFactory describes how to start a Relayer.
type RelayerFactory interface {
	// Build returns a Relayer associated with the given arguments.
	Build(t *testing.T, cli *client.Client, networkID string) ibc.Relayer

	// Name returns a descriptive name of the factory,
	// indicating details of the Relayer that will be built.
	Name() string

	// Capabilities is an indication of the features this relayer supports.
	// Tests for any unsupported features will be skipped rather than failed.
	Capabilities() map[relayer.Capability]bool
}

type Relayer interface {
	ibc.Relayer
	RestartRelayerContainer(context.Context) error
	StopRelayerContainer(context.Context, ibc.RelayerExecReporter) error
	WriteBlockHeight(context.Context, string, uint64) error
}

var _ Relayer = (*relayer.DockerRelayer)(nil)

// builtinRelayerFactory is the built-in relayer factory that understands
// how to start the cosmos relayer in a docker container.
type builtinRelayerFactory struct {
	log     *zap.Logger
	options relayer.RelayerOptions
}

func NewICONRelayerFactory(logger *zap.Logger, options ...relayer.RelayerOption) RelayerFactory {
	return builtinRelayerFactory{log: logger, options: options}
}

// Build returns a relayer chosen depending on f.impl.
func (f builtinRelayerFactory) Build(t *testing.T, cli *client.Client, networkID string) ibc.Relayer {
	return iconRelayer.NewICONRelayer(f.log, t.Name(), cli, networkID, f.options...)
}

func (f builtinRelayerFactory) Name() string {
	return "iconRelayer@"
}

// Capabilities returns the set of capabilities for the
// relayer implementation backing this factory.
func (f builtinRelayerFactory) Capabilities() map[relayer.Capability]bool {
	return iconRelayer.Capabilities()
}
