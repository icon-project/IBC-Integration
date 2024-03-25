package icon

import (
	"context"
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
	testconfig   *testconfig.Chain
}

const (
	WalletKeyStore       = "/goloop/data/"
	MinterWalletKeyStore = "/goloop/data/minter.json"
)

func (c *IconRemotenet) CreateKey(ctx context.Context, keyName string) error {
	panic("implement me")

}

func NewIconRemotenet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, client *client.Client, network string, testconfig *testconfig.Chain) chains.Chain {
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
func (c *IconRemotenet) SendIBCTokenTransfer(ctx context.Context, sourceChannel, destinationChannel, port, receiver, chainID, ibcamount string) (string, error) {
	bankAppClient := c.IBCAddresses["bankAppClient"]
	commands := []string{"rpc", "sendtx", "call"}
	amount, denom, _ := strings.Cut(ibcamount, "/")
	// amount, denom := parts[0], parts[1]
	commands = append(commands,
		"--to", bankAppClient,
		"--method", "sendTransfer",
		"--uri", c.GetHostRPCAddress(),
		"--key_store", WalletKeyStore+c.testconfig.KeystoreFile,
		"--key_password", c.testconfig.KeystorePassword,
		"--value", amount,
		"--step_limit", "25000000000",
		"--nid", "0x3",
	)
	params := `{"denom":"` + denom + `","receiver":"` + receiver + `","amount":"` + amount + `","sourcePort":"` + port + `","sourceChannel":"` + sourceChannel + `","timeoutHeight":"5000000","timeoutRevisionNumber":"1"}`
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

// GetBalance fetches the current balance for a specific account address and denom.
func (c *IconRemotenet) GetWalletBalance(ctx context.Context, address string, denom string) (*big.Int, error) {
	bankApp := c.IBCAddresses["bankApp"]
	commands := []string{"rpc", "call"}
	commands = append(commands,
		"--to", bankApp,
		"--method", "balanceOf",
		"--uri", c.GetHostRPCAddress(),
	)
	params := `{"denom":"` + denom + `","account":"` + address + `"}`
	commands = append(commands, "--params", params)
	var output string
	stdout, _, err := c.Exec(ctx, commands, nil)
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
	panic("not implemented")
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
	time.Sleep(4 * time.Second)
	ibcAddress, err := c.DeployContractRemote(ctx, c.scorePaths["ibc_ics"], "")
	if err != nil {
		return nil, err
	}
	fmt.Println("IBC deployed at ", ibcAddress)

	client, err := c.DeployContractRemote(ctx, c.scorePaths["ibc_ics_client"], "ibcHandler="+ibcAddress)
	if err != nil {
		return nil, err
	}

	fmt.Println("IBC Clinet deployed at ", client)
	ics20client, err := c.DeployContractRemote(ctx, c.scorePaths["ibc_ics20_client"], "ibcHandler="+ibcAddress)
	if err != nil {
		return nil, err
	}
	fmt.Println("IBC ics20 Client deployed at ", ics20client)
	c.ExecuteContractRemote(context.Background(), ibcAddress, "registerClient", `{"clientType":"`+"07-tendermint"+`", "client":"`+client+`"}`)

	c.ExecuteContractRemote(context.Background(), ibcAddress, "registerClient", `{"hashType":"1","clientType":"`+"ics08-tendermint"+`", "client":"`+ics20client+`"}`)

	bankApp, err := c.DeployContractRemote(ctx, c.scorePaths["ics20bank"], "")
	if err != nil {
		return nil, err
	}

	bankClientApp, err := c.DeployContractRemote(ctx, c.scorePaths["ics20app"], `{"_ibcHandler":"`+ibcAddress+`", "_bank":"`+bankApp+`"}`)
	if err != nil {
		return nil, err
	}
	contracts.ContractAddress = map[string]string{
		"ibc":           ibcAddress,
		"client":        client,
		"bankAppClient": bankClientApp,
		"bankApp":       bankApp,
	}
	c.IBCAddresses = contracts.ContractAddress

	//assume Preps are already configured
	params := `{"networkTypeName":"eth", "name":"eth", "owner":"` + ibcAddress + `"}`
	ctx, _ = c.ExecuteContractRemote(ctx, "cx0000000000000000000000000000000000000001", "openBTPNetwork", params)
	//height, _ := ctx.Value("txResult").(icontypes.TransactionResult).BlockHeight.Int()
	id := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[0].Indexed[2]
	typeId := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[0].Indexed[1]
	btpNetworkId, _ := icontypes.HexInt(id).Int()
	btpNetworkTypeId, _ := icontypes.HexInt(typeId).Int()
	// //bind ics app
	portId := "transfer"
	params = `{"portId":"` + portId + `","moduleAddress":"` + bankClientApp + `"}`
	ctx, err = c.ExecuteContractRemote(ctx, ibcAddress, "bindPort", params)
	if err != nil {
		return ctx, err
	}

	//setup Operator
	minterWallet := "hxac1f0b75d2c05692fdea027fdd0d8475650c72d6"
	params = `{"account":"` + bankClientApp + `"}`
	ctx, err = c.ExecuteContractRemote(ctx, bankApp, "setupOperator", params)
	if err != nil {
		return ctx, err
	}

	params = `{"account":"` + minterWallet + `"}`
	ctx, err = c.ExecuteContractRemote(ctx, bankApp, "setupOperator", params)
	if err != nil {
		return ctx, err
	}

	amount := ibc.WalletAmount{
		Address: minterWallet,
		Amount:  1000,
	}

	err = c.SendFundsFromGodwallet(ctx, amount)
	if err != nil {
		return ctx, err
	}

	//mint to ics20app
	params = `{"account":"` + bankClientApp + `","denom":"vsp","amount":"10000000000000000"}`
	ctx, err = c.ExecuteContractRemote(ctx, bankApp, "mint", params)
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

	return context.WithValue(ctx, chains.Mykey("contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), nil
}
