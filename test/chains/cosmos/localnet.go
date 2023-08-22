package cosmos

import (
	"context"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
	"strconv"
	"strings"
	"time"

	"github.com/avast/retry-go/v4"
	"github.com/cosmos/gogoproto/proto"
	clienttypes "github.com/cosmos/ibc-go/v7/modules/core/02-client/types"

	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	chantypes "github.com/cosmos/ibc-go/v7/modules/core/04-channel/types"

	icontypes "github.com/icon-project/ibc-integration/libraries/go/common/icon"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	rpchttp "github.com/cometbft/cometbft/rpc/client/http"
	ctypes "github.com/cometbft/cometbft/rpc/core/types"

	"go.uber.org/zap"
)

var contracts = chains.ContractKey{
	ContractAddress: make(map[string]string),
	ContractOwner:   make(map[string]string),
}

func NewCosmosLocalnet(testName string, log *zap.Logger, chainConfig ibc.ChainConfig, numValidators int, numFullNodes int, keyPassword string, contracts map[string]string) (chains.Chain, error) {
	chain := cosmos.NewCosmosChain(testName, chainConfig, numValidators, numFullNodes, log)
	return &CosmosLocalnet{
		CosmosChain: chain,
		cfg:         chain.Config(),
		keyName:     keyPassword,
		filepath:    contracts,
	}, nil
}

func (c *CosmosLocalnet) PreGenesis() error {
	ctx := context.TODO()
	chainNodes := c.Nodes()
	for _, cn := range chainNodes {
		if _, _, err := cn.ExecBin(ctx, "add-consumer-section"); err != nil {
			return err
		}
	}

	return nil
}

func (c *CosmosLocalnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	var contracts chains.ContractKey
	time.Sleep(4 * time.Second)

	ibcCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["ibc"])
	if err != nil {
		return nil, err
	}

	ibcAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, ibcCodeId, "{}", true, c.GetCommonArgs()...)
	if err != nil {
		return nil, err
	}

	clientCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["client"])
	if err != nil {
		return ctx, err
	}

	// Parameters here will be empty in the future
	clientAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, clientCodeId, `{"ibc_host":"`+ibcAddress+`"}`, true, c.GetCommonArgs()...)
	if err != nil {
		return nil, err
	}

	contracts.ContractAddress = map[string]string{
		"ibc":    ibcAddress,
		"client": clientAddress,
	}
	_, err = c.executeContract(context.Background(), ibcAddress, keyName, "register_client", `{"client_type":"iconclient", "client_address":"`+clientAddress+`"}`)
	if err != nil {
		return nil, err
	}

	c.IBCAddresses = contracts.ContractAddress
	overrides := map[string]any{
		"ibc-handler-address": ibcAddress,
		"start-height":        0,
		"block-interval":      6000,
	}

	cfg := c.cfg
	cfg.ConfigFileOverrides = overrides
	c.cfg = cfg

	return context.WithValue(ctx, chains.Mykey("contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), nil
}

func (c *CosmosLocalnet) SetupXCall(ctx context.Context, portId string, keyName string) error {
	testcase := ctx.Value("testcase").(string)
	ibcAddress := c.IBCAddresses["ibc"]
	denom := c.Config().Denom
	xCallCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["xcall"])
	if err != nil {
		return err
	}

	xCallAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, xCallCodeId, `{"network_id": "`+c.Config().ChainID+`", "denom":"`+denom+`"}`, true, c.GetCommonArgs()...)
	if err != nil {
		return err
	}

	connectionCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["connection"])
	if err != nil {
		return err
	}

	connectionAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, connectionCodeId, `{"port_id":"`+portId+`","xcall_address":"`+xCallAddress+`", "denom":"`+denom+`", "ibc_host":"`+ibcAddress+`"}`, true, c.GetCommonArgs()...)
	if err != nil {
		return err
	}

	_, err = c.ExecuteContract(context.Background(), ibcAddress, keyName, chains.BindPort, map[string]interface{}{
		"port_id": portId, "address": connectionAddress,
	})
	if err != nil {
		return err
	}

	c.IBCAddresses[fmt.Sprintf("xcall-%s", testcase)] = xCallAddress
	c.IBCAddresses[fmt.Sprintf("connection-%s", testcase)] = connectionAddress
	return nil
}

