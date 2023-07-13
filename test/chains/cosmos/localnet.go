package cosmos

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/avast/retry-go/v4"
	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"

	rpchttp "github.com/tendermint/tendermint/rpc/client/http"

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

func (c *CosmosLocalnet) SetupIBC(ctx context.Context, keyName string) (context.Context, error) {
	var contracts chains.ContractKey
	time.Sleep(4 * time.Second)

	ibcCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["ibc"])
	if err != nil {
		return nil, err
	}

	ibcAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, ibcCodeId, "{}", true)
	if err != nil {
		return nil, err
	}

	clientCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["client"])
	if err != nil {
		return ctx, err
	}

	// Parameters here will be empty in the future
	clientAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, clientCodeId, `{}`, true)
	if err != nil {
		return nil, err
	}

	contracts.ContractAddress = map[string]string{
		"ibc":    ibcAddress,
		"client": clientAddress,
	}
	fmt.Println(contracts.ContractAddress)

	err = c.CosmosChain.ExecuteContract(context.Background(), keyName, ibcAddress, `{"register_client":{"client_type":"iconclient", "client_address":"`+clientAddress+`"}}`)
	if err != nil {
		return nil, err
	}

	c.IBCAddresses = contracts.ContractAddress
	overrides := map[string]any{
		"ibc-handler-address": ibcAddress,
	}

	cfg := c.cfg
	cfg.ConfigFileOverrides = overrides
	c.cfg = cfg

	return context.WithValue(ctx, chains.Mykey("Contract Names"), chains.ContractKey{
		ContractAddress: contracts.ContractAddress,
		ContractOwner:   contracts.ContractOwner,
	}), err
}

func (c *CosmosLocalnet) SetupXCall(ctx context.Context, portId string, keyName string) error {
	ibcAddress := c.IBCAddresses["ibc"]
	denom := c.Config().Denom
	xCallCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["xcall"])
	if err != nil {
		return err
	}

	xCallAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, xCallCodeId, `{"network_id": "`+c.Config().ChainID+`", "denom":"`+denom+`"}`, true)
	if err != nil {
		return err
	}

	connectionCodeId, err := c.CosmosChain.StoreContract(ctx, keyName, c.filepath["connection"])
	if err != nil {
		return err
	}

	connectionAddress, err := c.CosmosChain.InstantiateContract(ctx, keyName, connectionCodeId, `{"port_id":"`+portId+`","xcall_address":"`+xCallAddress+`", "denom":"`+denom+`", "ibc_host":"`+ibcAddress+`"}`, true)
	if err != nil {
		return err
	}

	err = c.CosmosChain.ExecuteContract(context.Background(), keyName, ibcAddress, `{"bind_port":{"port_id":"`+portId+`", "address":"`+connectionAddress+`"}}`)
	if err != nil {
		return err
	}

	c.IBCAddresses["xcall"] = xCallAddress
	c.IBCAddresses["connection"] = connectionAddress
	return nil
}

func (c *CosmosLocalnet) ConfigureBaseConnection(ctx context.Context, connection chains.XCallConnection) (context.Context, error) {
	temp := "iconclient-0"
	params := `{"connection_id":"` + connection.ConnectionId + `","counterparty_port_id":"` + connection.CounterPartyPortId + `","counterparty_nid":"` + connection.CounterpartyNid + `","client_id":"` + temp + `","timeout_height":100}`
	_, err := c.ExecuteContract(context.Background(), c.IBCAddresses["connection"], connection.KeyName, "configure_connection", params)

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
	xcall := c.IBCAddresses["xcall"]
	dappCodeId, err := c.CosmosChain.StoreContract(ctx, connection.KeyName, c.filepath["dapp"])
	if err != nil {
		return err
	}

	dappAddress, err := c.CosmosChain.InstantiateContract(ctx, connection.KeyName, dappCodeId, `{"address":"`+xcall+`"}`, true)
	if err != nil {
		return err
	}

	_, err = c.ExecuteContract(context.Background(), dappAddress, connection.KeyName, "add_connection", `{"src_endpoint":"`+c.IBCAddresses["connection"]+`", "dest_endpoint":"`+connection.CounterPartyConnection+`","network_id":"`+connection.CounterpartyNid+`"}`)
	if err != nil {
		return err
	}
	c.IBCAddresses["dapp"] = dappAddress

	return nil
}

