package icon

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"math/big"
	"regexp"
	"strconv"
	"strings"
	"sync"
	"time"

	"github.com/icon-project/ibc-integration/libraries/go/common/tendermint"
	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/icon-project/icon-bridge/common/wallet"

	dockertypes "github.com/docker/docker/api/types"
	volumetypes "github.com/docker/docker/api/types/volume"
	"github.com/docker/docker/client"
	"github.com/gorilla/websocket"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/ibc-integration/test/internal/dockerutil"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"

	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"

	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	// "github.com/strangelove-ventures/interchaintest/v7/testutil"
	"go.uber.org/zap"
	"golang.org/x/sync/errgroup"
)

type IconLocalnet struct {
	log           *zap.Logger
	testName      string
	cfg           ibc.ChainConfig
	numValidators int
	numFullNodes  int
	FullNodes     IconNodes
	findTxMu      sync.Mutex
	keystorePath  string
	scorePaths    map[string]string
	IBCAddresses  map[string]string     `json:"addresses"`
	Wallets       map[string]ibc.Wallet `json:"wallets"`
}

func (c *IconLocalnet) CreateKey(ctx context.Context, keyName string) error {
	//TODO implement me
	panic("implement me")
}

func NewIconLocalnet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int, scorePaths map[string]string) chains.Chain {
	return &IconLocalnet{
		testName:      testName,
		cfg:           chainConfig,
		numValidators: numValidators,
		numFullNodes:  numFullNodes,
		log:           log,
		scorePaths:    scorePaths,
		Wallets:       map[string]ibc.Wallet{},
	}
}

// Config fetches the chain configuration.
func (c *IconLocalnet) Config() ibc.ChainConfig {
	return c.cfg
}

func (c *IconLocalnet) OverrideConfig(key string, value any) {
	if value == nil {
		return
	}
	c.cfg.ConfigFileOverrides[key] = value
}

// Initialize initializes node structs so that things like initializing keys can be done before starting the chain
func (c *IconLocalnet) Initialize(ctx context.Context, testName string, cli *client.Client, networkID string) error {
	chainCfg := c.Config()
	// c.pullImages(ctx, cli)
	image := chainCfg.Images[0]

	newFullNodes := make(IconNodes, c.numFullNodes)
	copy(newFullNodes, c.FullNodes)

	eg, egCtx := errgroup.WithContext(ctx)
	for i := len(c.FullNodes); i < c.numFullNodes; i++ {
		i := i
		eg.Go(func() error {
			fn, err := c.NewChainNode(egCtx, testName, cli, networkID, image, false)
			if err != nil {
				return err
			}
			fn.Index = i
			newFullNodes[i] = fn
			return nil
		})
	}
	if err := eg.Wait(); err != nil {
		return err
	}
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	c.FullNodes = newFullNodes
	return nil
}

func (c *IconLocalnet) pullImages(ctx context.Context, cli *client.Client) {
	for _, image := range c.Config().Images {
		rc, err := cli.ImagePull(
			ctx,
			image.Repository+":"+image.Version,
			dockertypes.ImagePullOptions{},
		)
		if err != nil {
			c.log.Error("Failed to pull image",
				zap.Error(err),
				zap.String("repository", image.Repository),
				zap.String("tag", image.Version),
			)
		} else {
			_, _ = io.Copy(io.Discard, rc)
			_ = rc.Close()
		}
	}
}

func (c *IconLocalnet) NewChainNode(
	ctx context.Context,
	testName string,
	cli *client.Client,
	networkID string,
	image ibc.DockerImage,
	validator bool,
) (*IconNode, error) {
	// Construct the ChainNode first so we can access its name.
	// The ChainNode's VolumeName cannot be set until after we create the volume.
	in := &IconNode{
		log:          c.log,
		Chain:        c,
		DockerClient: cli,
		NetworkID:    networkID,
		TestName:     testName,
		Image:        image,
	}

	v, err := cli.VolumeCreate(ctx, volumetypes.CreateOptions{
		Labels: map[string]string{
			dockerutil.CleanupLabel: testName,

			dockerutil.NodeOwnerLabel: in.Name(),
		},
	})
	if err != nil {
		return nil, fmt.Errorf("creating volume for chain node: %w", err)
	}
	in.VolumeName = v.Name

	if err := dockerutil.SetVolumeOwner(ctx, dockerutil.VolumeOwnerOptions{
		Log: c.log,

		Client: cli,

		VolumeName: v.Name,
		ImageRef:   image.Ref(),
		TestName:   testName,
		UidGid:     image.UidGid,
	}); err != nil {
		return nil, fmt.Errorf("set volume owner: %w", err)
	}
	return in, nil
}

