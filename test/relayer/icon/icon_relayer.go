// Package rly provides an interface to the cosmos relayer running in a Docker container.
package rly

import (
	"context"
	"encoding/json"
	"fmt"
	"strconv"
	"strings"

	"github.com/strangelove-ventures/interchaintest/v7/relayer/rly"

	// "github.com/cosmos/cosmos-sdk/crypto/keyring"
	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/relayer"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"go.uber.org/zap"
)

const (
	RlyDefaultUidGid = "100:1000"
)

// ICONRelayer is the ibc.Relayer implementation for github.com/cosmos/relayer.
type ICONRelayer struct {
	// Embedded DockerRelayer so commands just work.
	*relayer.DockerRelayer
}

func NewICONRelayer(log *zap.Logger, testName string, cli *client.Client, networkID string, options ...relayer.RelayerOption) *ICONRelayer {
	c := commander{log: log}
	for _, opt := range options {
		switch o := opt.(type) {
		case relayer.RelayerOptionExtraStartFlags:
			c.extraStartFlags = o.Flags
		}
	}
	dr, err := relayer.NewDockerRelayer(context.TODO(), log, testName, cli, networkID, c, options...)
	if err != nil {
		panic(err) // TODO: return
	}
	return &ICONRelayer{DockerRelayer: dr}
}

type ICONRelayerChainConfigValue struct {
	KeyDirectory      string `json:"key-directory"`
	Key               string `json:"key"`
	ChainID           string `json:"chain-id"`
	RPCAddr           string `json:"rpc-addr"`
	Timeout           string `json:"timeout"`
	Keystore          string `json:"keystore"`
	Password          string `json:"password"`
	IconNetworkID     int    `json:"icon-network-id"`
	BtpNetworkID      int    `json:"btp-network-id"`
	StartHeight       int    `json:"start-height"`
	BTPNetworkTypeID  int    `json:"btp-network-type-id"`
	IBCHandlerAddress string `json:"ibc-handler-address"`
	BlockInterval     int    `json:"block-interval"`
}

type ArchRelayerChainConfigValue struct {
	rly.CosmosRelayerChainConfigValue
	KeyDir            string `json:"key-directory"`
	MinGasAmount      int    `json:"min-gas-amount"`
	CoinType          int    `json:"coin-type"`
	StartHeight       int    `json:"start-height"`
	IBCHandlerAddress string `json:"ibc-handler-address"`
	BlockInterval     int    `json:"block-interval"`
}

type ICONRelayerChainConfig struct {
	Type  string                      `json:"type"`
	Value ICONRelayerChainConfigValue `json:"value"`
}

type ArchRelayerChainConfig struct {
	Type  string                      `json:"type"`
	Value ArchRelayerChainConfigValue `json:"value"`
}

const (
	DefaultContainerImage   = "ghcr.io/cosmos/relayer"
	DefaultContainerVersion = "v2.3.1"
)

// Capabilities returns the set of capabilities of the Cosmos relayer.
//
// Note, this API may change if the rly package eventually needs
// to distinguish between multiple rly versions.
func Capabilities() map[relayer.Capability]bool {
	// RC1 matches the full set of capabilities as of writing.
	return relayer.FullCapabilities()
}

func ChainConfigToICONRelayerChainConfig(chainConfig ibc.ChainConfig, keyName, rpcAddr, gprcAddr string) ICONRelayerChainConfig {
	chainType := chainConfig.Type
	fmt.Println(chainConfig)
	return ICONRelayerChainConfig{
		Type: chainType,
		Value: ICONRelayerChainConfigValue{
			Key:               "icx",
			ChainID:           chainConfig.ChainID,
			RPCAddr:           "http://" + rpcAddr + "/api/v3/",
			Timeout:           "10s",
			Keystore:          "relayer-" + chainConfig.Name,
			KeyDirectory:      "/home/relayer/.relayer/keys",
			Password:          "relayer-" + chainConfig.Name,
			IconNetworkID:     3,
			BtpNetworkID:      chainConfig.ConfigFileOverrides["btp-network-id"].(int),
			StartHeight:       chainConfig.ConfigFileOverrides["start-height"].(int),
			BTPNetworkTypeID:  chainConfig.ConfigFileOverrides["btp-network-type-id"].(int),
			IBCHandlerAddress: chainConfig.ConfigFileOverrides["ibc-handler-address"].(string),
			BlockInterval:     chainConfig.ConfigFileOverrides["block-interval"].(int),
		},
	}
}

