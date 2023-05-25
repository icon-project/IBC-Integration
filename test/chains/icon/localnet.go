package icon

import (
	"context"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"io"
	"strconv"
	"strings"
	"sync"
	"time"

	dockertypes "github.com/docker/docker/api/types"
	volumetypes "github.com/docker/docker/api/types/volume"
	"github.com/docker/docker/client"
	"github.com/gorilla/websocket"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/ibc-integration/test/internal/dockerutil"

	icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	// "github.com/strangelove-ventures/interchaintest/v7/testutil"
	"go.uber.org/zap"
	"golang.org/x/sync/errgroup"
)

type IconLocalnet struct {
	log                       *zap.Logger
	testName                  string
	cfg                       ibc.ChainConfig
	numValidators             int
	numFullNodes              int
	FullNodes                 IconNodes
	findTxMu                  sync.Mutex
	keystorePath, keyPassword string
	scorePaths                map[string]string
	ibcAddresses              map[string]string
}

func NewIconLocalnet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int, keystorePath string, keyPassword string, scorePaths map[string]string) chains.Chain {
	return &IconLocalnet{
		testName:      testName,
		cfg:           chainConfig,
		numValidators: numValidators,
		numFullNodes:  numFullNodes,
		log:           log,
		keystorePath:  keystorePath,
		keyPassword:   keyPassword,
		scorePaths:    scorePaths,
	}
}

