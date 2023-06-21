package cosmos

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/strangelove-ventures/interchaintest/v7/chain/cosmos"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"go.uber.org/zap"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
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
	contractOwner, _, err := c.GetAndFundTestUser(ctx, keyName, int64(100_000_000), c.CosmosChain)
	if err != nil {
		return nil, err
	}

	ibcCodeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath["ibc"])
	if err != nil {
		return nil, err
	}

	ibcAddress, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, ibcCodeId, "{}", true)
	if err != nil {
		return nil, err
	}

	clientCodeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath["client"])
	if err != nil {
		return ctx, err
	}

	// Parameters here will be empty in the future
	clientAddress, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, clientCodeId, `{"src_network_id": "0x3.icon", "network_id": 1, "network_type_id": "1"}`, true)
	if err != nil {
		return nil, err
	}

	xCallCodeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath["xcall"])
	if err != nil {
		return ctx, err
	}

	xCallAddress, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, xCallCodeId, `{"timeout_height": 100, "connection_host":"`+ibcAddress+`"}`, true)
	if err != nil {
		return nil, err
	}

	connectionCodeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath["connection"])
	if err != nil {
		return ctx, err
	}

	connectionAddress, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, connectionCodeId, `{"timeout_height": 100, "ibc_host":"`+ibcAddress+`", "protocol_fee":"0"}`, true)
	if err != nil {
		return nil, err
	}

	dappCodeId, err := c.CosmosChain.StoreContract(ctx, contractOwner, c.filepath["dapp"])
	if err != nil {
		return ctx, err
	}

	dappAddress, err := c.CosmosChain.InstantiateContract(ctx, contractOwner, dappCodeId, `{"address":"`+xCallAddress+`"}`, true)
	if err != nil {
		return nil, err
	}

	contracts.ContractAddress = map[string]string{
		"ibc":        ibcAddress,
		"client":     clientAddress,
		"xcall":      xCallAddress,
		"connection": connectionAddress,
		"dapp":       dappAddress,
	}
	fmt.Println(contracts.ContractAddress)

	err = c.CosmosChain.ExecuteContract(context.Background(), keyName, ibcAddress, `{"register_client":{"client_type":"iconclient", "client_address":"`+clientAddress+`"}}`)
	if err != nil {
		return nil, err
	}

	err = c.CosmosChain.ExecuteContract(context.Background(), keyName, ibcAddress, `{"bind_port":{"port_id":"mock", "address":"`+connectionAddress+`"}}`)
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

func (c *CosmosLocalnet) ConfigureBaseConnection(ctx context.Context, keyName, channel, counterpartyNid, counterpartyConnection string) (context.Context, error) {
	panic("unimplemented")
}

func (c *CosmosLocalnet) XCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data, rollback []byte) (string, string, error) {
	panic("unimplemented")
}

func (c *CosmosLocalnet) EOAXCall(ctx context.Context, targetChain chains.Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, error) {
	panic("unimplemented")
}

func (c *CosmosLocalnet) ExecuteCall(ctx context.Context, reqId string) (context.Context, error) {
	panic("unimplemented")
}

func (c *CosmosLocalnet) ExecuteRollback(ctx context.Context, sn string) (context.Context, error) {
	panic("unimplemented")
}

func (c *CosmosLocalnet) FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, error) {
	panic("unimplemented")
	// endpoint := c.GetHostRPCAddress()
	// contractAddress := c.ibcAddresses["xcall"]

	// client, err := rpc.NewClient(endpoint)
	// if err != nil {
	// 	log.Fatalf("Failed to create RPC client: %s", err.Error())
	// }

	// ctx := context.Background()

	// eventQuery := fmt.Sprintf("message.action=execute&message.contract_address=%s", contractAddress)
	// eventReceiver := make(chan types.ResultEvent, 10)

	// err = client.SubscribeWithHeight(ctx, startingBlockHeight, "tm.event = 'Tx' AND "+eventQuery, eventReceiver)
	// if err != nil {
	// 	log.Fatalf("Failed to subscribe to events: %s", err.Error())
	// }

	// for event := range eventReceiver {
	// 	fmt.Printf("Received event: %v\n", event)
	// 	// Handle the received event as needed
	// }

	// // Close the event receiver channel when you're done
	// close(eventReceiver)
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

func (c *CosmosLocalnet) ExecuteContract(ctx context.Context, contractAddress, keyName, methodaName, param string) (context.Context, error) {
	// get param for executing a method in the contract
	ctx, params, err := c.GetExecuteParam(ctx, methodaName, param)
	if err != nil {
		return ctx, err
	}
	err = c.CosmosChain.ExecuteContract(ctx, keyName, contractAddress, params)
	return ctx, err
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
