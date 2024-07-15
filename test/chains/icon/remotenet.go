package icon

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/big"
	"path/filepath"
	"strings"
	"time"

	"github.com/docker/docker/client"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/ibc-integration/test/internal/dockerutil"
	"github.com/icon-project/ibc-integration/test/testsuite/testconfig"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"

	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"

	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	// "github.com/strangelove-ventures/interchaintest/v7/testutil"
	"go.uber.org/zap"
)

type IconRemotenet struct {
	log          *zap.Logger
	testName     string
	cfg          ibc.ChainConfig
	Client       *client.Client
	Network      string
	scorePaths   map[string]string
	IBCAddresses map[string]string     `json:"addresses"`
	Wallets      map[string]ibc.Wallet `json:"wallets"`
	testconfig   testconfig.Chain
}

const (
	WalletKeyStore       = "/goloop/data/"
	MinterWalletKeyStore = "/goloop/data/minter.json"
)

func (c *IconRemotenet) CreateKey(ctx context.Context, keyName string) error {
	panic("implement me")

}

func NewIconRemotenet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, client *client.Client, network string, testconfig testconfig.Chain) chains.Chain {
	return &IconRemotenet{
		testName:   testName,
		cfg:        chainConfig,
		log:        log,
		scorePaths: testconfig.Contracts,
		Wallets:    map[string]ibc.Wallet{},
		Client:     client,
		Network:    network,
		testconfig: testconfig,
	}
}

// Config fetches the chain configuration.
func (c *IconRemotenet) Config() ibc.ChainConfig {
	return c.cfg
}

func (c *IconRemotenet) OverrideConfig(key string, value any) {
	if value == nil {
		return
	}
	c.cfg.ConfigFileOverrides[key] = value
}

// Initialize initializes node structs so that things like initializing keys can be done before starting the chain
func (c *IconRemotenet) Initialize(ctx context.Context, testName string, cli *client.Client, networkID string) error {
	panic("not implemented")
}

func (c *IconRemotenet) NewChainNode(
	ctx context.Context,
	testName string,
	cli *client.Client,
	networkID string,
	image ibc.DockerImage,
	validator bool,
) (*IconNode, error) {
	panic("not implemented")
}

// Start sets up everything needed (validators, gentx, fullnodes, peering, additional accounts) for chain to start from genesis.
func (c *IconRemotenet) Start(testName string, ctx context.Context, additionalGenesisWallets ...ibc.WalletAmount) error {
	panic("not implemented")
}

// Exec runs an arbitrary command using Chain's docker environment.
// Whether the invoked command is run in a one-off container or execing into an already running container
// is up to the chain implementation.
//
// "env" are environment variables in the format "MY_ENV_VAR=value"
func (c *IconRemotenet) Exec(ctx context.Context, cmd []string, env []string) (stdout []byte, stderr []byte, err error) {
	cmd = append([]string{c.cfg.Bin}, cmd...)
	job := dockerutil.NewImage(c.log, c.Client, c.Network, c.testName, c.cfg.Images[0].Repository, c.cfg.Images[0].Version)
	var ContainerEnvs = [9]string{
		"GOCHAIN_CONFIG=/goloop/data/config.json",
		"GOCHAIN_GENESIS=/goloop/data/genesis.json",
		"GOCHAIN_DATA=/goloop/chain/iconee",
		"GOCHAIN_LOGFILE=/goloop/chain/iconee.log",
		"GOCHAIN_DB_TYPE=rocksdb",
		"GOCHAIN_CLEAN_DATA=true",
		"JAVAEE_BIN=/goloop/execman/bin/execman",
		"PYEE_VERIFY_PACKAGE=true",
		"ICON_CONFIG=/goloop/data/icon_config.json",
	}
	opts := dockerutil.ContainerOptions{
		Binds: []string{
			c.testconfig.ContractsPath + ":/contracts",
			c.testconfig.ConfigPath + ":/goloop/data",
		},
		Env: ContainerEnvs[:],
	}
	res := job.Run(ctx, cmd, opts)
	return res.Stdout, res.Stderr, res.Err
}

// ExportState exports the chain state at specific height.
func (c *IconRemotenet) ExportState(ctx context.Context, height int64) (string, error) {
	panic("not implemented")
}

// GetRPCAddress retrieves the rpc address that can be reached by other containers in the docker network.
func (c *IconRemotenet) GetRPCAddress() string {
	return c.testconfig.RPCUri
}

// GetGRPCAddress retrieves the grpc address that can be reached by other containers in the docker network.
// Not Applicable for Icon
func (c *IconRemotenet) GetGRPCAddress() string {
	return c.testconfig.RPCUri
}

// GetHostRPCAddress returns the rpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
func (c *IconRemotenet) GetHostRPCAddress() string {
	return c.testconfig.RPCUri
}

// GetHostGRPCAddress returns the grpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
// Not applicable for Icon
func (c *IconRemotenet) GetHostGRPCAddress() string {
	return c.testconfig.RPCUri
}

// HomeDir is the home directory of a node running in a docker container. Therefore, this maps to
// the container's filesystem (not the host).
func (c *IconRemotenet) HomeDir() string {
	return c.getFullNode().HomeDir()
}

// RecoverKey recovers an existing user from a given mnemonic.
func (c *IconRemotenet) RecoverKey(ctx context.Context, name string, mnemonic string) error {
	panic("not implemented") // TODO: Implement
}

// GetAddress fetches the bech32 address for a test key on the "user" node (either the first fullnode or the first validator if no fullnodes).
func (c *IconRemotenet) GetAddress(ctx context.Context, keyName string) ([]byte, error) {
	panic("not implemented")
}

// SendFunds sends funds to a wallet from a user account.
func (c *IconRemotenet) SendFunds(ctx context.Context, keyName string, amount ibc.WalletAmount) error {
	panic("not implemented")
}