func (c *CosmosLocalnet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	clientId := c.GetClientName(0)

	params, _ := json.Marshal(map[string]interface{}{
		"connection_id":        connection.ConnectionId,
		"counterparty_port_id": connection.CounterPartyPortId,
		"counterparty_nid":     connection.CounterpartyNid,
		"client_id":            clientId,
		"timeout_height":       connection.TimeoutHeight,
	})

	//params := `{"connection_id":"` + connection.ConnectionId + `","counterparty_port_id":"` + connection.CounterPartyPortId + `","counterparty_nid":"` + connection.CounterpartyNid + `","client_id":"` + client_id + `","timeout_height":` + timeoutHeight + `}`
	_, err := c.executeContract(context.Background(), c.IBCAddresses[fmt.Sprintf("connection-%s", testcase)], connection.KeyName, "configure_connection", string(params))

	return ctx, err
}

func (c *CosmosLocalnet) GetIBCAddress(key string) string {
	value, exist := c.IBCAddresses[key]
	if !exist {
		panic(fmt.Sprintf(`IBC address not exist %s`, key))
	}
	return value
}

func (c *CosmosLocalnet) DeployXCallMockApp(ctx context.Context, connection chains.XCallConnection) error {
	testcase := ctx.Value("testcase").(string)
	connectionKey := fmt.Sprintf("connection-%s", testcase)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	xCall := c.IBCAddresses[xCallKey]
	dappCodeId, err := c.CosmosChain.StoreContract(ctx, connection.KeyName, c.filepath["dapp"])
	if err != nil {
		return err
	}

	dappAddress, err := c.CosmosChain.InstantiateContract(ctx, connection.KeyName, dappCodeId, `{"address":"`+xCall+`"}`, true, c.GetCommonArgs()...)
	if err != nil {
		return err
	}

	_, err = c.executeContract(context.Background(), dappAddress, connection.KeyName, "add_connection", `{"src_endpoint":"`+c.IBCAddresses[connectionKey]+`", "dest_endpoint":"`+connection.CounterPartyConnection+`","network_id":"`+connection.CounterpartyNid+`"}`)
	if err != nil {
		return err
	}
	c.IBCAddresses[fmt.Sprintf("dapp-%s", testcase)] = dappAddress

	return nil
}

func (c *CosmosLocalnet) InitEventListener(ctx context.Context, contract string) chains.EventListener {
	listener := NewCosmosEventListener(c, contract, 30*time.Second)
	listener.Start()
	return listener
}

func (c *CosmosLocalnet) CheckForTimeout(ctx context.Context, params map[string]interface{}, listener chains.EventListener) (context.Context, error) {
	var result = new(chains.TimeoutResponse)
	ctx, err := c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.HasPacketReceipt, params)
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
	var connStr = types.HexBytes(event["indexed"][1])

	_ = chains.HexBytesToProtoUnmarshal(connStr, packet)
	result.HasTimeout = packet != nil
	return context.WithValue(ctx, "timeout-response", result), nil
}

func (c *CosmosLocalnet) SendPacketXCall(ctx context.Context, keyName, _to string, data, rollback []byte) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)

	dataArray := strings.Join(strings.Fields(fmt.Sprintf("%d", data)), ",")
	rollbackArray := strings.Join(strings.Fields(fmt.Sprintf("%d", rollback)), ",")
	params := fmt.Sprintf(`{"to":"%s", "data":%s, "rollback":%s}`, _to, dataArray, rollbackArray)
	ctx, err := c.executeContract(ctx, c.IBCAddresses[dappKey], chains.FaucetAccountKeyName, "send_call_message", params)
	if err != nil {
		return nil, err
	}
	tx := ctx.Value("txResult").(*TxResul)
	return context.WithValue(ctx, "sn", c.findSn(tx)), nil
}

