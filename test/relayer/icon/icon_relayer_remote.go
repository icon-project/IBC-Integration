// Package rly provides an interface to the cosmos relayer running in a Docker container.
package rly

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	// "github.com/cosmos/cosmos-sdk/crypto/keyring"
	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/relayer"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/relayer/rly"
	"go.uber.org/zap"
)

// ICONRemoteRelayer is the ibc.Relayer implementation for github.com/cosmos/relayer.
type ICONRemoteRelayer struct {
	// Embedded DockerRelayer so commands just work.
	*relayer.DockerRelayer
}

func NewICONRemoteRelayer(log *zap.Logger, testName string, cli *client.Client, networkID string, relayerAccounts map[string]string, useExistingConfig bool, options ...relayer.RelayerOption) *ICONRemoteRelayer {
	c := RemoteCommander{log: log}
	for _, opt := range options {
		switch o := opt.(type) {
		case relayer.RelayerOptionExtraStartFlags:
			c.extraStartFlags = o.Flags
		}
	}
	dr, err := relayer.NewDockerRelayer(context.TODO(), log, testName, cli, networkID, c, relayerAccounts, useExistingConfig, options...)
	if err != nil {
		panic(err) // TODO: return
	}
	return &ICONRemoteRelayer{DockerRelayer: dr}
}

type ICONRemoteRelayerChainConfigValue struct {
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

type ICONRemoteRelayerChainConfig struct {
	Type  string                            `json:"type"`
	Value ICONRemoteRelayerChainConfigValue `json:"value"`
}

func ChainConfigToICONRemoteRelayerRemoteChainConfig(chainConfig ibc.ChainConfig, keyName, rpcAddr, gprcAddr, keystorePwd string) ICONRemoteRelayerChainConfig {
	chainType := chainConfig.Type
	keystore := "godwallet"
	pwd := "gochain"
	if keystorePwd != "" {
		parts := strings.Split(keystorePwd, ":")
		keystore = parts[0]
		pwd = parts[1]
	}
	return ICONRemoteRelayerChainConfig{
		Type: chainType,
		Value: ICONRemoteRelayerChainConfigValue{
			Key:               "icx",
			ChainID:           chainConfig.ChainID,
			RPCAddr:           rpcAddr,
			Timeout:           "10s",
			Keystore:          keystore,
			KeyDirectory:      "/home/relayer/.relayer/keys",
			Password:          pwd,
			IconNetworkID:     3,
			BtpNetworkID:      chainConfig.ConfigFileOverrides["btp-network-id"].(int),
			StartHeight:       chainConfig.ConfigFileOverrides["start-height"].(int),
			BTPNetworkTypeID:  chainConfig.ConfigFileOverrides["btp-network-type-id"].(int),
			IBCHandlerAddress: chainConfig.ConfigFileOverrides["ibc-handler-address"].(string),
			BlockInterval:     chainConfig.ConfigFileOverrides["block-interval"].(int),
		},
	}
}

type RemoteCommander struct {
	log             *zap.Logger
	extraStartFlags []string
}

func (RemoteCommander) Name() string {
	return "rly"
}

func (RemoteCommander) DockerUser() string {
	return RlyDefaultUidGid // docker run -it --rm --entrypoint echo ghcr.io/cosmos/relayer "$(id -u):$(id -g)"
}

func (RemoteCommander) AddChainConfiguration(containerFilePath, homeDir string) []string {
	return []string{
		"rly", "chains", "add", "-f", containerFilePath,
	}
}

func (RemoteCommander) AddKey(chainID, keyName, coinType, homeDir string) []string {
	return []string{
		"rly", "keys", "add", chainID, keyName,
		"--coin-type", fmt.Sprint(coinType),
	}
}

func (RemoteCommander) CreateChannel(pathName string, opts ibc.CreateChannelOptions, homeDir string) []string {
	cleanUpStoredHeight()
	return []string{
		"rly", "tx", "channel", pathName,
		"--src-port", opts.SourcePortName,
		"--dst-port", opts.DestPortName,
	}
}

func (RemoteCommander) CreateClients(pathName string, opts ibc.CreateClientOptions, homeDir string) []string {
	cleanUpStoredHeight()
	if strings.Contains(pathName, "icon") {
		return []string{
			"rly", "tx", "clients", pathName, "--client-tp", opts.TrustingPeriod,
			"--src-wasm-code-id", "1cff60adf40895b5fccb1e9ce6305a65ae01400a02cc4ded2cf3669221905adc", "--override", "-d",
		}
	}
	return []string{
		"rly", "tx", "clients", pathName, "--client-tp", opts.TrustingPeriod,
		"--override", "-d",
	}

}

// passing a value of 0 for customeClientTrustingPeriod will use default
func (RemoteCommander) CreateClient(pathName, homeDir, customeClientTrustingPeriod string) []string {
	cleanUpStoredHeight()
	return []string{
		"rly", "tx", "client", pathName, "--client-tp", customeClientTrustingPeriod,
	}
}

func (RemoteCommander) CreateConnections(pathName string, homeDir string) []string {
	cleanUpStoredHeight()
	return []string{
		"rly", "tx", "connection", pathName,
	}
}

func (RemoteCommander) Flush(pathName, channelID, homeDir string) []string {
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

func (RemoteCommander) GeneratePath(srcChainID, dstChainID, pathName, homeDir string) []string {
	return []string{
		"rly", "paths", "new", dstChainID, srcChainID, pathName,
	}
}

func (RemoteCommander) UpdatePath(pathName, homeDir string, filter ibc.ChannelFilter) []string {
	return []string{
		"rly", "paths", "update", pathName,

		"--filter-rule", filter.Rule,
		"--filter-channels", strings.Join(filter.ChannelList, ","),
	}
}

func (RemoteCommander) GetChannels(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "channels", chainID,
	}
}

func (RemoteCommander) GetConnections(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "connections", chainID,
	}
}