func (c *IconRemotenet) SendFundsFromGodwallet(ctx context.Context, amount ibc.WalletAmount) error {
	commands := []string{"rpc", "sendtx", "transfer"}
	commands = append(commands,
		"--uri", c.GetHostRPCAddress(),
		"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
		"--key_password", c.testconfig.KeystorePassword,
		"--to", amount.Address,
		"--value", fmt.Sprint(amount.Amount)+"000000000000000000",
		"--step_limit", "25000000000",
		"--nid", "0x3",
	)
	_, _, err := c.Exec(ctx, commands, nil)
	return err

}

// SendIBCTransfer sends an IBC transfer returning a transaction or an error if the transfer failed.
func (c *IconRemotenet) SendIBCTransfer(ctx context.Context, channelID string, keyName string, amount ibc.WalletAmount, options ibc.TransferOptions) (ibc.Tx, error) {
	panic("not implemented")
}
func (c *IconRemotenet) SendIBCTokenTransfer(ctx context.Context, sourceChannel, destinationChannel, port, sender, receiver, chainID, ibcamount string, hopRequired bool) (string, error) {

	ics20App := c.IBCAddresses["ics20App"]
	commands := []string{"rpc", "sendtx", "call"}
	amount, denom, _ := strings.Cut(ibcamount, "/")
	if denom == "icx" {
		commands = append(commands,
			"--to", ics20App,
			"--method", "sendICX",
			"--uri", c.GetHostRPCAddress(),
			"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
			"--key_password", c.testconfig.KeystorePassword,
			"--value", amount,
			"--step_limit", "25000000000",
			"--nid", "0x3",
		)

		params := `{"params":{"receiver":"pfm","sourcePort":"` + port + `","sourceChannel":"` +
			sourceChannel +
			`","timeoutHeight":{ "revisionHeight": "80","revisionNumber": "80"},"timeoutTimestamp":"0",` +
			`"memo":"{\\\"forward\\\":{\\\"receiver\\\":\\\"` +
			receiver + `\\\",\\\"port\\\":\\\"` + port + `\\\",\\\"channel\\\":\\\"` +
			destinationChannel + `\\\",\\\"timeout\\\":\\\"10m\\\",\\\"retries\\\":2}}"}}`
		if !hopRequired {
			// remove memo for no hop
			params = `{"params":{"receiver":"` + receiver + `","sourcePort":"` + port + `","sourceChannel":"` +
				sourceChannel +
				`","timeoutHeight":{ "revisionHeight": "80","revisionNumber": "80"},"timeoutTimestamp":"0"` +
				`}}`
		}
		commands = append(commands, "--raw", params)
		var output string
		stdout, _, err := c.Exec(ctx, commands, nil)
		if err != nil {
			return "", err
		}
		err = json.Unmarshal(stdout, &output)
		if err != nil {
			return "", err
		}
		return c.waitForTxn(ctx, output)
	}
	// Find the index where "transfer" starts
	transferIndex := strings.Index(ibcamount, "transfer")

	// Extract the amount
	sendAmount := ibcamount[:transferIndex]

	// Extract the denom
	sendDenom := ibcamount[transferIndex:]
	tokenContractAddress, err := c.GetTokenContractAddress(ctx, sendDenom)
	if err != nil {
		fmt.Println("Failed to get token contract address")
		return "", err
	}
	commands = append(commands,
		"--to", tokenContractAddress,
		"--method", "transfer",
		"--uri", c.GetHostRPCAddress(),
		"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
		"--key_password", c.testconfig.KeystorePassword,
		"--step_limit", "25000000000",
		"--nid", "0x3",
	)

	ftparams := `{"method":"sendFungibleTokens","params":{"denomination":"` + sendDenom + `","amount": ` + sendAmount + `,"sender": "` + sender + `","receiver": "` + receiver + `","sourcePort": "transfer","sourceChannel":"channel-0","timeoutHeight":{"latestHeight":80,"revisionNumber":80},"timeoutTimestamp":0,"memo":"{\\\"forward\\\":{\\\"receiver\\\":\\\"` +
		receiver + `\\\",\\\"port\\\":\\\"` + port + `\\\",\\\"channel\\\":\\\"` +
		destinationChannel + `\\\",\\\"timeout\\\":\\\"10m\\\",\\\"retries\\\":2}}"}}`

	if !hopRequired {
		// remove memo for no hop
		ftparams = `{"method":"sendFungibleTokens","params":{"denomination":"` + sendDenom + `","amount": ` + sendAmount + `,"sender": "` + sender + `","receiver": "` + receiver + `","sourcePort": "transfer","sourceChannel":"channel-0","timeoutHeight":{"latestHeight":80,"revisionNumber":80},"timeoutTimestamp":0}}`
	}
	tokenParams := hex.EncodeToString([]byte(ftparams))
	params := `{"params":{"_to":"` + ics20App + `","_value":"` + sendAmount + `","_data":"` + tokenParams + `"}}`
	commands = append(commands, "--raw", params)
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return "", err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		return "", err
	}
	return c.waitForTxn(ctx, output)
}

// Height returns the current block height or an error if unable to get current height.
func (c *IconRemotenet) Height(ctx context.Context) (uint64, error) {
	panic("not implemented")
}

// GetGasFeesInNativeDenom gets the fees in native denom for an amount of spent gas.
func (c *IconRemotenet) GetGasFeesInNativeDenom(gasPaid int64) int64 {
	panic("not implemented")
}

// Acknowledgements returns all acknowledgements in a block at height.
func (c *IconRemotenet) Acknowledgements(ctx context.Context, height uint64) ([]ibc.PacketAcknowledgement, error) {
	panic("not implemented") // TODO: Implement
}

// Timeouts returns all timeouts in a block at height.
func (c *IconRemotenet) Timeouts(ctx context.Context, height uint64) ([]ibc.PacketTimeout, error) {
	panic("not implemented") // TODO: Implement
}