// FindTargetXCallMessage returns the request id and the data of the message sent to the target chain
func (c *CosmosLocalnet) FindTargetXCallMessage(ctx context.Context, target chains.Chain, height uint64, to string) (*chains.XCallResponse, error) {
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("dapp-%s", testcase)
	sn := ctx.Value("sn").(string)
	reqId, destData, err := target.FindCallMessage(ctx, height, c.cfg.ChainID+"/"+c.IBCAddresses[dappKey], to, sn)
	return &chains.XCallResponse{SerialNo: sn, RequestID: reqId, Data: destData}, err
}

func (c *CosmosLocalnet) SendPacketMockDApp(ctx context.Context, targetChain chains.Chain, keyName string, params map[string]interface{}) (chains.PacketTransferResponse, error) {
	listener := targetChain.InitEventListener(ctx, "ibc")
	defer listener.Stop()
	response := chains.PacketTransferResponse{}
	testcase := ctx.Value("testcase").(string)
	dappKey := fmt.Sprintf("mockdapp-%s", testcase)
	execMethodName, execParams := c.getExecuteParam(ctx, chains.SendMessage, params)
	ctx, err := c.executeContract(ctx, c.IBCAddresses[dappKey], keyName, execMethodName, execParams)
	if err != nil {
		return response, err
	}
	tx := ctx.Value("txResult").(*TxResul)
	response.IsPacketSent = true
	packet := c.findPacket(tx, "wasm-send_packet")
	response.Packet = packet
	value, _ := chains.ProtoMarshalToHexBytes(&packet)
	filters := map[string]interface{}{
		"signature": "RecvPacket(bytes)",
		"index":     []*string{&value},
	}
	event, err := listener.FindEvent(filters)
	response.IsPacketReceiptEventFound = event != nil
	return response, err

}

func (c *CosmosLocalnet) findPacket(tx *TxResul, eventType string) chantypes.Packet {
	var packet chantypes.Packet
	for _, event := range tx.Events {
		if event.Type == eventType {
			packet = chantypes.Packet{}
			for _, attribute := range event.Attributes {
				keyName, _ := base64.StdEncoding.DecodeString(attribute.Key)
				if string(keyName) == "packet_src_channel" {
					channel, _ := base64.StdEncoding.DecodeString(attribute.Value)
					packet.SourceChannel = string(channel)
				} else if string(keyName) == "packet_src_port" {
					port, _ := base64.StdEncoding.DecodeString(attribute.Value)
					packet.SourcePort = string(port)
				} else if string(keyName) == "packet_sequence" {
					sn, _ := base64.StdEncoding.DecodeString(attribute.Value)
					sequence, _ := strconv.Atoi(string(sn))
					packet.Sequence = uint64(sequence)
				} else if string(keyName) == "packet_timeout_height" {
					_hex, _ := base64.StdEncoding.DecodeString(attribute.Value)
					revisionSplit := strings.Split(string(_hex), "-")
					if len(revisionSplit) != 2 {
						continue
					}
					revisionNumberString := revisionSplit[0]
					revisionNumber, err := strconv.ParseUint(revisionNumberString, 10, 64)
					if err != nil {
						continue
					}
					revisionHeightString := revisionSplit[1]
					revisionHeight, err := strconv.ParseUint(revisionHeightString, 10, 64)
					if err != nil {
						continue
					}
					packet.TimeoutHeight = clienttypes.Height{
						RevisionNumber: revisionNumber,
						RevisionHeight: revisionHeight,
					}
				} else if string(keyName) == "packet_data_hex" {
					_hex, _ := base64.StdEncoding.DecodeString(attribute.Value)
					data, _ := hex.DecodeString(string(_hex))
					packet.Data = data
				} else if string(keyName) == "packet_dst_channel" {
					channel, _ := base64.StdEncoding.DecodeString(attribute.Value)
					packet.DestinationChannel = string(channel)
				} else if string(keyName) == "packet_dst_port" {
					port, _ := base64.StdEncoding.DecodeString(attribute.Value)
					packet.DestinationPort = string(port)
				}
			}
			break
		}
	}
	return packet
}

