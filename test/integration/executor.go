package integration_test

import (
	"context"
	"testing"

	"github.com/docker/docker/client"
	"github.com/strangelove-ventures/interchaintest/v6"
	"github.com/strangelove-ventures/interchaintest/v6/ibc"
)

type Executor struct {
	chain ibc.Chain
	*testing.T
	ic      *interchaintest.Interchain
	network string
	client  *client.Client
	ctx     context.Context
	*Config
}

func NewExecutor(t *testing.T) *Executor {
	return &Executor{
		T:   t,
		ctx: context.Background(),
		Config: GetConfig(),
	}
}