// Start sets up everything needed (validators, gentx, fullnodes, peering, additional accounts) for chain to start from genesis.
func (c *IconLocalnet) Start(testName string, ctx context.Context, additionalGenesisWallets ...ibc.WalletAmount) error {
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	eg, egCtx := errgroup.WithContext(ctx)
	for _, n := range c.FullNodes {
		n := n
		eg.Go(func() error {
			if err := n.CreateNodeContainer(egCtx, additionalGenesisWallets...); err != nil {
				return err
			}
			// All (validators, gentx, fullnodes, peering, additional accounts) are included in the image itself.
			return n.StartContainer(ctx)
		})
	}
	return eg.Wait()
}

// Exec runs an arbitrary command using Chain's docker environment.
// Whether the invoked command is run in a one-off container or execing into an already running container
// is up to the chain implementation.
//
// "env" are environment variables in the format "MY_ENV_VAR=value"
func (c *IconLocalnet) Exec(ctx context.Context, cmd []string, env []string) (stdout []byte, stderr []byte, err error) {
	return c.getFullNode().Exec(ctx, cmd, env)
}

// ExportState exports the chain state at specific height.
func (c *IconLocalnet) ExportState(ctx context.Context, height int64) (string, error) {
	block, err := c.getFullNode().GetBlockByHeight(ctx, height)
	return block, err
}

// GetRPCAddress retrieves the rpc address that can be reached by other containers in the docker network.
func (c *IconLocalnet) GetRPCAddress() string {
	return c.getFullNode().Name() + ":9080"
}

// GetGRPCAddress retrieves the grpc address that can be reached by other containers in the docker network.
// Not Applicable for Icon
func (c *IconLocalnet) GetGRPCAddress() string {
	return ""
}

// GetHostRPCAddress returns the rpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
func (c *IconLocalnet) GetHostRPCAddress() string {
	return "http://" + c.getFullNode().HostRPCPort
}

// GetHostGRPCAddress returns the grpc address that can be reached by processes on the host machine.
// Note that this will not return a valid value until after Start returns.
// Not applicable for Icon
func (c *IconLocalnet) GetHostGRPCAddress() string {
	return ""
}

// HomeDir is the home directory of a node running in a docker container. Therefore, this maps to
// the container's filesystem (not the host).
func (c *IconLocalnet) HomeDir() string {
	return c.getFullNode().HomeDir()
}

func (c *IconLocalnet) createKeystore(ctx context.Context, keyName string) (string, string, error) {
	w := wallet.New()
	ks, err := wallet.KeyStoreFromWallet(w, []byte(keyName))
	if err != nil {
		return "", "", err
	}

	err = c.getFullNode().RestoreKeystore(ctx, ks, keyName)
	if err != nil {
		c.log.Error("fail to restore keystore", zap.Error(err))
		return "", "", err
	}
	ksd, err := wallet.NewKeyStoreData(ks)
	if err != nil {
		return "", "", err
	}
	key, err := wallet.DecryptICONKeyStore(ksd, []byte(keyName))
	if err != nil {
		return "", "", err
	}
	return w.Address(), hex.EncodeToString(key.Bytes()), nil
}

// RecoverKey recovers an existing user from a given mnemonic.
func (c *IconLocalnet) RecoverKey(ctx context.Context, name string, mnemonic string) error {
	panic("not implemented") // TODO: Implement
}

// GetAddress fetches the bech32 address for a test key on the "user" node (either the first fullnode or the first validator if no fullnodes).
func (c *IconLocalnet) GetAddress(ctx context.Context, keyName string) ([]byte, error) {
	addrInByte, err := json.Marshal(keyName)
	if err != nil {
		return nil, err
	}
	return addrInByte, nil
}

// SendFunds sends funds to a wallet from a user account.
func (c *IconLocalnet) SendFunds(ctx context.Context, keyName string, amount ibc.WalletAmount) error {
	c.CheckForKeyStore(ctx, keyName)

	cmd := c.getFullNode().NodeCommand("rpc", "sendtx", "transfer", "--key_store", c.keystorePath, "--key_password", keyName,
		"--to", amount.Address, "--value", fmt.Sprint(amount.Amount)+"000000000000000000", "--step_limit", "10000000000000")
	_, _, err := c.getFullNode().Exec(ctx, cmd, nil)
	return err
}

// SendIBCTransfer sends an IBC transfer returning a transaction or an error if the transfer failed.
func (c *IconLocalnet) SendIBCTransfer(ctx context.Context, channelID string, keyName string, amount ibc.WalletAmount, options ibc.TransferOptions) (ibc.Tx, error) {
	panic("not implemented") // TODO: Implement
}

// Height returns the current block height or an error if unable to get current height.
func (c *IconLocalnet) Height(ctx context.Context) (uint64, error) {
	return c.getFullNode().Height(ctx)
}