func (c *CosmosLocalnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, to string, data, rollback []byte) (*chains.XCallResponse, error) {
	height, err := targetChain.(ibc.Chain).Height(ctx)
	if err != nil {
		return nil, err
	}
	ctx, err = c.SendPacketXCall(ctx, keyName, to, data, rollback)
	if err != nil {
		return nil, err
	}
	return c.FindTargetXCallMessage(ctx, targetChain, height, strings.Split(to, "/")[1])
}

func (c *CosmosLocalnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error) {
	dataArray := strings.Join(strings.Fields(fmt.Sprintf("%d", data)), ",")
	params := fmt.Sprintf(`{"to":"%s", "data":%s}`, _to, dataArray)
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	ctx, err := c.executeContract(context.Background(), c.IBCAddresses["xcall"], keyName, "send_call_message", params)
	if err != nil {
		return "", "", "", err
	}

	tx := ctx.Value("txResult").(*TxResul)
	sn := c.findSn(tx)
	reqId, destData, err := targetChain.FindCallMessage(ctx, height, c.cfg.ChainID+"/"+c.IBCAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, destData, err
}

func (c *CosmosLocalnet) findSn(tx *TxResul) string {
	// find better way to parse events
	for _, event := range tx.Events {
		if event.Type == "wasm-CallMessageSent" {
			for _, attribute := range event.Attributes {
				keyName, _ := base64.StdEncoding.DecodeString(attribute.Key)
				if string(keyName) == "sn" {
					sn, _ := base64.StdEncoding.DecodeString(attribute.Value)
					return string(sn)
				}
			}
		}
	}
	return ""
}

// IsPacketReceived returns the receipt of the packet sent to the target chain
func (c *CosmosLocalnet) IsPacketReceived(ctx context.Context, params map[string]interface{}) bool {
	ctx, err := c.QueryContract(ctx, c.IBCAddresses["ibc"], chains.HasPacketReceipt, params)
	if err != nil {
		fmt.Printf("Error--%v\n", err)
		return false
	}
	response := ctx.Value("query-result").(map[string]interface{})
	return response["data"].(bool)
}

func (c *CosmosLocalnet) ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	return c.executeContract(ctx, c.IBCAddresses[xCallKey], chains.FaucetAccountKeyName, "execute_call", `{"request_id":"`+reqId+`", "data":`+data+`}`)
}

func (c *CosmosLocalnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	return c.executeContract(context.Background(), c.IBCAddresses[xCallKey], chains.FaucetAccountKeyName, "execute_rollback", `{"sequence_no":"`+sn+`"}`)
}

func (c *CosmosLocalnet) FindCallMessage(ctx context.Context, startHeight uint64, from, to, sn string) (string, string, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	index := strings.Join([]string{
		fmt.Sprintf("wasm-CallMessage.from CONTAINS '%s'", from),
		fmt.Sprintf("wasm-CallMessage.to CONTAINS '%s'", to),
		fmt.Sprintf("wasm-CallMessage.sn CONTAINS '%s'", sn),
	}, " AND ")
	event, err := c.FindEvent(ctx, startHeight, xCallKey, index)
	if err != nil {
		return "", "", err
	}

	return event.Events["wasm-CallMessage.reqId"][0], event.Events["wasm-CallMessage.data"][0], nil

}

func (c *CosmosLocalnet) FindCallResponse(ctx context.Context, startHeight uint64, sn string) (string, error) {
	testcase := ctx.Value("testcase").(string)
	xCallKey := fmt.Sprintf("xcall-%s", testcase)
	index := fmt.Sprintf("wasm-ResponseMessage.sn CONTAINS '%s'", sn)
	event, err := c.FindEvent(ctx, startHeight, xCallKey, index)
	if err != nil {
		return "", err
	}

	return event.Events["wasm-ResponseMessage.code"][0], nil
}