// BuildRelayerWallet will return a chain-specific wallet populated with the mnemonic so that the wallet can
// be restored in the relayer node using the mnemonic. After it is built, that address is included in
// genesis with some funds.
func (c *IconRemotenet) BuildRelayerWallet(ctx context.Context, keyName string) (ibc.Wallet, error) {
	panic("not implemented")
}

func (c *IconRemotenet) BuildWallet(ctx context.Context, keyName string, mnemonic string) (ibc.Wallet, error) {
	panic("not implemented")
}

func (c *IconRemotenet) getFullNode() *IconNode {
	panic("not implemented")
}

func (c *IconRemotenet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("not implemented")
}

func (c *IconRemotenet) GetTokenContractAddress(ctx context.Context, denom string) (string, error) {
	ics20App := c.IBCAddresses["ics20App"]
	commands := []string{"rpc", "call"}
	commands = append(commands,
		"--to", ics20App,
		"--method", "getTokenContractAddress",
		"--uri", c.GetHostRPCAddress(),
	)
	// params := `{"denom":"` + denom + `","account":"` + address + `"}`
	params := `{"denom":"` + denom + `"}`
	commands = append(commands, "--params", params)
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return "", err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		return "", err
	}
	return output, nil
}

// GetBalance fetches the current balance for a specific account address and denom.
func (c *IconRemotenet) GetWalletBalance(ctx context.Context, address string, denom string) (*big.Int, error) {
	if denom == "icx" {
		bal, err := c.GetBalance(ctx, address, denom)
		return big.NewInt(bal), err
	}

	// get token contract address
	ics20App := c.IBCAddresses["ics20App"]
	commands := []string{"rpc", "call"}
	commands = append(commands,
		"--to", ics20App,
		"--method", "getTokenContractAddress",
		"--uri", c.GetHostRPCAddress(),
	)
	params := `{"denom":"` + denom + `"}`
	commands = append(commands, "--params", params)
	var tokenContractAddress string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		fmt.Println(err)
		return big.NewInt(0), err
	}
	err = json.Unmarshal(stdout, &tokenContractAddress)
	if err != nil {
		fmt.Println(err)
		return big.NewInt(0), err
	}

	//get balance
	commands = []string{"rpc", "call"}
	commands = append(commands,
		"--to", tokenContractAddress,
		"--method", "balanceOf",
		"--uri", c.GetHostRPCAddress(),
	)
	// params := `{"denom":"` + denom + `","account":"` + address + `"}`
	params = `{"_owner":"` + address + `"}`
	commands = append(commands, "--params", params)
	var output string
	stdout, _, err = c.Exec(ctx, commands, nil)
	if err != nil {
		fmt.Println(err)
		return big.NewInt(0), err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		fmt.Println(err)
		return big.NewInt(0), err
	}
	balanceBigInt := new(big.Int)
	output = strings.TrimPrefix(output, "0x")
	balanceBigInt.SetString(output, 16)
	return balanceBigInt, nil
}

func (c *IconRemotenet) GetBalance(ctx context.Context, address string, denom string) (int64, error) {
	commands := []string{"rpc", "balance", address}
	commands = append(commands,
		"--uri", c.GetHostRPCAddress(),
	)
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return 0, nil
	}
	balanceBigInt := new(big.Int)
	if err := json.Unmarshal(stdout, &output); err != nil {
		panic(err)
	}
	output = strings.TrimPrefix(output, "0x")
	balanceBigInt.SetString(output, 16)
	return balanceBigInt.Int64(), nil
}

func (c *IconRemotenet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) SetupXCall(ctx context.Context, portId string, keyName string) error {
	panic("unimplemented")
}

func (c *IconRemotenet) PreGenesis() error {
	panic("unimplemented")
}

func (c *IconRemotenet) DeployXCallMockApp(ctx context.Context, connection chains.XCallConnection) error {
	panic("unimplemented")
}

func (c *IconRemotenet) GetIBCAddress(key string) string {
	value, exist := c.IBCAddresses[key]
	if !exist {
		panic(fmt.Sprintf(`IBC address not exist %s`, key))
	}
	return value
}

func (c *IconRemotenet) BackupConfig() ([]byte, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) RestoreConfig(backup []byte) error {
	panic("unimplemented")
}

func (c *IconRemotenet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) SendPacketXCall(ctx context.Context, keyName, _to string, data, rollback []byte) (context.Context, error) {
	panic("unimplemented")
}

// HasPacketReceipt returns the receipt of the packet sent to the target chain
func (c *IconRemotenet) IsPacketReceived(ctx context.Context, params map[string]interface{}, order ibc.Order) bool {
	panic("not implemented")
}

// FindTargetXCallMessage returns the request id and the data of the message sent to the target chain
func (c *IconRemotenet) FindTargetXCallMessage(ctx context.Context, target chains.Chain, height uint64, to string) (*chains.XCallResponse, error) {
	panic("not implemented")
}

func (c *IconRemotenet) XCall(ctx context.Context, targetChain chains.Chain, keyName, to string, data, rollback []byte) (*chains.XCallResponse, error) {
	panic("not implemented")
}

func (c *IconRemotenet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error) {
	panic("not implemented")
}

func (c *IconRemotenet) ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error) {
	panic("not implemented")
}

func (c *IconRemotenet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) FindCallMessage(ctx context.Context, startHeight uint64, from, to, sn string) (string, string, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) FindCallResponse(ctx context.Context, startHeight uint64, sn string) (string, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) FindEvent(ctx context.Context, startHeight uint64, contract, signature string, index []*string) (*icontypes.EventNotification, error) {
	panic("unimplemented")
}

// DeployContract implements chains.Chain
func (c *IconRemotenet) DeployContract(ctx context.Context, scorePath string) (context.Context, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName string, params map[string]interface{}) (context.Context, error) {
	panic("unimplemented")
}