// GetGasFeesInNativeDenom gets the fees in native denom for an amount of spent gas.
func (c *IconLocalnet) GetGasFeesInNativeDenom(gasPaid int64) int64 {
	gasPrice, _ := strconv.ParseFloat(strings.Replace(c.cfg.GasPrices, c.cfg.Denom, "", 1), 64)
	fees := float64(gasPaid) * gasPrice
	return int64(fees)
}

// Acknowledgements returns all acknowledgements in a block at height.
func (c *IconLocalnet) Acknowledgements(ctx context.Context, height uint64) ([]ibc.PacketAcknowledgement, error) {
	panic("not implemented") // TODO: Implement
}

// Timeouts returns all timeouts in a block at height.
func (c *IconLocalnet) Timeouts(ctx context.Context, height uint64) ([]ibc.PacketTimeout, error) {
	panic("not implemented") // TODO: Implement
}

// BuildRelayerWallet will return a chain-specific wallet populated with the mnemonic so that the wallet can
// be restored in the relayer node using the mnemonic. After it is built, that address is included in
// genesis with some funds.
func (c *IconLocalnet) BuildRelayerWallet(ctx context.Context, keyName string) (ibc.Wallet, error) {
	return c.BuildWallet(ctx, keyName, "")
}

func (c *IconLocalnet) BuildWallet(ctx context.Context, keyName string, mnemonic string) (ibc.Wallet, error) {
	address, privateKey, err := c.createKeystore(ctx, keyName)
	if err != nil {
		return nil, fmt.Errorf("failed to create key with name %q on chain %s: %w", keyName, c.cfg.Name, err)

	}

	w := NewWallet(keyName, []byte(address), privateKey, c.cfg)
	c.Wallets[keyName] = w
	return w, nil
}

func (c *IconLocalnet) getFullNode() *IconNode {
	c.findTxMu.Lock()
	defer c.findTxMu.Unlock()
	if len(c.FullNodes) > 0 {
		// use first full node
		return c.FullNodes[0]
	}
	return c.FullNodes[0]
}

func (c *IconLocalnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	fn := c.getFullNode()
	return fn.FindTxs(ctx, height)
}

// GetBalance fetches the current balance for a specific account address and denom.
func (c *IconLocalnet) GetBalance(ctx context.Context, address string, denom string) (int64, error) {
	return c.getFullNode().GetBalance(ctx, address)
}

func (c *IconLocalnet) GetWalletBalance(ctx context.Context, address string, denom string) (*big.Int, error) {
	panic("not implemented")
}

func (c *IconLocalnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	var contracts chains.ContractKey
	time.Sleep(4 * time.Second)
	ownerAddr := c.CheckForKeyStore(ctx, keyName)
	if ownerAddr != nil {
		contracts.ContractOwner = map[string]string{
			keyName: ownerAddr.FormattedAddress(),
		}
	}
	ibcAddress, err := c.getFullNode().DeployContract(ctx, c.scorePaths["ibc"], c.keystorePath, "")
	if err != nil {
		return nil, err
	}

	client, err := c.getFullNode().DeployContract(ctx, c.scorePaths["client"], c.keystorePath, "ibcHandler="+ibcAddress)
	if err != nil {
		return nil, err
	}
	// TODO: variable clientType
	c.executeContract(context.Background(), ibcAddress, interchaintest.IBCOwnerAccount, "registerClient", `{"clientType":"`+"07-tendermint"+`", "client":"`+client+`"}`)

	contracts.ContractAddress = map[string]string{
		"ibc":    ibcAddress,
		"client": client,
	}
	c.IBCAddresses = contracts.ContractAddress

	params := `{"name": "test","country": "KOR","city": "Seoul","email": "prep@icon.foundation.com","website": "https://icon.kokoa.com","details": "https://icon.kokoa.com/json/details.json","p2pEndpoint": "localhost:9080"}`
	_, _ = c.executeContract(ctx, "cx0000000000000000000000000000000000000000", interchaintest.IBCOwnerAccount, "registerPRep", params)
	params = `{"pubKey":"0x04b3d972e61b4e8bf796c00e84030d22414a94d1830be528586e921584daadf934f74bd4a93146e5c3d34dc3af0e6dbcfe842318e939f8cc467707d6f4295d57e5"}`
	_, _ = c.executeContract(ctx, "cx0000000000000000000000000000000000000000", interchaintest.IBCOwnerAccount, "setPRepNodePublicKey", params)
	params = `{"networkTypeName":"eth", "name":"testNetwork", "owner":"` + ibcAddress + `"}`
	ctx, _ = c.executeContract(ctx, "cx0000000000000000000000000000000000000001", interchaintest.IBCOwnerAccount, "openBTPNetwork", params)
	//height, _ := ctx.Value("txResult").(icontypes.TransactionResult).BlockHeight.Int()
	id := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[1].Indexed[2]
	typeId := ctx.Value("txResult").(*icontypes.TransactionResult).EventLogs[1].Indexed[1]
	btpNetworkId, _ := icontypes.HexInt(id).Int()
	btpNetworkTypeId, _ := icontypes.HexInt(typeId).Int()

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
	}), err
}