func (RemoteCommander) GetClients(chainID, homeDir string) []string {
	return []string{
		"rly", "q", "clients", chainID,
	}
}

// TODO figure out values for commented parameters
func (RemoteCommander) LinkPath(pathName, homeDir string, channelOpts ibc.CreateChannelOptions, clientOpt ibc.CreateClientOptions) []string {
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

func (RemoteCommander) RestoreKey(chainID, keyName, coinType, mnemonic, homeDir string) []string {
	return []string{
		"rly", "keys", "restore", chainID, keyName, mnemonic,
		"--coin-type", fmt.Sprint(coinType),
	}
}

func (c RemoteCommander) StartRelayer(homeDir string, pathNames ...string) []string {
	cmd := []string{
		"rly", "start",
	}
	return cmd
}

func (RemoteCommander) UpdateClients(pathName, homeDir string) []string {
	return []string{
		"rly", "tx", "update-clients", pathName,
	}
}

func (RemoteCommander) ConfigContent(ctx context.Context, cfg ibc.ChainConfig, keyName, rpcAddr, grpcAddr string, relayerAccounts map[string]string) ([]byte, error) {

	switch chainType := cfg.Type; chainType {
	case "wasm":
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
	case "cosmos":
		cosmosRelayerChainConfig := rly.ChainConfigToCosmosRelayerChainConfig(cfg, keyName, rpcAddr, grpcAddr)
		coinType, err := strconv.Atoi(cfg.CoinType)

		archRelayerChainConfig := &CosmosRelayerChainConfig{
			Type: "cosmos",
			Value: CosmosRelayerChainConfigValue{
				CosmosRelayerChainConfigValue: cosmosRelayerChainConfig.Value,
				KeyDir:                        "/home/relayer/.relayer/keys/" + cfg.ChainID,
				MinGasAmount:                  1000000,
				CoinType:                      coinType,
			},
		}
		if relayerAccounts != nil {
			key := relayerAccounts[cfg.Name]
			archRelayerChainConfig = &CosmosRelayerChainConfig{
				Type: "cosmos",
				Value: CosmosRelayerChainConfigValue{
					CosmosRelayerChainConfigValue: cosmosRelayerChainConfig.Value,
					KeyDir:                        "/home/relayer/.relayer/keys/" + cfg.ChainID,
					MinGasAmount:                  1000000,
					CoinType:                      coinType,
					Key:                           key,
				},
			}
		}
		jsonBytes, err := json.Marshal(archRelayerChainConfig)
		if err != nil {
			return nil, err
		}
		return jsonBytes, nil

	default:
		if relayerAccounts != nil {
			ICONRelayerChainConfig := ChainConfigToICONRemoteRelayerRemoteChainConfig(cfg, keyName, rpcAddr, grpcAddr, relayerAccounts["icon"])
			jsonBytes, err := json.Marshal(ICONRelayerChainConfig)
			if err != nil {
				return nil, err
			}
			return jsonBytes, nil
		}
		ICONRelayerChainConfig := ChainConfigToICONRelayerChainConfig(cfg, keyName, rpcAddr, grpcAddr)
		jsonBytes, err := json.Marshal(ICONRelayerChainConfig)
		if err != nil {
			return nil, err
		}
		return jsonBytes, nil

	}
}

func (RemoteCommander) DefaultContainerImage() string {
	return DefaultContainerImage
}

func (RemoteCommander) DefaultContainerVersion() string {
	return DefaultContainerVersion
}

func (RemoteCommander) ParseAddKeyOutput(stdout, stderr string) (ibc.Wallet, error) {
	var wallet WalletModel
	err := json.Unmarshal([]byte(stdout), &wallet)
	rlyWallet := NewWallet("", wallet.Address, wallet.Mnemonic)
	return rlyWallet, err
}

func (RemoteCommander) ParseRestoreKeyOutput(stdout, stderr string) string {
	return strings.Replace(stdout, "\n", "", 1)
}

func (c RemoteCommander) ParseGetChannelsOutput(stdout, stderr string) ([]ibc.ChannelOutput, error) {
	var channels []ibc.ChannelOutput
	channelSplit := strings.Split(stdout, "\n")
	for _, channel := range channelSplit {
		if strings.TrimSpace(channel) == "" {
			continue
		}
		if strings.HasPrefix(channel, "{") {
			var channelOutput ibc.ChannelOutput
			err := json.Unmarshal([]byte(channel), &channelOutput)
			if err != nil {
				c.log.Error("Failed to parse channels json", zap.Error(err))
				continue
			}
			channels = append(channels, channelOutput)
		}
	}

	return channels, nil
}

func (c RemoteCommander) ParseGetConnectionsOutput(stdout, stderr string) (ibc.ConnectionOutputs, error) {
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

func (c RemoteCommander) ParseGetClientsOutput(stdout, stderr string) (ibc.ClientOutputs, error) {
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

func (RemoteCommander) Init(homeDir string) []string {
	path, _ := os.Getwd()
	baseTestDir := filepath.Dir(path)
	if _, err := os.Stat(baseTestDir + "/relayer/data/config/config.yaml"); err == nil {
		err := os.RemoveAll(baseTestDir + "/relayer/data/config")
		if err != nil {
			log.Fatal("failed to remove file:", err)
		}
	}
	if _, err := os.Stat(baseTestDir + "/relayer/data/ibc-icon"); err == nil {
		err = os.RemoveAll(baseTestDir + "/relayer/data/ibc-icon")
		if err != nil {
			log.Fatal("failed to remove file:", err)
		}
	}
	return []string{
		"rly", "config", "init",
	}
}

func (c RemoteCommander) CreateWallet(keyName, address, mnemonic string) ibc.Wallet {
	return NewWallet(keyName, address, mnemonic)
}

func cleanUpStoredHeight() {
	path, _ := os.Getwd()
	baseTestDir := filepath.Dir(path)
	if _, err := os.Stat(baseTestDir + "/relayer/data/ibc-icon"); err == nil {
		err = os.RemoveAll(baseTestDir + "/relayer/data/ibc-icon")
		if err != nil {
			log.Fatal("failed to remove file:", err)
		}
	}
}
