package cosmos

import (
	"testing"
	"time"

	"github.com/icon-project/icon-bridge/common/codec"
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

type XcallInit struct {
	TimeoutHeight int    `json:"timeout_height"`
	IbcHost       string `json:"ibc_host"`
}

type DappInit struct {
	Address string `json:"address"`
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
	ChainID          string
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

type TxResul struct {
	Height    string        `json:"height"`
	Txhash    string        `json:"txhash"`
	Codespace string        `json:"codespace"`
	Code      int           `json:"code"`
	Data      string        `json:"data"`
	RawLog    string        `json:"raw_log"`
	Logs      []interface{} `json:"logs"`
	Info      string        `json:"info"`
	GasWanted string        `json:"gas_wanted"`
	GasUsed   string        `json:"gas_used"`
	Tx        interface{}   `json:"tx"`
	Timestamp string        `json:"timestamp"`
	Events    []struct {
		Type       string `json:"type"`
		Attributes []struct {
			Key   string `json:"key"`
			Value string `json:"value"`
			Index bool   `json:"index"`
		} `json:"attributes"`
	} `json:"events"`
}

type CallServiceMessageType int64

const (
	CallServiceRequest  CallServiceMessageType = 1
	CallServiceResponse CallServiceMessageType = 2
)

type CSMessageResponseType int64

const (
	SUCCESS   CSMessageResponseType = 0
	FAILURE   CSMessageResponseType = -1
	IBC_ERROR CSMessageResponseType = -2
)

type CallServiceMessage struct {
	MessageType CallServiceMessageType
	Payload     []byte
}

func RlpEncode(request []byte, callType CallServiceMessageType) ([]byte, error) {

	data := CallServiceMessage{
		MessageType: callType,
		Payload:     request,
	}
	return codec.RLP.MarshalToBytes(data)
}

func RlpDecode(raw_data []byte) (CallServiceMessage, error) {

	var callservicemessage = CallServiceMessage{}

	codec.RLP.UnmarshalFromBytes(raw_data, &callservicemessage)

	return CallServiceMessage{}, nil
}

type CallServiceMessageRequest struct {
	From       string `json:"from"`
	To         string `json:"to"`
	SequenceNo uint64 `json:"sequence_no"`
	Rollback   bool   `json:"rollback"`
	Data       []byte `json:"data"`
}

func (cr *CallServiceMessageRequest) RlpEncode() ([]byte, error) {

	return codec.RLP.MarshalToBytes(cr)
}

type CallServiceMessageResponse struct {
	SequenceNo uint64
	Code       CSMessageResponseType
	Msg        string
}

func (csr *CallServiceMessageResponse) RlpEncode() ([]byte, error) {
	return codec.RLP.MarshalToBytes(csr)
}