func (c *IconLocalnet) SetupXCall(ctx context.Context, portId string, keyName string) error {
	testcase := ctx.Value("testcase").(string)
	nid := c.cfg.ChainID
	ibcAddress := c.IBCAddresses["ibc"]
	_ = c.CheckForKeyStore(ctx, keyName)
	xcall, err := c.getFullNode().DeployContract(ctx, c.scorePaths["xcall"], c.keystorePath, `{"networkId":"`+nid+`"}`)
	if err != nil {
		return err
	}

	connection, err := c.getFullNode().DeployContract(ctx, c.scorePaths["connection"], c.keystorePath, `{"_xCall":"`+xcall+`","_ibc":"`+ibcAddress+`", "_port":"`+portId+`"}`)
	if err != nil {
		return err
	}

	ctx, err = c.executeContract(context.Background(), ibcAddress, interchaintest.IBCOwnerAccount, "bindPort", `{"portId":"`+portId+`", "moduleAddress":"`+connection+`"}`)
	c.IBCAddresses[fmt.Sprintf("xcall-%s", testcase)] = xcall
	c.IBCAddresses[fmt.Sprintf("connection-%s", testcase)] = connection
	return err
}

func (c *IconLocalnet) PreGenesis() error {
	panic("unimplemented")
}

func (c *IconLocalnet) DeployXCallMockApp(ctx context.Context, connection chains.XCallConnection) error {
	testcase := ctx.Value("testcase").(string)
	connectionKey := fmt.Sprintf("connection-%s", testcase)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	xCall := c.IBCAddresses[xCallKey]
	params := `{"_callService":"` + xCall + `"}`
	dapp, err := c.getFullNode().DeployContract(ctx, c.scorePaths["dapp"], c.keystorePath, params)
	if err != nil {
		return err
	}
	params = `{"nid":"` + connection.CounterpartyNid + `", "source":"` + c.IBCAddresses[connectionKey] + `", "destination":"` + connection.CounterPartyConnection + `"}`
	ctx, err = c.executeContract(context.Background(), dapp, connection.KeyName, "addConnection", params)
	if err != nil {
		panic(err)
	}
	c.IBCAddresses[fmt.Sprintf("dapp-%s", testcase)] = dapp
	return nil
}

func (c *IconLocalnet) GetIBCAddress(key string) string {
	value, exist := c.IBCAddresses[key]
	if !exist {
		panic(fmt.Sprintf(`IBC address not exist %s`, key))
	}
	return value
}

func (c *IconLocalnet) BackupConfig() ([]byte, error) {
	wallets := make(map[string]interface{})
	for key, value := range c.Wallets {
		wallets[key] = map[string]string{
			"mnemonic":         value.Mnemonic(),
			"address":          hex.EncodeToString(value.Address()),
			"formattedAddress": value.FormattedAddress(),
		}
	}
	backup := map[string]interface{}{
		"addresses": c.IBCAddresses,
		"wallets":   wallets,
	}
	return json.MarshalIndent(backup, "", "\t")
}

func (c *IconLocalnet) RestoreConfig(backup []byte) error {
	result := make(map[string]interface{})
	err := json.Unmarshal(backup, &result)
	if err != nil {
		return err
	}
	c.IBCAddresses = result["addresses"].(map[string]string)
	wallets := make(map[string]ibc.Wallet)

	for key, value := range result["wallets"].(map[string]interface{}) {
		_value := value.(map[string]string)
		mnemonic := _value["mnemonic"]
		address, _ := hex.DecodeString(_value["address"])
		wallets[key] = NewWallet(key, address, mnemonic, c.Config())
	}
	c.Wallets = wallets
	return nil
}

func (c *IconLocalnet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	clientId := "07-tendermint-0"
	params, _ := json.Marshal(map[string]string{
		"connectionId":       connection.ConnectionId,
		"counterpartyPortId": connection.CounterPartyPortId,
		"counterpartyNid":    connection.CounterpartyNid,
		"clientId":           clientId,
		"timeoutHeight":      strconv.Itoa(connection.TimeoutHeight),
	})
	connectionAddress := c.IBCAddresses[fmt.Sprintf("connection-%s", testcase)]
	ctx, err := c.executeContract(context.Background(), connectionAddress, connection.KeyName, "configureConnection", string(params))
	if err != nil {
		return nil, err
	}
	params = []byte(`{"_nid":"` + connection.CounterpartyNid + `","_connection":"` + connectionAddress + `"}`)
	ctx, err = c.executeContract(context.Background(), c.IBCAddresses[fmt.Sprintf("xcall-%s", testcase)], connection.KeyName, "setDefaultConnection", string(params))
	if err != nil {
		return nil, err
	}

	return ctx, nil
}