// GetBlockByHeight implements chains.Chain
func (c *IconRemotenet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetLastBlock implements chains.Chain
func (c *IconRemotenet) GetLastBlock(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) InitEventListener(ctx context.Context, contract string) chains.EventListener {
	panic("unimplemented")
}

func (c *IconRemotenet) CheckForTimeout(ctx context.Context, target chains.Chain, params map[string]interface{}, listener chains.EventListener) (context.Context, error) {

	panic("unimplemented")
}

// QueryContract implements chains.Chain
func (c *IconRemotenet) QueryContract(ctx context.Context, contractAddress, methodName string, params map[string]interface{}) (context.Context, error) {
	panic("unimplemented")

}

func (c *IconRemotenet) BuildWallets(ctx context.Context, keyName string) (ibc.Wallet, error) {
	return nil, nil
}

func (c *IconRemotenet) GetClientName(suffix int) string {
	return fmt.Sprintf("08-tendermint-%d", suffix)
}

func (c *IconRemotenet) GetClientState(ctx context.Context, clientSuffix int) (any, error) {
	panic("unimplemented")
}

// GetClientsCount returns the next sequence number for the client
func (c *IconRemotenet) GetClientsCount(ctx context.Context) (int, error) {
	panic("unimplemented")
}

// GetConnectionState returns the next sequence number for the client
func (c *IconRemotenet) GetConnectionState(ctx context.Context, clientSuffix int) (*conntypes.ConnectionEnd, error) {
	panic("unimplemented")
}

// GetNextConnectionSequence returns the next sequence number for the client
func (c *IconRemotenet) GetNextConnectionSequence(ctx context.Context) (int, error) {
	panic("unimplemented")
}

func (c *IconRemotenet) GetChannel(ctx context.Context, channelSuffix int, portID string) (*chantypes.Channel, error) {
	panic("unimplemented")
}

// GetNextChannelSequence returns the next sequence number for the client
func (c *IconRemotenet) GetNextChannelSequence(ctx context.Context) (int, error) {
	panic("unimplemented")
}

// PauseNode pauses the node
func (c *IconRemotenet) PauseNode(ctx context.Context) error {
	panic("unimplemented")
}

// UnpauseNode starts the paused node
func (c *IconRemotenet) UnpauseNode(ctx context.Context) error {
	panic("unimplemented")
}

func (c *IconRemotenet) SendPacketMockDApp(ctx context.Context, targetChain chains.Chain, keyName string, params map[string]interface{}) (chains.PacketTransferResponse, error) {
	panic("unimplemented")

}

func (c *IconRemotenet) waitForTxn(ctx context.Context, txnhash string) (string, error) {
	loop := 0
	time.Sleep(time.Second * 4)
	for loop < 10 {
		commands := []string{"rpc", "txresult", txnhash}
		commands = append(commands,
			"--uri", c.GetHostRPCAddress(),
		)
		stdout, _, err := c.Exec(ctx, commands, nil)
		if err != nil {
			fmt.Println(err)
		}
		var res *icontypes.TransactionResult
		if err := json.Unmarshal(stdout, &res); err != nil {
			panic(err)
		}
		if res.Status == "0x1" {
			return string(res.SCOREAddress), nil
		}
		if res.Failure.CodeValue != "Ox0" {
			return "", fmt.Errorf("error encountered in transaction execution")
		}
		time.Sleep(time.Second * 2)
		loop++
	}
	return "", fmt.Errorf("timeout waiting for transaction")
}

func (c *IconRemotenet) waitForTxnRsp(ctx context.Context, txnhash string) (*icontypes.TransactionResult, error) {
	loop := 0
	time.Sleep(time.Second * 4)
	for loop < 10 {
		commands := []string{"rpc", "txresult", txnhash}
		commands = append(commands,
			"--uri", c.GetHostRPCAddress(),
		)
		stdout, _, err := c.Exec(ctx, commands, nil)
		if err != nil {
			fmt.Println(err)
		}
		var res *icontypes.TransactionResult
		if err := json.Unmarshal(stdout, &res); err != nil {
			panic(err)
		}
		if res.Status == "0x1" {
			return res, nil
		}
		if res.Failure.CodeValue != "Ox0" {
			return nil, fmt.Errorf("error encountered in transaction execution")
		}
		time.Sleep(time.Second * 2)
		loop++
	}
	return nil, fmt.Errorf("timeout waiting for transaction")
}

func (c *IconRemotenet) DeployContractRemote(ctx context.Context, contractPath, initMessage string) (string, error) {
	_, score := filepath.Split(contractPath)
	commands := []string{"rpc", "sendtx", "deploy", "/contracts/" + score}
	commands = append(commands,
		"--to", "cx0000000000000000000000000000000000000000",
		"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
		"--key_password", c.testconfig.KeystorePassword,
		"--step_limit", "25000000000",
		"--content_type", "application/java",
		"--nid", "0x3",
		"--uri", c.GetHostRPCAddress(),
	)
	if initMessage != "" && initMessage != "{}" {
		if strings.HasPrefix(initMessage, "{") {
			commands = append(commands, "--params", initMessage)
		} else {
			commands = append(commands, "--param", initMessage)
		}
	}
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return "", err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		return "", err
	}
	return c.waitForTxn(ctx, output)
}