// Config fetches the chain configuration.
func (c *IconLocalnet) Config() ibc.ChainConfig {
	return c.cfg
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

	v, err := cli.VolumeCreate(ctx, volumetypes.VolumeCreateBody{
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
	_ = in.CopyFile(ctx, c.keystorePath, "gochain.json")
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
			if err := n.CreateNodeContainer(egCtx); err != nil {
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

// CreateKey creates a test key in the "user" node (either the first fullnode or the first validator if no fullnodes).
func (c *IconLocalnet) CreateKey(ctx context.Context, password string) error {
	return c.getFullNode().CreateKey(ctx, password)
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

	cmd := c.getFullNode().NodeCommand("rpc", "sendtx", "transfer", "--key_store", c.keystorePath, "--key_password", c.keyPassword,
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
	if err := c.CreateKey(ctx, keyName); err != nil {
		return nil, fmt.Errorf("failed to create key with name %q on chain %s: %w", keyName, c.cfg.Name, err)
	}
	addr := c.getFullNode().Address
	addrBytes, err := c.GetAddress(ctx, addr)
	if err != nil {
		return nil, fmt.Errorf("failed to get account address for key %q on chain %s: %w", keyName, c.cfg.Name, err)
	}

	return NewWallet(keyName, addrBytes, mnemonic, c.cfg), nil
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

func (c *IconLocalnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	var contracts chains.ContractKey
	time.Sleep(4 * time.Second)
	ownerAddr := c.CheckForKeyStore(ctx, keyName)
	if ownerAddr != "" {
		contracts.ContractOwner = map[string]string{
			keyName: ownerAddr,
		}
	}
	ibcAddress, err := c.getFullNode().DeployContract(ctx, c.scorePaths["ibc"], c.keystorePath, "")
	if err != nil {
		return nil, err
	}

	client, err := c.getFullNode().DeployContract(ctx, c.scorePaths["client"], c.keystorePath, "") //"ibcHandler="+ibcAddress)
	if err != nil {
		return nil, err
	}
	// TODO: variable clientType
	c.ExecuteContract(context.Background(), ibcAddress, "gochain", "registerClient", `{"clientType":"`+"07-tendermint"+`", "client":"`+client+`"}`)

	nid := c.cfg.ChainID
	xcall, err := c.getFullNode().DeployContract(ctx, c.scorePaths["xcall"], c.keystorePath, `{"networkId":"`+nid+`"}`)
	if err != nil {
		return nil, err
	}

	connection, err := c.getFullNode().DeployContract(ctx, c.scorePaths["connection"], c.keystorePath, `{"_xCall":"`+xcall+`","_ibc":"`+ibcAddress+`", "_nid":"`+nid+`", "_timeoutHeight":"100"}`)
	if err != nil {
		return nil, err
	}

	dapp, err := c.getFullNode().DeployContract(ctx, c.scorePaths["dapp"], c.keystorePath, `{"_callService":"`+xcall+`"}`)
	if err != nil {
		return nil, err
	}

	c.ExecuteContract(context.Background(), ibcAddress, "gochain", "bindPort", `{"portId":"mock", "moduleAddress":"`+connection+`"}`)

	contracts.ContractAddress = map[string]string{
		"ibc":        ibcAddress,
		"client":     client,
		"xcall":      xcall,
		"connection": connection,
		"dapp":       dapp,
	}
	c.ibcAddresses = contracts.ContractAddress

	params := `{"name": "test","country": "KOR","city": "Seoul","email": "prep@icon.foundation.com","website": "https://icon.kokoa.com","details": "https://icon.kokoa.com/json/details.json","p2pEndpoint": "localhost:9080"}`
	_, _ = c.ExecuteContract(ctx, "cx0000000000000000000000000000000000000000", "gochain", "registerPRep", params)
	params = `{"pubKey":"0x04b3d972e61b4e8bf796c00e84030d22414a94d1830be528586e921584daadf934f74bd4a93146e5c3d34dc3af0e6dbcfe842318e939f8cc467707d6f4295d57e5"}`
	_, _ = c.ExecuteContract(ctx, "cx0000000000000000000000000000000000000000", "gochain", "setPRepNodePublicKey", params)
	params = `{"networkTypeName":"eth", "name":"testNetwork", "owner":"` + ibcAddress + `"}`
	ctx, _ = c.ExecuteContract(ctx, "cx0000000000000000000000000000000000000001", "gochain", "openBTPNetwork", params)
	height, _ := ctx.Value("txResult").(icontypes.TransactionResult).BlockHeight.Int()
	id := ctx.Value("txResult").(icontypes.TransactionResult).EventLogs[1].Indexed[2]
	btpNetworkId, _ := icontypes.HexInt(id).Int()
	overrides := map[string]any{
		"ibc-handler-address": ibcAddress,
		"start-btp-height":    height + 1,
		"btp-network-id":      btpNetworkId,
	}

	cfg := c.cfg
	cfg.ConfigFileOverrides = overrides
	c.cfg = cfg

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

func (c *IconLocalnet) ConfigureBaseConnection(ctx context.Context, keyName, channel, counterpartyNid, counterpartyConnection string) (context.Context, error) {
	ctx, err := c.ExecuteContract(context.Background(), c.ibcAddresses["connection"], keyName, "configureChannel", `{"channelId":"`+channel+`", "counterpartyNid":"`+counterpartyNid+`"}`)
	if err != nil {
		return ctx, err
	}

	ctx, err = c.ExecuteContract(context.Background(), c.ibcAddresses["dapp"], keyName, "addConnection", `{"nid":"`+counterpartyNid+`", "source":"`+c.ibcAddresses["connection"]+`", "destination":"`+counterpartyConnection+`"}`)
	return ctx, err
}

func (c *IconLocalnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data, rollback []byte) (string, string, error) {
	// TODO: send fees
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	var params string
	if rollback == nil {
		params = `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `"}`
	} else {
		params = `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `", "_rollback":"` + hex.EncodeToString(rollback) + `"}`
	}

	ctx, _ = c.ExecuteContract(context.Background(), c.ibcAddresses["dapp"], keyName, "sendMessage", params)
	sn := getSn(ctx.Value("txResult").(icontypes.TransactionResult))
	reqId, err := targetChain.FindCallMessage(ctx, int64(height), c.cfg.ChainID+"/"+c.ibcAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, err
}

func getSn(tx icontypes.TransactionResult) string {
	for _, log := range tx.EventLogs {
		if string(log.Indexed[0]) == "CallMessageSent(Address,str,int)" {
			return log.Indexed[3]
		}
	}
	return ""
}

func (c *IconLocalnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, error) {
	// TODO: send fees
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	params := `{"_to":"` + _to + `", "_data":"` + hex.EncodeToString(data) + `"}`
	ctx, _ = c.ExecuteContract(context.Background(), c.ibcAddresses["xcall"], keyName, "sendCallMessage", params)
	sn := getSn(ctx.Value("txResult").(icontypes.TransactionResult))
	reqId, err := targetChain.FindCallMessage(ctx, int64(height), c.cfg.ChainID+"/"+c.ibcAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, err
}

func (c *IconLocalnet) ExecuteCall(ctx context.Context, reqId string) (context.Context, error) {
	return c.ExecuteContract(context.Background(), c.ibcAddresses["xcall"], "gochain", "executeCall", `{"_reqId":"`+reqId+`"}`)
}

func (c *IconLocalnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	return c.ExecuteContract(context.Background(), c.ibcAddresses["xcall"], "gochain", "executeRollback", `{"_sn":"`+sn+`"}`)
}

func (c *IconLocalnet) FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, error) {
	index := []*string{&from, &to, &sn}
	filter := icontypes.EventFilter{
		Addr:      icontypes.Address(c.ibcAddresses["xcall"]),
		Signature: "CallMessage(str,str,int,int)",
		Indexed:   index,
	}
	socketContext := context.Background()
	req := icontypes.EventRequest{
		EventFilter: filter,
		Height:      icontypes.NewHexInt(startHeight),
	}
	channel := make(chan *icontypes.EventNotification)
	response := func(conn *websocket.Conn, v *icontypes.EventNotification) error {
		channel <- v
		return nil
	}
	errRespose := func(conn *websocket.Conn, err error) {}
	go c.getFullNode().Client.MonitorEvent(socketContext, &req, response, errRespose)

	select {
	case v := <-channel:
		intHeight, _ := v.Height.Int()
		block2, _ := c.getFullNode().Client.GetBlockByHeight(&icontypes.BlockHeightParam{Height: icontypes.NewHexInt(int64(intHeight - 1))})
		index, _ := v.Index.Int()
		tx := block2.NormalTransactions[index]
		trResult, _ := c.getFullNode().TransactionResult(ctx, string(tx.TxHash))
		eventIndex, _ := v.Events[0].Int()
		reqId := trResult.EventLogs[eventIndex].Data[0]
		return reqId, nil
	case <-time.After(20 * time.Second):
		return "", fmt.Errorf("failed to find eventLog")
	}
}

// DeployContract implements chains.Chain
func (c *IconLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := ctxValue.ContractName

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg

	if contractName == "xcall" {
		ctxValue := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
		bmcAddr := ctxValue.ContractAddress["bmc"]
		initMessage = initMessage + bmcAddr
	}
	var contracts chains.ContractKey

	// Check if keystore is alreadry available for given keyName
	ownerAddr := c.CheckForKeyStore(ctx, keyName)
	if ownerAddr != "" {
		contracts.ContractOwner = map[string]string{
			keyName: ownerAddr,
		}
	}

	// Get ScoreAddress
	scoreAddress, err := c.getFullNode().DeployContract(ctx, c.scorePaths[contractName], c.keystorePath, initMessage)
	fmt.Println(err)
	contracts.ContractAddress = map[string]string{
		contractName: scoreAddress,
	}
	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

// ExecuteContract implements chains.Chain
func (c *IconLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	// Check if keystore is alreadry available for given keyName
	c.CheckForKeyStore(ctx, keyName)

	ctx, execMethodName, params := c.GetExecuteParam(ctx, methodName, param)
	hash, err := c.getFullNode().ExecuteContract(ctx, contractAddress, execMethodName, c.keystorePath, params)
	if err != nil {
		return ctx, err
	}
	fmt.Printf("Transaction Hash: %s\n", hash)

	// wait for few blocks to finish
	time.Sleep(2 * time.Second)
	trResult, _ := c.getFullNode().TransactionResult(ctx, hash)
	if trResult.Status == "0x1" {
		return context.WithValue(ctx, "txResult", trResult), nil
	} else {
		return ctx, fmt.Errorf("%s", trResult.Failure.MessageValue)
	}

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

// QueryContract implements chains.Chain
func (c *IconLocalnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	time.Sleep(2 * time.Second)

	// get query msg
	queryMsg := c.GetQueryParam(methodName)
	output, err := c.getFullNode().QueryContract(ctx, contractAddress, queryMsg, "")
	chains.Response = output
	fmt.Printf("Response is : %s \n", output)
	return ctx, err
}

func (c *IconLocalnet) BuildWallets(ctx context.Context, keyName string) error {
	address := c.CheckForKeyStore(ctx, keyName)
	if address == "" {
		return nil
	}

	amount := ibc.WalletAmount{
		Address: address,
		Amount:  10000,
	}
	return c.SendFunds(ctx, "gochain", amount)
}