func (c *IconLocalnet) SendPacketXCall(ctx context.Context, keyName, _to string, data, rollback []byte) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	// TODO: send fees
	var params = `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `"}`
	if rollback != nil {
		params = `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `", "_rollback":"` + hex.EncodeToString(rollback) + `"}`
	}
	ctx, err := c.executeContract(ctx, c.IBCAddresses[dappKey], keyName, "sendMessage", params)
	if err != nil {
		return nil, err
	}
	txn := ctx.Value("txResult").(*icontypes.TransactionResult)
	return context.WithValue(ctx, "sn", getSn(txn)), nil
}

// HasPacketReceipt returns the receipt of the packet sent to the target chain
func (c *IconLocalnet) IsPacketReceived(ctx context.Context, params map[string]interface{}, order ibc.Order) bool {
	if order == ibc.Ordered {
		sequence := params["sequence"].(uint64) //2
		ctx, err := c.QueryContract(ctx, c.IBCAddresses["ibc"], chains.GetNextSequenceReceive, params)
		if err != nil {
			fmt.Printf("Error--%v\n", err)
			return false
		}
		response, err := formatHexNumberFromResponse(ctx.Value("query-result").([]byte))

		if err != nil {
			fmt.Printf("Error--%v\n", err)
			return false
		}
		fmt.Printf("response[\"data\"]----%v", response)
		return sequence < response
	}
	ctx, _ = c.QueryContract(ctx, c.IBCAddresses["ibc"], chains.HasPacketReceipt, params)

	response, err := formatHexNumberFromResponse(ctx.Value("query-result").([]byte))
	if err != nil {
		fmt.Printf("Error--%v\n", err)
		return false
	}
	return response == 1
}

func formatHexNumberFromResponse(value []byte) (uint64, error) {
	pattern := `0x[0-9a-fA-F]+`
	regex := regexp.MustCompile(pattern)
	result := regex.FindString(string(value))
	if result == "" {
		return 0, fmt.Errorf("number not found")

	}

	response, err := strconv.ParseInt(result, 0, 64)
	if err != nil {
		return 0, err
	}
	return uint64(response), nil
}

// FindTargetXCallMessage returns the request id and the data of the message sent to the target chain
func (c *IconLocalnet) FindTargetXCallMessage(ctx context.Context, target chains.Chain, height uint64, to string) (*chains.XCallResponse, error) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	sn := ctx.Value("sn").(string)
	reqId, destData, err := target.FindCallMessage(ctx, height, c.cfg.ChainID+"/"+c.IBCAddresses[dappKey], to, sn)
	return &chains.XCallResponse{SerialNo: sn, RequestID: reqId, Data: destData}, err
}

func (c *IconLocalnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, to string, data, rollback []byte) (*chains.XCallResponse, error) {
	height, err := targetChain.(ibc.Chain).Height(ctx)
	if err != nil {
		return nil, err
	}
	// TODO: send fees
	ctx, err = c.SendPacketXCall(ctx, keyName, to, data, rollback)
	if err != nil {
		return nil, err
	}
	return c.FindTargetXCallMessage(ctx, targetChain, height, strings.Split(to, "/")[1])
}

func getSn(tx *icontypes.TransactionResult) string {
	for _, log := range tx.EventLogs {
		if string(log.Indexed[0]) == "CallMessageSent(Address,str,int)" {
			sn, _ := strconv.ParseInt(log.Indexed[3], 0, 64)
			return strconv.FormatInt(sn, 10)
		}
	}
	return ""
}

func (c *IconLocalnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error) {
	// TODO: send fees
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	params := `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `"}`
	ctx, _ = c.executeContract(context.Background(), c.IBCAddresses["xcall"], keyName, "sendCallMessage", params)
	sn := getSn(ctx.Value("txResult").(*icontypes.TransactionResult))
	reqId, destData, err := targetChain.FindCallMessage(ctx, height, c.cfg.ChainID+"/"+c.IBCAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, destData, err
}

func (c *IconLocalnet) ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	return c.executeContract(ctx, c.IBCAddresses[xCallKey], interchaintest.UserAccount, "executeCall", `{"_reqId":"`+reqId+`","_data":"`+data+`"}`)
}

func (c *IconLocalnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	ctx, err := c.executeContract(ctx, c.IBCAddresses[xCallKey], interchaintest.UserAccount, "executeRollback", `{"_sn":"`+sn+`"}`)
	if err != nil {
		return nil, err
	}
	txn := ctx.Value("txResult").(*icontypes.TransactionResult)
	sequence, err := icontypes.HexInt(txn.EventLogs[0].Indexed[1]).Int()
	return context.WithValue(ctx, "IsRollbackEventFound", fmt.Sprintf("%d", sequence) == sn), nil

}