func (c *IconRemotenet) ExecuteContractRemote(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {

	commands := []string{"rpc", "sendtx", "call"}
	keyst := WalletKeyStore + c.testconfig.KeystoreFile
	if methodName == "mint" {
		keyst = MinterWalletKeyStore
	}
	commands = append(commands,
		"--to", contractAddress,
		"--method", methodName,
		"--key_store", keyst,
		"--key_password", c.testconfig.KeystorePassword,
		"--step_limit", "25000000000",
		"--nid", "0x3",
		"--uri", c.GetHostGRPCAddress(),
	)
	if params != "" && params != "{}" {
		if strings.HasPrefix(params, "{") {
			commands = append(commands, "--params", params)
		} else {
			commands = append(commands, "--param", params)
		}
	}
	if methodName == "registerPRep" {
		commands = append(commands, "--value", "2000000000000000000000")
	}
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		fmt.Println("error occurred whiuc executing command ", err)
		return ctx, err
	}
	err = json.Unmarshal(stdout, &output)
	if err != nil {
		return ctx, err
	}
	res, err := c.waitForTxnRsp(ctx, output)
	if err != nil {
		return ctx, err
	}
	return context.WithValue(ctx, "txResult", res), nil
}

func (c *IconRemotenet) SetupIBCICS20(ctx context.Context, keyName string) (context.Context, error) {
	var contracts chains.ContractKey

	if c.scorePaths["ics20_app"] == "" {
		time.Sleep(1 * time.Second)
		ibcAddress, err := c.DeployContractRemote(ctx, c.scorePaths["ibc_ics"], "")
		if err != nil {
			return nil, err
		}
		fmt.Println("IBC Handler deployed at ", ibcAddress)

		ics20client, err := c.DeployContractRemote(ctx, c.scorePaths["ibc_ics20_client"], "ibcHandler="+ibcAddress)
		if err != nil {
			return nil, err
		}
		fmt.Println("IBC ics20 light Client deployed at ", ics20client)

		c.ExecuteContractRemote(context.Background(), ibcAddress, "registerClient", `{"hashType":"1","clientType":"`+"ics08-tendermint"+`", "client":"`+ics20client+`"}`)

		ics20App, err := c.DeployContractRemote(ctx, c.scorePaths["ics20app"], `{"_ibcHandler":"`+ibcAddress+`","_serializeIrc2":"504b03041400080808000000210000000000000000000000000014000d004d4554412d494e462f4d414e49464553542e4d46555405000100000000feca0000f34dcccb4c4b2d2ed10d4b2d2acecccfb35230d433e0e5f24dccccd375ce492c2eb65270e2e5e2e50200504b0708fbb81f722a00000028000000504b0304140008080800000021000000000000000000000000000d0009004d4554412d494e462f41504953555405000100000000b594cf4ee33010875d077802dea30f00422a072424240e5ded613944d3641a821c3bb29d22dea205ab77a07f5240629703e73e080fc3246d2fd084b2da3d46fee59bf17c93bcb1dd211b4948d063fd017f39b9801e3405c8a8d9b63a96d1de904dcc55d251a23a908718c4090853441aaf8b4802f6bc791847c7d262849a628f565910ed2c4dc5557d72d601aa10e069d76b0cdcc4579712f5f3890994c6662b0c351ab3b73daf21e45683345dd49cdfb87bdfaacf6f13b70722c30a4663eec67e0816eece0ebd797fc8f31f4b24f31c1d75b54ad640ff492d36d98f656c0f98774307859d3573e77337f5176ad69fcefc9597dbe346411d75322d394d74ea43a232692bdba1f00c84509785058f5fbb718502373529caf09b7646492c2d276cc5b436e96f0a69aa550f0b4c75136ef20569b075fb6bc89e56eb724456f97ff6bb28f92796814630d85acdf98b9b3c401862f8b3b6cc124ddebf857e3659872610d8cdf83975725aec43b14b639f9ed64049d1a3c6840c6d129d4568cb5cf90fd9f97df62144be635306bc1a50d91ecf5be5668060f4edffc5e2ba71bdcbfe3b504b07088f8e194ea001000035050000504b03041400080808000000210000000000000000000000000007000900422e636c617373555405000100000000a556f9571b5514fe5e32744232b434b6546a9774cf46e9225d002b10a88d96a204a904b7217984d064864e268075a9fbbed5ddd65d6b5dea39d66329e8d1e3cffe4d1e8ff74d262190d0726a388737efcd7df7fbee77efbb6ffefef7b73f011cc44f0cac4b06a3a153869386888c1a066fb43fb26fc050935c1dc9f0dde3eaa44aefc618ea8fe712bac19bbb0c554b8c7577b5310c2e5a6ab7e79dc9a4c173b9367bda9d4e98555e0ad7cd59d51c6bee4aa7a29ac953dc683b427f844798ab16d9d36a9ac1793c424f2bdad35ada3cc2e0f717dc64542dd51c338db4966aab5c89060615ac42bd1b125633b8d54c469f22ca3ce7c22d0c6baa3191b196a1ae4021a2d3dab429631d8347e353c578195aae8f1fc9a8c43cb05839058d58efc6adb84d810bb5b570602343706c7acf327f2e6c2e91b3f591b185221b35f46c8106c3b62adc028b4455b00ddbddd88a1d0a3c5004935d24748f8c00a9ac4f69dc208d420c6bab39b3546d722388dd0aeab0526cdfc3208de40d8da1d15f3dc581414aa35f6c76e176376d6961a84d71334269e106c36a7f25cb833824f43a4cdc9256d697534b02a20ded02e20e8228659de140858385d34075870c2b1766524627834335197696ebd33732ce1366d14bf9928208ba3de8420f83527e3a64dc45c2c57bfafb181aaaa32b88e2ee5a34e01eda4b7af519dd7c54cd6708fd5015f465f2e9c5090f8e8160a56c5a235f2ca1a0bfa05a8c415627260c7d922f5ff438b9a0ea3b780389974ed9493c20c087284a9374ce8d72e3289535c3e19b7239dc2528d526f4ec846af001fd3a851955f0101e7693c48f50bca211ee69f51539f8d4ac9ed74c1f9f4e709eccf94ae5e402752bd9e0a7f36983746af0c7ab1f950492a286c9c495cb8f90db04a9bd7b292e4bd5400a638220b54236f23f24213ea790113a93b0abd35ac2e06a8e77ce1f11a79a4c2a9828a09d269b24afb0d9614b547c959c17c537c2e9c977861bba0b14e776dbd42e279f68543e738c5b163eb540d7854986ad8b2c4dbd8add3411cc71f2bb6b79954fe19ec1e3a2d09f20f93b2dd76a46c153855a3b4b094ce7fa44bba31b70b1a681b88267f0ac072bf01c1d137fbc90cd1744365f2477c46ac9add4eb36cfd38952874ba99998a99abc87ea68c24ceb9a8c5729ea0aa34e2395cf72cd2cb37b5dc1cb7845f0789361fdfc8e7eaacb74b6dce3db7417508be8250e6a8a32b5c61fa82c4a05efe05d37cee13deaf5069fe48629ca375abd7c3fc08722e08fe80a3478963263c5ace07c81d1854213b716734289c0704517ff149f09d3cfe96362718a647cc9b0b19818bfbd275c1ca933055cf89ac4cee8a99e492e1a55c03f5c99e92a4b82fc457c2bc85f2292b1744a53cdbc38aa52444fd25047f9489cea552706c4270f2917d3f346821f4d6738b6507548207de084575cd6f4fc3d7d4039e0a6395d99f4ec15771f8dabe81d7d65d0ff1f68d642fb241a03c15078e3357883de35de86596cb88a4d41af6f163bafc21ffc1dc1216ff81a9aaf62ef15881fc33eecb75d3412a883c6bae00c0e0467d11a9ac191a2d99de8b0cd76929993c6f5c15fb1293487a30efc8563e15f707c0ef73ae9b9e1677bcf7d68b7f7acb3f7782cd7a1f00c068a8eefc760c948b28d0436d9c4a5cbb6d1301eb48d386a481d20ba009d3694e1b7d634d634cde0d10b90a54b909cde91598c06436464ad8f9367e2d0d428cd40b320187ea4ff7434ff4150464307753be8255e2d563a84b48297c0a5b1881c2ac325ca4639eb5cc9458e5c88ec9cbcceee134de172ce79c1f9c680e345c0b218fc85181ce26eb309c4edf2e810eefc178b205304125eb0f298805d90d8a6393ce984106c064f5f2921b920291dacdec2a12f2b1b6703cd188df5e463afc5fa790766f15231dbd4506ccb0eb214aaae139684f6960397a0f43a29a6f767f1f115ab14dd54ebafd9e31b36b00c47af8c732e9a9e5fd2db2737e18dfa85edad01cc8ac22dbccde10b86624953632e55abc372eab11129ce42de25d1e66da3fd3413d5baabdef907be1a727abf890d49a1d8504d38d62ad5336bad2946f52a35d6cce2bba24a97ad232fff07504b0708c979aeffe40500003e0d0000504b03041400080808000000210000000000000000000000000007000900412e636c61737355540500010000000095578b7fdb5615feae2dc78aa3248e69434d9bcedda3f32b49d775a3cb4249f3e896ad2350979624409065c555234b992427eb06940263830d187bc15ade8395376bbbbac9da3d78ad309efb7ff801e74ab2eb3aca167eb12d9da373cff9ce39dfb95779fb3fafbe0ee083788781ed8f8231c48fcb4bf2a02e1be5c1a9e2715571a208d3c3f128220c5d9387c6768fcab6a60c7033d2d3b7fba0ad98963ab8bf54b254dbbe9bb445864e5f7b44b6c64749b7a3591e3e782d4ac1b134a37cf73ebe4ea16f89e19600db8aec1c1b1cd5ca9386a39655cbb357e93b4fb07cfb714d71dc60775ca7186e01f82e0ecbf49d61681bd60ccdd9c7904eaf45ba56339939c2104e678e48e8c59618042419628eb9a01a73865c51456cbdaeb0deb228faa84c1eb43193503c44b5be814134d465377386ddef1e7e4c97299dcc759596b00337c690c24d1262e8684708b730481e18fb44a568ea226e95d0e93dcb50bc92aa681559b745e418360515278a7e095dde8a41d79b23eb73767571513f21e2360971efd9ede4ad28133645256f7730b4532e5e13186eff7f92f15b291137f7f26cee92d083048f416dea684a388a0f51edcbaa43d0d39983adec250f1fc64807f6613fc3ce000499b52a0922c662146a9c3cdbdcf3e6f45ac7bcdb07700f777d2f435fbd882943554b76ca31534535b568da9aa32d1101ee63885aea8355cd22cef6a667028090bf83788027fb11865ca03f7220eb29d34ae9e6b26aa59c63b291da7d9b888f92f72559afaa53340dc9f47d99608a4b3884420c0338cc2050d44a6bcd1a053882a3316cc727c8af664f54169d132ec16724cc60963ff9242db51d2ba511c0aaee6252b999884f4bd884cdbc539f61d872cdf77edb562d47338d09cb328951c518143e27029f101a398f9bb424bd0e7aeabbcbbb824b3b5a383371688a8ab95eae1a8eb753ae0bc457a2c79435aececb55dd69a141bd9b41cca9c0e0ed3589c73eab797d6f4db7ec27eb563b0e8bd3c8a6a9702cd9b0e7558b21dbba3c78f5ec28df54da09f998aceb7c614f3ad3b252c2321ee284a16adcb5c6ed46a3f07de2118ef3b39cf8edbc739f6f6c4cfef228be402d72e967af330ec48d2fe24b319cc297196e9e9bb7cc4a4af656a714d9304c8733f861d532eb6a115f61b871ce31dfd3ee71aa84625616654b3d6c72860767939994f0353cc119fe24c3d6397724d619c86f1081270dbb3a3faf299a6a5050afc3229ea276d9d522754c21b20cac176bbda63f8d67388067ddc363233de01de8c1f3bc03dfa629a3a425bce03939cd109aa52deebb749c68363f24382809dfe743780a3fa036b91bfb01a24851561644fc88e6422189e1dee0e8cd733ebbc13978113fe12cfb2915e6b0cf6309673dd2fc8c9a3db56cd06ef45e4dfc05b544ae9855aa75704b7e45d82b9ae188f80d4522e68f9e7054a29b90cecc8e4a7819e7f8ce739e14c5aa6588788526bbf9b488a2c6b037809b1b1ef7950e5cc2aa3be21b71e29e0097f99a2b0c37d54b93f60b9eaf5f29a57c91a79211f13a65a69be58925a21c43261dd082a0ae50a437f13bde84df93836145f7df4fba5bcecf28fe443b6249b5e99c293576dc82233b555bc2554e9bb7f0678648da9bfc5ebccd89f4579aaf825636c88c9f4fc29859a24b27ad53161e90170fcb459de458c1ac5a8a7a40d3556107b55e00ed0988621bdf36e83532c15f3848f777ba0f4122b9b345ee6a92bb498eb7c874c437e4f7c5b7f08384eebb49d78bf7d3ef3f48ba8c3045069eca5ec207b2896d89ed2bb8f92276661369ff369b4de41303eeedae6c62b77fbb279bb893df0e5dc470f615ec5c05bd935cc5fdfc3e9bab61621593219094cde6eb52df194485b310c289fb5730d517dffc625dfe18c964bbabefd1157cdcb53d479818fe49bf4944fe8b05085184a2d84e1f068cb0ae1196a0b4e8d5c24fe41e9242741dccbd851e02305dc3a74ed7fdcf91fff3904fa33b77151d97a14c5f42e90d6ef572234e1bc28911d64b928aa3bed3245d291c3a1b29be89ed7c095f341f68966d35cba11c60b6cb371ba89b1d6b986da33cb8599cccf69c87be8ac550b3e583b07ccb849f731bafb8537f5ec592ff7c2b22f4472d5fc1c343423629e4f2fd357cee9c6b28f083ca377c87b26fa3eb4be771925c3dda28dc636ee14ee69b755f255d3fe1aae1eb8d867e93379083e86fd67ecbd5d6f5cfd5f09d6c9ed3814b67484a0a57d11ebe229e4524290c45e8c90f5fc0d67ce2c76daf41980ee70ad3427f613a928c1456f0d2d12ca14f466af87933373a3811dccfa61196a37fbabef76fa4e897d2ebc4f37e7a8f5015c274955bd3fb2501ccb7a4f26b8f8b7b5aaa4fe0ce789cf612ca7b2964b9cb7ce2b7355cb80e9908c1434640ba1a409ef5812c6f18881f2ab0a679bfa681609ff3c1f2ee252eb6c2eb405b57bd6a44a17883527d848f53aa873c0ee75c97af86afb12fc4cf56df740b99f26424d734bf8ad7c238e773f06c835a7ba90051bae6e382dbd4c41bd455ea6c245f986eeb2f0c45e2ccd52785c2505b32926c5bc11f3c2f22fe48313c2f77ba91801b12a11afe522fdb05c897716a3abef98a78097fbb8093d7128c20248df071fb978b3afa3f504b070808fc33285b0700009c0f0000504b03041400080808000000210000000000000000000000000007000900432e636c61737355540500010000000075514d4fc240107d034881fa057e1d389878928b3d7ae0440c26241a12493ce06969274dc9b235dba5e25ff3e00ff0471987a64183ba979979efe5cd9becc7e7db3b806b9c11e8c603110ee72a578156260ec6b33987ce4395d01ebf18b6f7caa898edd55a4268a8282a6042e7f22e0b53cbc1208a2c6759bff74838ff761a69cdb1d213a71c0f57213fbb24351e1a848b5fa2818d970b36ee87ae45e8960b9431a9b8081c0c578ead51ba4ff02d2fd29ccb34cd985dd166eb64bda7ad6c92dcb28a52a35fab72bcdcec25d9bf874c09ad4d1431ec3e2c8d4b163c3279922533cd834d22615b93746943be4d34d70915d4b07eb51aa10e4fe66399ea529bf07152f4845d1110f684a72d6c1f0725e61798fcce146d213a38fa8390722a50053b5f504b07080769ea7c21010000d8010000504b03041400080808000000210000000000000000000000000007000900442e636c6173735554050001000000003bf56fd73e06060673067646064617760646460681acc4b244fd9cc4bc747dffa4acd4e41276066646064ecf2067233d9014230357707e695172aa5b664e2a1b230313030b03043032b0014926065600504b070830f176835000000056000000504b03041400080808000000210000000000000000000000000007000900452e636c6173735554050001000000008555db72db54145d475250a2c86ded2681128738a1802f49c3a550682e34755330380de0129a16684eec83aa54918da4a429cffd09dace30d3175ef24067208961a03cf3097c001fd1c914f69115db3566b0c73ad2d6de6bafb5ce96fcc7939f7f03700636035bd0c1188eadf32d3ee570d79a5a5a5b17e540874a37f33a7a1806976eb9c25be42eb78457d8a839a76436dd5e67385af4cb554f4ccd7b1ebf7de1fc3443eae9c8cce165a5e209df9f9ea39467666cd70ee608395d6c352e059eed5ad3996506359d5936d18f98010d4718624f61e838d60ce5ab6e20b6896e82c170c5ada82bc3e92ed06d91bcc3894ca683bd89010c1a388e2113bde8eb8382e74827f9944877e8c8145ab41ad53a920c9a6f7f23420105132f60b41f2348d1b5250229b79029763a4d4dc7f1a2cc3b49ce88af37b9e37738132566ae9a7819af188823dd8d111937de2a2a388eb0b833ef599b1bc20d16b6cba216d8555747cec4f3183648dc24c300773cc12bb75362dbf6033f55957bdd8b2969bf696002af117b5ea9fc0725daa637705ab27f938cba4993e456830eacb709a156ad51af7457f967312d016628cd97360da50bdd3bcde15d99788e3add60e8234ff39c547a0cf174a6c30d13795c905bb9c0a0db7e38c2dd4c234fdfc3fbd20cdad0d17fd9570a7820dabcfb90e178396c9ab2fd94d41a895c3450945e3187a19798850d4d7cdc20f10943d2b2b7849be28dc6b23a6472587f99cae85e54765e8ebe82cf4c7c84ac3c5b9103199e5d63e8f7c446754b44b9671af12f1b8e84415f2acd5cebd04a9bd319d2b1466525db7279b0e9d1e06af96a859618e92edf5ce4b5cb7ccda16ba36901411ba5eaa65716176d47608c3a6ba0c1a55f423e33b456e8e5a2c0a02f93434447419161a814058e66f7e8974bc4f7f1ec4f38f110f2a3c9898c120b94a6d23aa9ce26b33fe2441d630c0f302acf9375bca4e077c473bbc87c8b9ee4ce1d857dfff79fca0e15307c454703ea011436fe183d044ba6119c085f776a083b9aa5da53b3c97b88fd828995c4ab7b78fd91c4ced5f19682874d2092748001062be4c7e851d0a4d711dc1502d3683d1bc1dd8fe0de3984ab639649a6975a22bec39150c4641df36a5b27138aec7480933a3ddbedfd680e2257b2d48fd13a98ddc7c55d7c7057f62bae242eed61e9510b8a9206a88ea6a6ad4eda3eb88f52c3b406cf4f25cf8e3a86e5a6be615a1bdbb58b2b52e35579f8bcb15d56688f42133f41455ffc4fd1f56e45abe011c3ebc4573a79aee993b6fa2be2c59c4a944bcde0ac3a37927c80e1dc88b62a832387a350baa3d208fc95fba1292606e509c67494197b8c2405e9098f9a4d440c87223beea23fdcc07be8d176d4d6146950e81f85e14638cbfa3f504b0708f1364a35e903000033070000504b0102140014000808080000002100fbb81f722a0000002800000014000d0000000000000000000000000000004d4554412d494e462f4d414e49464553542e4d46555405000100000000feca0000504b01021400140008080800000021008f8e194ea0010000350500000d00090000000000000000000000790000004d4554412d494e462f41504953555405000100000000504b0102140014000808080000002100c979aeffe40500003e0d000007000900000000000000000000005d020000422e636c617373555405000100000000504b010214001400080808000000210008fc33285b0700009c0f000007000900000000000000000000007f080000412e636c617373555405000100000000504b01021400140008080800000021000769ea7c21010000d8010000070009000000000000000000000018100000432e636c617373555405000100000000504b010214001400080808000000210030f176835000000056000000070009000000000000000000000077110000442e636c617373555405000100000000504b0102140014000808080000002100f1364a35e903000033070000070009000000000000000000000005120000452e636c617373555405000100000000504b05060000000007000700c90100002c1600000000"}`)
		if err != nil {
			return nil, err
		}
		fmt.Println("ICS20 app deployed at ", ics20App)
		contracts.ContractAddress = map[string]string{
			"ibc":      ibcAddress,
			"ics20App": ics20App,
		}

		// assume Preps are already configured
		params := `{"networkTypeName":"eth", "name":"eth", "owner":"` + ibcAddress + `"}`
		ctx, _ = c.ExecuteContractRemote(ctx, "cx0000000000000000000000000000000000000001", "openBTPNetwork", params)
		//height, _ := ctx.Value("txResult").(icontypes.TransactionResult).BlockHeight.Int()
		id := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[0].Indexed[2]
		typeId := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[0].Indexed[1]
		btpNetworkId, _ := icontypes.HexInt(id).Int()
		btpNetworkTypeId, _ := icontypes.HexInt(typeId).Int()
		// //bind ics app
		portId := "transfer"
		params = `{"portId":"` + portId + `","moduleAddress":"` + ics20App + `"}`
		ctx, err = c.ExecuteContractRemote(ctx, ibcAddress, "bindPort", params)
		if err != nil {
			return ctx, err
		}

		overrides := map[string]any{
			"ibc-handler-address": ibcAddress,
			"start-height":        0, // height + 1,
			"btp-network-id":      btpNetworkId,
			"btp-network-type-id": btpNetworkTypeId,
			"block-interval":      2_000,
		}
		cfg := c.cfg
		cfg.ConfigFileOverrides = overrides
		c.cfg = cfg
	} else {
		contracts.ContractAddress = map[string]string{
			"ics20App": c.scorePaths["ics20_app"],
		}
	}
	c.IBCAddresses = contracts.ContractAddress
	return context.WithValue(ctx, chains.Mykey("contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), nil
}

func (c *IconRemotenet) RegisterToken(ctx context.Context, name, symbol, decimal string) error {
	ics20App := c.IBCAddresses["ics20App"]
	commands := []string{"rpc", "sendtx", "call"}
	commands = append(commands,
		"--to", ics20App,
		"--method", "registerCosmosToken",
		"--uri", c.GetHostRPCAddress(),
		"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
		"--key_password", c.testconfig.KeystorePassword,
		"--step_limit", "25000000000",
		"--nid", "0x3",
	)
	params := `{"name":"` + name + `","symbol":"` + symbol + `","decimals":"` + decimal + `"}`
	commands = append(commands, "--params", params)
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
	if err != nil {
		return err
	}
	json.Unmarshal(stdout, &output)
	_, err = c.waitForTxn(ctx, output)
	return err
}

func (c *IconRemotenet) GetSenderReceiverAddress() (string, string) {
	return c.testconfig.Sender, c.testconfig.Receiver
}