func (c *CosmosLocalnet) FindEvent(ctx context.Context, startHeight uint64, contract, index string) (*ctypes.ResultEvent, error) {
	endpoint := c.GetHostRPCAddress()
	client, err := rpchttp.New(endpoint, "/websocket")
	if err != nil {
		return nil, err
	}

	err = client.Start()
	if err != nil {
		return nil, err
	}
	defer client.Stop()
	ctx, cancel := context.WithTimeout(context.Background(), 20*time.Second)
	defer cancel()
	query := strings.Join([]string{"tm.event = 'Tx'",
		fmt.Sprintf("tx.height >= %d ", startHeight),
		"message.module = 'wasm'",
		fmt.Sprintf("wasm._contract_address = '%s'", c.IBCAddresses[contract]),
		index,
	}, " AND ")
	channel, err := client.Subscribe(ctx, "wasm-client", query)
	if err != nil {
		return nil, err
	}

	select {
	case event := <-channel:
		return &event, nil
	case <-ctx.Done():
		return nil, fmt.Errorf("failed to find eventLog")
	}
}

func (c *CosmosLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Fund user to deploy contract
	contractOwner, _, _ := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)

	// Get contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	initMsg := ctx.Value(chains.InitMessageKey("init-msg")).(chains.InitMessage)

	contractName := strings.ToLower(ctxValue.ContractName)
	codeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath[contractName])
	if err != nil {
		return ctx, err
	}

	initMessage := c.getInitParams(ctx, contractName, initMsg.Message)
	address, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, codeId, initMessage, true, c.GetCommonArgs()...)
	if err != nil {
		return nil, err
	}

	testcase := ctx.Value("testcase").(string)
	contract := fmt.Sprintf("%s-%s", contractName, testcase)
	c.IBCAddresses[contract] = address

	return context.WithValue(ctx, chains.Mykey("contract Names"), contracts), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context, contractAddress, methodName string, params map[string]interface{}) (context.Context, error) {
	// wait for few blocks after executing before querying
	time.Sleep(2 * time.Second)

	// get query msg
	query := c.GetQueryParam(methodName, params)
	chains.Response = ""
	err := c.CosmosChain.QueryContract(ctx, contractAddress, query, &chains.Response)
	fmt.Printf("Response is : %s \n", chains.Response)
	return context.WithValue(ctx, "query-result", chains.Response), err
	//return context.WithValue(ctx, "txResult", chains.Response.(map[string]interface{})["data"]), nil

}

func (c *CosmosLocalnet) executeContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	txHash, err := c.getFullNode().ExecTx(ctx, keyName,
		"wasm", "execute", contractAddress, `{"`+methodName+`":`+param+`}`, "--gas", "auto")
	if err != nil || txHash == "" {
		return nil, err
	}
	tx, err := c.getTransaction(txHash)
	if err != nil {
		return nil, err
	}
	return context.WithValue(ctx, "txResult", tx), nil
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName string, params map[string]interface{}) (context.Context, error) {
	execMethodName, execParams := c.getExecuteParam(ctx, methodName, params)
	return c.executeContract(ctx, contractAddress, keyName, execMethodName, execParams)
}

func (c *CosmosLocalnet) getTransaction(txHash string) (*TxResul, error) {
	// Retry because sometimes the tx is not committed to state yet.
	var result TxResul

	err := retry.Do(func() error {
		var err error
		stdout, _, err := c.getFullNode().ExecQuery(context.Background(), "tx", txHash)
		err = json.Unmarshal(stdout, &result)
		return err
	},
		// retry for total of 6 seconds
		retry.Attempts(30),
		retry.Delay(200*time.Millisecond),
		retry.DelayType(retry.FixedDelay),
		retry.LastErrorOnly(true),
	)
	return &result, err
}

func (c *CosmosLocalnet) getFullNode() *cosmos.ChainNode {
	if len(c.FullNodes) > 0 {
		// use first full node
		return c.FullNodes[0]
	}
	// use first validator
	return c.Validators[0]
}

func (c *CosmosLocalnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	h, err := c.CosmosChain.Height(ctx)
	return context.WithValue(ctx, chains.LastBlock{}, h), err
}

func (c *CosmosLocalnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosLocalnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	return nil, nil
}

func (c *CosmosLocalnet) BuildWallets(ctx context.Context, keyName string) error {
	// Build Wallet and fund user
	_, _, err := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)
	return err
}