func (c *IconLocalnet) FindCallMessage(ctx context.Context, startHeight uint64, from, to, sn string) (string, string, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	index := []*string{&from, &to, &sn}
	event, err := c.FindEvent(ctx, startHeight, xCallKey, "CallMessage(str,str,int,int,bytes)", index)
	if err != nil {
		return "", "", err
	}

	intHeight, _ := event.Height.Int()
	block, _ := c.getFullNode().Client.GetBlockByHeight(&icontypes.BlockHeightParam{Height: icontypes.NewHexInt(int64(intHeight - 1))})
	i, _ := event.Index.Int()
	tx := block.NormalTransactions[i]
	trResult, _ := c.getFullNode().TransactionResult(ctx, string(tx.TxHash))
	eventIndex, _ := event.Events[0].Int()
	reqId := trResult.EventLogs[eventIndex].Data[0]
	data := trResult.EventLogs[eventIndex].Data[1]
	return reqId, data, nil
}

func (c *IconLocalnet) FindCallResponse(ctx context.Context, startHeight uint64, sn string) (string, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	index := []*string{&sn}
	event, err := c.FindEvent(ctx, startHeight, xCallKey, "ResponseMessage(int,int)", index)
	if err != nil {
		return "", err
	}

	intHeight, _ := event.Height.Int()
	block, _ := c.getFullNode().Client.GetBlockByHeight(&icontypes.BlockHeightParam{Height: icontypes.NewHexInt(int64(intHeight - 1))})
	i, _ := event.Index.Int()
	tx := block.NormalTransactions[i]
	trResult, _ := c.getFullNode().TransactionResult(ctx, string(tx.TxHash))
	eventIndex, _ := event.Events[0].Int()
	code, _ := strconv.ParseInt(trResult.EventLogs[eventIndex].Data[0], 0, 64)

	return strconv.FormatInt(code, 10), nil
}

func (c *IconLocalnet) FindEvent(ctx context.Context, startHeight uint64, contract, signature string, index []*string) (*icontypes.EventNotification, error) {
	filter := icontypes.EventFilter{
		Addr:      icontypes.Address(c.IBCAddresses[contract]),
		Signature: signature,
		Indexed:   index,
	}

	// Create a context with a timeout of 16 seconds.
	_ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	// Create an event request with the given filter and start height.
	req := &icontypes.EventRequest{
		EventFilter: filter,
		Height:      icontypes.NewHexInt(int64(startHeight)),
	}
	channel := make(chan *icontypes.EventNotification)
	response := func(_ *websocket.Conn, v *icontypes.EventNotification) error {
		channel <- v
		return nil
	}
	errRespose := func(conn *websocket.Conn, err error) {}
	go func(ctx context.Context, req *icontypes.EventRequest, response func(*websocket.Conn, *icontypes.EventNotification) error, errRespose func(*websocket.Conn, error)) {
		defer func() {
			if err := recover(); err != nil {
				log.Printf("Recovered: %v", err)
			}
		}()
		if err := c.getFullNode().Client.MonitorEvent(ctx, req, response, errRespose); err != nil {
			log.Printf("MonitorEvent error: %v", err)
		}
	}(ctx, req, response, errRespose)

	select {
	case v := <-channel:
		return v, nil
	case <-_ctx.Done():
		return nil, fmt.Errorf("failed to find eventLog: %s", _ctx.Err())
	}
}