// commander satisfies relayer.RelayerCommander.
type commander struct {
	log             *zap.Logger
	extraStartFlags []string
}

func (commander) Name() string {
	return "rly"
}

func (commander) DockerUser() string {
	return RlyDefaultUidGid // docker run -it --rm --entrypoint echo ghcr.io/cosmos/relayer "$(id -u):$(id -g)"
}

func (commander) AddChainConfiguration(containerFilePath, homeDir string) []string {
	return []string{
		"rly", "chains", "add", "-f", containerFilePath,
	}
}

func (commander) AddKey(chainID, keyName, coinType, homeDir string) []string {
	return []string{
		"rly", "keys", "add", chainID, keyName,
		"--coin-type", fmt.Sprint(coinType),
	}
}

func (commander) CreateChannel(pathName string, opts ibc.CreateChannelOptions, homeDir string) []string {
	return []string{
		"rly", "tx", "channel", pathName,
		"--src-port", opts.SourcePortName,
		"--dst-port", opts.DestPortName,
		"--order", opts.Order.String(),
		"--version", opts.Version,
	}
}

func (commander) CreateClients(pathName string, opts ibc.CreateClientOptions, homeDir string) []string {
	return []string{
		"rly", "tx", "clients", pathName, "--client-tp", opts.TrustingPeriod,
	}
}

// passing a value of 0 for customeClientTrustingPeriod will use default
func (commander) CreateClient(pathName, homeDir, customeClientTrustingPeriod string) []string {
	return []string{
		"rly", "tx", "client", pathName, "--client-tp", customeClientTrustingPeriod,
	}
}

func (commander) CreateConnections(pathName string, homeDir string) []string {
	return []string{
		"rly", "tx", "connection", pathName,
	}
}

func (commander) Flush(pathName, channelID, homeDir string) []string {
	cmd := []string{"rly", "tx", "flush"}
	if pathName != "" {
		cmd = append(cmd, pathName)
		if channelID != "" {
			cmd = append(cmd, channelID)
		}
	}
	cmd = append(cmd, "--home", homeDir)
	return cmd
}

func (commander) GeneratePath(srcChainID, dstChainID, pathName, homeDir string) []string {
	return []string{
		"rly", "paths", "new", srcChainID, dstChainID, pathName,
	}
}

func (commander) UpdatePath(pathName, homeDir string, filter ibc.ChannelFilter) []string {
	return []string{
		"rly", "paths", "update", pathName,

		"--filter-rule", filter.Rule,
		"--filter-channels", strings.Join(filter.ChannelList, ","),
	}
}

func (commander) GetChannels(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "channels", chainID,
	}
}

func (commander) GetConnections(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "connections", chainID,
	}
}

func (commander) GetClients(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "clients", chainID,
	}
}

// TODO figure out values for commented parameters
func (commander) LinkPath(pathName, homeDir string, channelOpts ibc.CreateChannelOptions, clientOpt ibc.CreateClientOptions) []string {
	return []string{
		"rly", "tx", "link", pathName,
		"--src-port", channelOpts.SourcePortName,
		"--dst-port", channelOpts.DestPortName,
		"--order", channelOpts.Order.String(),
		// "--version", channelOpts.Version,
		"--client-tp", clientOpt.TrustingPeriod, //should not be 0 set large integer in string e.g 1000m
		"--debug",
	}
}

func (commander) RestoreKey(chainID, keyName, coinType, mnemonic, homeDir string) []string {
	return []string{
		"rly", "keys", "restore", chainID, keyName, mnemonic,
		"--coin-type", fmt.Sprint(coinType),
	}
}

func (c commander) StartRelayer(homeDir string, pathNames ...string) []string {
	cmd := []string{
		"rly", "start", "--debug",
	}
	cmd = append(cmd, c.extraStartFlags...)
	cmd = append(cmd, pathNames...)
	return cmd
}

func (commander) UpdateClients(pathName, homeDir string) []string {
	return []string{
		"rly", "tx", "update-clients", pathName,
	}
}

