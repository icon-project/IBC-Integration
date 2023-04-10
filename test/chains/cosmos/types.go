package cosmos

import (
	"testing"
	"time"

	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	rpcclient "github.com/tendermint/tendermint/rpc/client"
)

type CosmosLocalnet struct {
	*cosmos.CosmosChain
	keyName  string
	filepath map[string]string
	t        *testing.T
}

type Query struct {
	GetAdmin       *GetAdmin       `json:"get_admin,omitempty"`
	GetProtocolFee *GetProtocolFee `json:"get_protocol_fee,omitempty"`
}

type SetAdmin struct {
	SetAdmin struct {
		Address string `json:"address"`
	} `json:"set_admin"`
}

type UpdateAdmin struct {
	UpdateAdmin struct {
		Address string `json:"address"`
	} `json:"update_admin"`
}

type GetProtocolFee struct{}

type GetAdmin struct{}

type CosmosTestnet struct {
	bin              string
	keystorePath     string
	keyPassword      string
	scorePaths       map[string]string
	defaultStepLimit string
	url              string
	Client           rpcclient.Client
}

type Result struct {
	NodeInfo struct {
		ProtocolVersion struct {
			P2P   string `json:"p2p"`
			Block string `json:"block"`
			App   string `json:"app"`
		} `json:"protocol_version"`
		ID         string `json:"id"`
		ListenAddr string `json:"listen_addr"`
		Network    string `json:"network"`
		Version    string `json:"version"`
		Channels   string `json:"channels"`
		Moniker    string `json:"moniker"`
		Other      struct {
			TxIndex    string `json:"tx_index"`
			RPCAddress string `json:"rpc_address"`
		} `json:"other"`
	} `json:"NodeInfo"`
	SyncInfo struct {
		LatestBlockHash     string    `json:"latest_block_hash"`
		LatestAppHash       string    `json:"latest_app_hash"`
		LatestBlockHeight   string    `json:"latest_block_height"`
		LatestBlockTime     time.Time `json:"latest_block_time"`
		EarliestBlockHash   string    `json:"earliest_block_hash"`
		EarliestAppHash     string    `json:"earliest_app_hash"`
		EarliestBlockHeight string    `json:"earliest_block_height"`
		EarliestBlockTime   time.Time `json:"earliest_block_time"`
		CatchingUp          bool      `json:"catching_up"`
	} `json:"SyncInfo"`
	ValidatorInfo struct {
		Address string `json:"Address"`
		PubKey  struct {
			Type  string `json:"type"`
			Value string `json:"value"`
		} `json:"PubKey"`
		VotingPower string `json:"VotingPower"`
	} `json:"ValidatorInfo"`
}
