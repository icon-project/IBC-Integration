package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/docker/docker/client"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/ibc-integration/test/testsuite/testconfig"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	ctypes "github.com/cometbft/cometbft/rpc/core/types"

	"go.uber.org/zap"
)

func NewCosmosRemotenet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, cli *client.Client, network string, keyPassword string, testconfig *testconfig.Chain) (chains.Chain, error) {

	chain := NewCosmosRemoteChain(testName, chainConfig, cli, network, log, testconfig)
	return &CosmosRemotenet{
		CosmosRemoteChain: chain,
		cfg:               chain.Config(),
		keyName:           keyPassword,
		filepath:          testconfig.Contracts,
		Wallets:           map[string]ibc.Wallet{},
		testCfg:           testconfig,
	}, nil
}

func (c *CosmosRemotenet) PreGenesis() error {
	return nil
}

func (c *CosmosRemotenet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) SetupXCall(ctx context.Context, portId string, keyName string) error {
	panic("not implemented")
}

func (c *CosmosRemotenet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) GetIBCAddress(key string) string {
	panic("not implemented")
}

func (c *CosmosRemotenet) DeployXCallMockApp(ctx context.Context, connection chains.XCallConnection) error {
	panic("not implemented")
}

func (c *CosmosRemotenet) InitEventListener(ctx context.Context, contract string) chains.EventListener {
	panic("not implemented")
}

func (c *CosmosRemotenet) CheckForTimeout(ctx context.Context, target chains.Chain, params map[string]interface{}, listener chains.EventListener) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) SendPacketXCall(ctx context.Context, keyName, _to string, data, rollback []byte) (context.Context, error) {
	panic("not implemented")
}

// FindTargetXCallMessage returns the request id and the data of the message sent to the target chain
func (c *CosmosRemotenet) FindTargetXCallMessage(ctx context.Context, target chains.Chain, height uint64, to string) (*chains.XCallResponse, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) SendPacketMockDApp(ctx context.Context, targetChain chains.Chain, keyName string, params map[string]interface{}) (chains.PacketTransferResponse, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) findPacket(tx *TxResul, eventType string) chantypes.Packet {
	panic("not implemented")
}

func (c *CosmosRemotenet) XCall(ctx context.Context, targetChain chains.Chain, keyName, to string, data, rollback []byte) (*chains.XCallResponse, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) findSn(tx *TxResul, eType string) string {
	panic("not implemented")
}

// IsPacketReceived returns the receipt of the packet sent to the target chain
func (c *CosmosRemotenet) IsPacketReceived(ctx context.Context, params map[string]interface{}, order ibc.Order) bool {
	panic("not implemented")
}

func (c *CosmosRemotenet) ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) FindCallMessage(ctx context.Context, startHeight uint64, from, to, sn string) (string, string, error) {
	panic("not implemented")

}

func (c *CosmosRemotenet) FindCallResponse(ctx context.Context, startHeight uint64, sn string) (string, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) FindEvent(ctx context.Context, startHeight uint64, contract, index string) (*ctypes.ResultEvent, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) QueryContract(ctx context.Context, contractAddress, methodName string, params map[string]interface{}) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName string, params map[string]interface{}) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) GetLastBlock(ctx context.Context) (context.Context, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosRemotenet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) BuildWallets(ctx context.Context, keyName string) (ibc.Wallet, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) BuildWallet(ctx context.Context, keyName string, mnemonic string) (ibc.Wallet, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) GetCommonArgs() []string {
	return []string{"--gas", "auto"}
}

func (c *CosmosRemotenet) GetClientName(suffix int) string {
	return fmt.Sprintf("remote-cosmos-%d", suffix)
}

func (c *CosmosRemotenet) GetClientState(ctx context.Context, clientSuffix int) (any, error) {

	panic("not implemented")
}

// GetClientsCount returns the next sequence number for the client
func (c *CosmosRemotenet) GetClientsCount(ctx context.Context) (int, error) {
	panic("not implemented")
}

// GetConnectionState returns the next sequence number for the client
func (c *CosmosRemotenet) GetConnectionState(ctx context.Context, connectionPrefix int) (*conntypes.ConnectionEnd, error) {
	panic("not implemented")
}

// GetNextConnectionSequence returns the next sequence number for the client
func (c *CosmosRemotenet) GetNextConnectionSequence(ctx context.Context) (int, error) {
	panic("not implemented")
}

// GetChannel returns the next sequence number for the client
func (c *CosmosRemotenet) GetChannel(ctx context.Context, connectionPrefix int, portID string) (*chantypes.Channel, error) {
	panic("not implemented")
}

// GetNextChannelSequence returns the next sequence number for the client
func (c *CosmosRemotenet) GetNextChannelSequence(ctx context.Context) (int, error) {
	panic("not implemented")
}

// PauseNode halts a node
func (c *CosmosRemotenet) PauseNode(ctx context.Context) error {
	panic("not implemented")
}

// UnpauseNode restarts a node
func (c *CosmosRemotenet) UnpauseNode(ctx context.Context) error {
	panic("not implemented")
}

func (c *CosmosRemotenet) BackupConfig() ([]byte, error) {
	panic("not implemented")
}

func (c *CosmosRemotenet) RestoreConfig(backup []byte) error {
	panic("not implemented")
}

func (c *CosmosRemotenet) SetupIBCICS20(ctx context.Context, keyName string) (context.Context, error) {
	time.Sleep(1 * time.Second)
	fmt.Println("Nothing to do in cosmos for ibc ics20 remote setup")
	// ibcWasmCodeId, err := c.CosmosRemoteChain.StoreContract(ctx, keyName, c.filepath["ibc_ics20_client"])
	// if err != nil {
	// 	return nil, err
	// }
	// fmt.Println(ibcWasmCodeId)
	return ctx, nil
}

func (c *CosmosRemotenet) SendIBCTokenTransfer(ctx context.Context, sourceChannel, destinationChannel, port, receiver, chainID, ibcamount string) (string, error) {
	commands := []string{"tx", "ibc-transfer", port, port, sourceChannel, receiver, ibcamount}
	commands = append(commands,
		"--node", c.GetHostRPCAddress(),
		"--packet-timeout-height", "1-500000",
		"--packet-timeout-timestamp", "0",
		"--gas-prices", "0.1stake",
		"--gas-adjustment", "1.5",
		"--gas", "auto",
		"--from", c.testConfig.KeystoreFile,
		"--keyring-backend", c.testConfig.KeystorePassword,
		"--output", "json",
		"--chain-id", chainID,
		"-y",
	)
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return "", err
	}
	var output TxResul
	if err != nil {
		return "", err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		return "", err
	}
	return output.Txhash, nil
}

// GetBalance fetches the current balance for a specific account address and denom.
func (c *CosmosRemotenet) GetBalance(ctx context.Context, address string, denom string) (int64, error) {
	panic("not implemented")
}