func (c *CosmosLocalnet) GetCommonArgs() []string {
	return []string{"--gas", "auto"}
}

func (c *CosmosLocalnet) GetClientName(suffix int) string {
	return fmt.Sprintf("iconclient-%d", suffix)
}

func (c *CosmosLocalnet) GetClientState(ctx context.Context, clientSuffix int) (any, error) {

	params := map[string]interface{}{
		"client_id": c.GetClientName(clientSuffix),
	}
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetClientState, params)
	if err != nil {
		return nil, err
	}
	res := ctx.Value("query-result").(map[string]interface{})

	var data = res["data"].(string)

	hexDecoded, err := hex.DecodeString(data)

	if err != nil {
		return nil, err
	}

	cdc := icon.MakeCodec()
	clientState, err := clienttypes.UnmarshalClientState(cdc, hexDecoded)
	if err != nil {
		return nil, err
	}
	return clientState.(*icontypes.ClientState), nil
}

// GetClientsCount returns the next sequence number for the client
func (c *CosmosLocalnet) GetClientsCount(ctx context.Context) (int, error) {
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextClientSequence, map[string]interface{}{})
	if err != nil {
		return 0, err
	}
	res := ctx.Value("query-result").(map[string]interface{})
	var data = res["data"].(float64)
	return int(data), nil
}

// GetConnectionState returns the next sequence number for the client
func (c *CosmosLocalnet) GetConnectionState(ctx context.Context, connectionPrefix int) (*conntypes.ConnectionEnd, error) {
	params := map[string]interface{}{
		"connection_id": fmt.Sprintf("connection-%d", connectionPrefix),
	}
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetConnection, params)
	if err != nil {
		return nil, err
	}
	res := ctx.Value("query-result").(map[string]interface{})

	data := res["data"].(string)

	hexDecoded, err := hex.DecodeString(data)

	if err != nil {
		return nil, err
	}

	var conn = new(conntypes.ConnectionEnd)

	if err := proto.Unmarshal(hexDecoded, conn); err != nil {
		return nil, err
	}

	return conn, nil
}

// GetNextConnectionSequence returns the next sequence number for the client
func (c *CosmosLocalnet) GetNextConnectionSequence(ctx context.Context) (int, error) {
	params := map[string]interface{}{}
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextConnectionSequence, params)
	if err != nil {
		return 0, err
	}
	res := ctx.Value("query-result").(map[string]interface{})

	count := res["data"].(float64)
	return int(count), err
}

// GetChannel returns the next sequence number for the client
func (c *CosmosLocalnet) GetChannel(ctx context.Context, connectionPrefix int, portID string) (*chantypes.Channel, error) {
	params := map[string]interface{}{
		"channel_id": fmt.Sprintf("channel-%d", connectionPrefix),
		"port_id":    portID,
	}
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetChannel, params)
	if err != nil {
		return nil, err
	}
	res := ctx.Value("query-result").(map[string]interface{})

	data := res["data"].(string)

	hexDecoded, err := hex.DecodeString(data)

	if err != nil {
		return nil, err
	}

	var channel = new(chantypes.Channel)

	return channel, proto.Unmarshal(hexDecoded, channel)
}

// GetNextChannelSequence returns the next sequence number for the client
func (c *CosmosLocalnet) GetNextChannelSequence(ctx context.Context) (int, error) {
	params := map[string]interface{}{}
	var err error
	ctx, err = c.QueryContract(ctx, c.GetIBCAddress("ibc"), chains.GetNextChannelSequence, params)
	if err != nil {
		return 0, err
	}
	res := ctx.Value("query-result").(map[string]interface{})

	count := res["data"].(float64)
	return int(count), err
}

// PauseNode halts a node
func (c *CosmosLocalnet) PauseNode(ctx context.Context) error {
	return c.getFullNode().Client.Stop()
}

// UnpauseNode restarts a node
func (c *CosmosLocalnet) UnpauseNode(ctx context.Context) error {
	return c.getFullNode().Client.Start()
}