// DeployContract implements chains.Chain
func (c *IconLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Get contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessageKey("init-msg")).(chains.InitMessage)

	initMessage := c.getInitParams(ctx, contractName, ctxVal.Message)

	var contracts chains.ContractKey

	// Check if keystore is alreadry available for given keyName
	ownerAddr := c.CheckForKeyStore(ctx, keyName)
	if ownerAddr != nil {
		contracts.ContractOwner = map[string]string{
			keyName: ownerAddr.FormattedAddress(),
		}
	}

	// Get ScoreAddress
	scoreAddress, err := c.getFullNode().DeployContract(ctx, c.scorePaths[contractName], c.keystorePath, initMessage)

	contracts.ContractAddress = map[string]string{
		contractName: scoreAddress,
	}

	testcase := ctx.Value("testcase").(string)
	contract := fmt.Sprintf("%s-%s", contractName, testcase)
	c.IBCAddresses[contract] = scoreAddress
	return context.WithValue(ctx, chains.Mykey("contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

// executeContract implements chains.Chain
func (c *IconLocalnet) executeContract(ctx context.Context, contractAddress, keyName, methodName, params string) (context.Context, error) {
	// Check if keystore is alreadry available for given keyName
	c.CheckForKeyStore(ctx, keyName)

	hash, err := c.getFullNode().ExecuteContract(ctx, contractAddress, methodName, c.keystorePath, params)
	if err != nil {
		return nil, err
	}
	fmt.Printf("Transaction Hash: %s\n", hash)

	txHashByte, err := hex.DecodeString(strings.TrimPrefix(hash, "0x"))
	if err != nil {
		return nil, fmt.Errorf("error when executing contract %v ", err)
	}
	_, res, err := c.getFullNode().Client.WaitForResults(ctx, &icontypes.TransactionHashParam{Hash: icontypes.NewHexBytes(txHashByte)})
	if err != nil {
		return nil, err
	}
	if res.Status == "0x1" {
		return context.WithValue(ctx, "txResult", res), nil
	}
	//TODO add debug flag to print trace
	trace, err := c.getFullNode().GetDebugTrace(ctx, icontypes.NewHexBytes(txHashByte))
	if err == nil {
		logs, _ := json.Marshal(trace.Logs)
		fmt.Printf("---------debug trace start-----------\n%s\n---------debug trace end-----------\n", string(logs))
	}
	return ctx, fmt.Errorf("%s", res.Failure.MessageValue)
}

func (c *IconLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName string, params map[string]interface{}) (context.Context, error) {
	execMethodName, execParams := c.getExecuteParam(ctx, methodName, params)
	return c.executeContract(ctx, contractAddress, keyName, execMethodName, execParams)
}

// GetBlockByHeight implements chains.Chain
func (c *IconLocalnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("unimplemented")
}

// GetLastBlock implements chains.Chain
func (c *IconLocalnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	time.Sleep(2 * time.Second)
	h, err := c.getFullNode().Height(ctx)
	return context.WithValue(ctx, chains.LastBlock{}, h), err
}

func (c *IconLocalnet) InitEventListener(ctx context.Context, contract string) chains.EventListener {
	listener := NewIconEventListener(c, contract)
	return listener
}

func (c *IconLocalnet) CheckForTimeout(ctx context.Context, target chains.Chain, params map[string]interface{}, listener chains.EventListener) (context.Context, error) {

	var result = new(chains.TimeoutResponse)
	ctx, err := target.QueryContract(ctx, target.GetIBCAddress("ibc"), chains.HasPacketReceipt, params)
	if err != nil {
		result.IsPacketFound = false
	} else {
		response := ctx.Value("query-result").(map[string]interface{})
		result.IsPacketFound = response["data"].(bool)
	}

	filters := map[string]interface{}{
		"signature": "PacketTimeout(bytes)",
		"index":     []*string{},
	}
	event, err := listener.FindEvent(filters)

	if err != nil {
		result.HasTimeout = false
		return context.WithValue(ctx, "timeout-response", result), err
	}

	var packet = new(chantypes.Packet)
	var connStr = icontypes.HexBytes(event["indexed"][1])

	sn := fmt.Sprintf("%d", params["sequence"])
	_ = chains.HexBytesToProtoUnmarshal(connStr, packet)
	result.HasTimeout = packet != nil && packet.Sequence == params["sequence"].(uint64)

	ctx, err = c.ExecuteRollback(ctx, sn)
	result.HasRollbackCalled = err == nil && ctx.Value("IsRollbackEventFound").(bool)
	return context.WithValue(ctx, "timeout-response", result), nil
}

// QueryContract implements chains.Chain
func (c *IconLocalnet) QueryContract(ctx context.Context, contractAddress, methodName string, params map[string]interface{}) (context.Context, error) {
	time.Sleep(2 * time.Second)

	// get query msg
	query := c.GetQueryParam(methodName, params)
	_params, _ := json.Marshal(query.Value)
	output, err := c.getFullNode().QueryContract(ctx, contractAddress, query.MethodName, string(_params))

	chains.Response = output
	fmt.Printf("Response is : %s \n", output)
	return context.WithValue(ctx, "query-result", chains.Response), err

}

func (c *IconLocalnet) BuildWallets(ctx context.Context, keyName string) (ibc.Wallet, error) {
	w := c.CheckForKeyStore(ctx, keyName)
	if w == nil {
		return nil, fmt.Errorf("error keyName already exists")
	}

	amount := ibc.WalletAmount{
		Address: w.FormattedAddress(),
		Amount:  10000,
	}
	var err error

	err = c.SendFunds(ctx, interchaintest.FaucetAccountKeyName, amount)
	return w, err
}

func (c *IconLocalnet) GetClientName(suffix int) string {
	return fmt.Sprintf("07-tendermint-%d", suffix)
}

func (c *IconLocalnet) GetClientState(ctx context.Context, clientSuffix int) (any, error) {
	params := map[string]interface{}{
		"client_id": c.GetClientName(clientSuffix),
	}
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetClientState, params)
	if err != nil {
		return nil, err
	}

	var connStr icontypes.HexBytes

	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &connStr); err != nil {
		return nil, err
	}

	var conn = new(tendermint.ClientState)
	if err := chains.HexBytesToProtoUnmarshal(connStr, conn); err != nil {
		return nil, err
	}
	return conn, nil
}