func (commander) ConfigContent(ctx context.Context, cfg ibc.ChainConfig, keyName, rpcAddr, grpcAddr string) ([]byte, error) {

	switch chainType := cfg.Type; chainType {
	case "cosmos", "wasm":
		cosmosRelayerChainConfig := rly.ChainConfigToCosmosRelayerChainConfig(cfg, keyName, rpcAddr, grpcAddr)
		coinType, err := strconv.Atoi(cfg.CoinType)
		archRelayerChainConfig := &ArchRelayerChainConfig{
			Type: cosmosRelayerChainConfig.Type,
			Value: ArchRelayerChainConfigValue{
				CosmosRelayerChainConfigValue: cosmosRelayerChainConfig.Value,
				KeyDir:                        "/home/relayer/.relayer/keys/" + cfg.ChainID,
				MinGasAmount:                  1000000,
				CoinType:                      coinType,
				StartHeight:                   cfg.ConfigFileOverrides["start-height"].(int),
				IBCHandlerAddress:             cfg.ConfigFileOverrides["ibc-handler-address"].(string),
				BlockInterval:                 cfg.ConfigFileOverrides["block-interval"].(int),
			},
		}

		jsonBytes, err := json.Marshal(archRelayerChainConfig)
		if err != nil {
			return nil, err
		}
		return jsonBytes, nil

	default:
		ICONRelayerChainConfig := ChainConfigToICONRelayerChainConfig(cfg, keyName, rpcAddr, grpcAddr)
		jsonBytes, err := json.Marshal(ICONRelayerChainConfig)
		if err != nil {
			return nil, err
		}
		return jsonBytes, nil
	}
}

func (commander) DefaultContainerImage() string {
	return DefaultContainerImage
}

func (commander) DefaultContainerVersion() string {
	return DefaultContainerVersion
}

func (commander) ParseAddKeyOutput(stdout, stderr string) (ibc.Wallet, error) {
	var wallet WalletModel
	err := json.Unmarshal([]byte(stdout), &wallet)
	rlyWallet := NewWallet("", wallet.Address, wallet.Mnemonic)
	return rlyWallet, err
}

func (commander) ParseRestoreKeyOutput(stdout, stderr string) string {
	return strings.Replace(stdout, "\n", "", 1)
}

func (c commander) ParseGetChannelsOutput(stdout, stderr string) ([]ibc.ChannelOutput, error) {
	var channels []ibc.ChannelOutput
	channelSplit := strings.Split(stdout, "\n")
	for _, channel := range channelSplit {
		if strings.TrimSpace(channel) == "" {
			continue
		}
		var channelOutput ibc.ChannelOutput
		err := json.Unmarshal([]byte(channel), &channelOutput)
		if err != nil {
			c.log.Error("Failed to parse channels json", zap.Error(err))
			continue
		}
		channels = append(channels, channelOutput)
	}

	return channels, nil
}

func (c commander) ParseGetConnectionsOutput(stdout, stderr string) (ibc.ConnectionOutputs, error) {
	var connections ibc.ConnectionOutputs
	for _, connection := range strings.Split(stdout, "\n") {
		if strings.TrimSpace(connection) == "" {
			continue
		}

		var connectionOutput ibc.ConnectionOutput
		if err := json.Unmarshal([]byte(connection), &connectionOutput); err != nil {
			c.log.Error(
				"Error parsing connection json",
				zap.Error(err),
			)

			continue
		}
		connections = append(connections, &connectionOutput)
	}

	return connections, nil
}

func (c commander) ParseGetClientsOutput(stdout, stderr string) (ibc.ClientOutputs, error) {
	var clients ibc.ClientOutputs
	for _, client := range strings.Split(stdout, "\n") {
		if strings.TrimSpace(client) == "" {
			continue
		}

		var clientOutput ibc.ClientOutput
		if err := json.Unmarshal([]byte(client), &clientOutput); err != nil {
			c.log.Error(
				"Error parsing client json",
				zap.Error(err),
			)

			continue
		}
		clients = append(clients, &clientOutput)
	}

	return clients, nil
}

func (commander) Init(homeDir string) []string {
	return []string{
		"rly", "config", "init",
	}
}

func (c commander) CreateWallet(keyName, address, mnemonic string) ibc.Wallet {
	return NewWallet(keyName, address, mnemonic)
}