func (c *CosmosLocalnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data, rollback []byte) (string, string, error) {
	dataArray := strings.Join(strings.Fields(fmt.Sprintf("%d", data)), ",")
	rollbackArray := strings.Join(strings.Fields(fmt.Sprintf("%d", rollback)), ",")
	params := fmt.Sprintf(`{"to":"%s", "data":%s, "rollback":%s}`, _to, dataArray, rollbackArray)
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	ctx, err := c.ExecuteContract(context.Background(), c.IBCAddresses["dapp"], chains.FaucetAccountKeyName, "send_call_message", params)
	if err != nil {
		return "", "", err
	}

	tx := ctx.Value("txResult").(*TxResul)
	sn := c.findSn(tx)
	reqId, err := targetChain.FindCallMessage(ctx, int64(height), c.cfg.ChainID+"/"+c.IBCAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, err
}

func (c *CosmosLocalnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, error) {
	dataArray := strings.Join(strings.Fields(fmt.Sprintf("%d", data)), ",")
	params := fmt.Sprintf(`{"to":"%s", "data":%s}`, _to, dataArray)
	height, _ := targetChain.(ibc.Chain).Height(ctx)
	ctx, err := c.ExecuteContract(context.Background(), c.IBCAddresses["xcall"], keyName, "send_call_message", params)
	if err != nil {
		return "", "", err
	}

	tx := ctx.Value("txResult").(*TxResul)
	sn := c.findSn(tx)
	reqId, err := targetChain.FindCallMessage(ctx, int64(height), c.cfg.ChainID+"/"+c.IBCAddresses["dapp"], strings.Split(_to, "/")[1], sn)
	return sn, reqId, err
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

func (c *CosmosLocalnet) ExecuteCall(ctx context.Context, reqId string) (context.Context, error) {
	return c.ExecuteContract(context.Background(), c.IBCAddresses["xcall"], chains.FaucetAccountKeyName, "execute_call", `{"request_id":"`+reqId+`"}`)
}

func (c *CosmosLocalnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	return c.ExecuteContract(context.Background(), c.IBCAddresses["xcall"], chains.FaucetAccountKeyName, "execute_rollback", `{"sequence_no":"`+sn+`"}`)
}

func (c *CosmosLocalnet) FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, error) {
	endpoint := c.GetHostRPCAddress()
	client, err := rpchttp.New(endpoint, "/websocket")
	if err != nil {
		log.Fatal(err)
	}

	err = client.Start()
	if err != nil {
		log.Fatal(err)
	}
	defer client.Stop()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()
	query := strings.Join([]string{"tm.event = 'Tx'",
		fmt.Sprintf("tx.height > %d ", startHeight),
		fmt.Sprintf("message.module = 'wasm'"),
		fmt.Sprintf("wasm._contract_address = '%s'", c.IBCAddresses["xcall"]),
		fmt.Sprintf("wasm-CallMessage.from CONTAINS '%s'", from),
		fmt.Sprintf("wasm-CallMessage.to CONTAINS '%s'", to),
		fmt.Sprintf("wasm-CallMessage.sn CONTAINS '%s'", sn),
	}, " AND ")
	txs, err := client.Subscribe(ctx, "wasm-client", query)
	if err != nil {
		log.Fatal(err)
	}

	for e := range txs {
		return e.Events["wasm-CallMessage.reqId"][0], nil
	}

	return "", fmt.Errorf("No message found")
}

func (c *CosmosLocalnet) DeployContract(ctx context.Context, keyName string) (context.Context, error) {
	// Fund user to deploy contract
	contractOwner, ownerAddr, _ := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)

	// Get Contract Name from context
	ctxValue := ctx.Value(chains.ContractName{}).(chains.ContractName)
	contractName := strings.ToLower(ctxValue.ContractName)
	codeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath[contractName])
	if err != nil {
		return ctx, err
	}

	// Get Init Message from context
	ctxVal := ctx.Value(chains.InitMessage{}).(chains.InitMessage)
	initMessage := ctxVal.InitMsg
	if initMessage == "runtime" {
		initMessage = c.getInitParams(ctx, contractName)
	}
	address, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, codeId, initMessage, true)
	if err != nil {
		return nil, err
	}

	c.IBCAddresses[contractName] = address
	contracts.ContractAddress[contractName] = address
	contracts.ContractOwner[keyName] = ownerAddr

	return context.WithValue(ctx, chains.Mykey("Contract Names"), contracts), err
}

func (c *CosmosLocalnet) QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error) {
	// wait for few blocks after executing before querying
	time.Sleep(2 * time.Second)

	// get query msg
	queryMsg := c.GetQueryParam(methodName)
	chains.Response = ""
	err := c.CosmosChain.QueryContract(ctx, contractAddress, queryMsg, &chains.Response)
	fmt.Printf("Response is : %s \n", chains.Response)
	return ctx, err
}

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error) {
	txHash, err := c.getFullNode().ExecTx(ctx, keyName,
		"wasm", "execute", contractAddress, `{"`+methodName+`":`+param+`}`)
	tx, err := c.getTransaction(txHash)
	ctx = context.WithValue(ctx, "txResult", tx)
	return ctx, err
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