// GetClientsCount returns the next sequence number for the client
func (c *IconLocalnet) GetClientsCount(ctx context.Context) (int, error) {
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextClientSequence, nil)
	if err != nil {
		return 0, err
	}
	var res string
	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &res); err != nil {
		return 0, err
	}
	n := new(big.Int)
	n.SetString(res, 0)
	return int(n.Int64()), nil
}

// GetConnectionState returns the next sequence number for the client
func (c *IconLocalnet) GetConnectionState(ctx context.Context, clientSuffix int) (*conntypes.ConnectionEnd, error) {
	params := map[string]interface{}{
		"connection_id": fmt.Sprintf(`connection-%d`, clientSuffix),
	}
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetConnection, params)
	if err != nil {
		return nil, err
	}

	var connStr icontypes.HexBytes

	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &connStr); err != nil {
		return nil, err
	}

	var conn = new(conntypes.ConnectionEnd)

	if err := chains.HexBytesToProtoUnmarshal(connStr, conn); err != nil {
		return nil, err
	}

	return conn, nil
}

// GetNextConnectionSequence returns the next sequence number for the client
func (c *IconLocalnet) GetNextConnectionSequence(ctx context.Context) (int, error) {
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextConnectionSequence, nil)
	if err != nil {
		return 0, err
	}
	var res string
	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &res); err != nil {
		return 0, err
	}
	n := new(big.Int)
	n.SetString(res, 0)
	return int(n.Int64()), nil
}

func (c *IconLocalnet) GetChannel(ctx context.Context, channelSuffix int, portID string) (*chantypes.Channel, error) {
	params := map[string]interface{}{
		"channel_id": fmt.Sprintf(`channel-%d`, channelSuffix),
		"port_id":    portID,
	}
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetChannel, params)
	if err != nil {
		return nil, err
	}

	var connStr icontypes.HexBytes

	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &connStr); err != nil {
		return nil, err
	}

	var channel = new(chantypes.Channel)
	if err := chains.HexBytesToProtoUnmarshal(connStr, channel); err != nil {
		return nil, err
	}
	return channel, nil
}

// GetNextChannelSequence returns the next sequence number for the client
func (c *IconLocalnet) GetNextChannelSequence(ctx context.Context) (int, error) {
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextChannelSequence, nil)
	if err != nil {
		return 0, err
	}
	var res string
	if err := json.Unmarshal(ctx.Value("query-result").([]byte), &res); err != nil {
		return 0, err
	}
	n := new(big.Int)
	n.SetString(res, 0)
	return int(n.Int64()), nil
}

// PauseNode pauses the node
func (c *IconLocalnet) PauseNode(ctx context.Context) error {
	return c.getFullNode().DockerClient.ContainerPause(ctx, c.getFullNode().ContainerID)
}

// UnpauseNode starts the paused node
func (c *IconLocalnet) UnpauseNode(ctx context.Context) error {
	return c.getFullNode().DockerClient.ContainerUnpause(ctx, c.getFullNode().ContainerID)
}

func (c *IconLocalnet) SendPacketMockDApp(ctx context.Context, targetChain chains.Chain, keyName string, params map[string]interface{}) (chains.PacketTransferResponse, error) {
	listener := targetChain.InitEventListener(ctx, "ibc")
	response := chains.PacketTransferResponse{}
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("mockdapp-%s", testcase)
	execMethodName, execParams := c.getExecuteParam(ctx, chains.SendMessage, params)
	ctx, err := c.executeContract(ctx, c.IBCAddresses[dappKey], keyName, execMethodName, execParams)
	if err != nil {
		return response, err
	}
	txn := ctx.Value("txResult").(*icontypes.TransactionResult)
	response.IsPacketSent = true

	var packet = chantypes.Packet{}
	var protoPacket = icontypes.HexBytes(txn.EventLogs[1].Indexed[1])
	_ = chains.HexBytesToProtoUnmarshal(protoPacket, &packet)
	response.Packet = packet
	filter := map[string]interface{}{
		"wasm-recv_packet.packet_sequence":    fmt.Sprintf("%d", packet.Sequence),
		"wasm-recv_packet.packet_src_port":    packet.SourcePort,
		"wasm-recv_packet.packet_src_channel": packet.SourceChannel,
	}
	event, err := listener.FindEvent(filter)
	response.IsPacketReceiptEventFound = event != nil
	return response, err

}

func (c *IconLocalnet) SetupIBCICS20(ctx context.Context, keyName string) (context.Context, error) {
	panic("not implemented")
}

func (c *IconLocalnet) SendIBCTokenTransfer(ctx context.Context, sourceChannel, destinationChannel, port, sender, receiver, chainID, ibcamount string, hopRequired bool) (string, error) {
	panic("not implemented")
}

func (c *IconLocalnet) RegisterToken(ctx context.Context, name, denom, decimal string) error {
	return nil
}

func (c *IconLocalnet) GetSenderReceiverAddress() (string, string) {
	return "", ""
}
