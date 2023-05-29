package e2e

import (
	"context"
	"fmt"
	"testing"
	"time"

	interchaintest "github.com/icon-project/ibc-integration/test"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/chains/icon"
	"github.com/icon-project/ibc-integration/test/relayer"
	"github.com/stretchr/testify/require"

	"go.uber.org/zap/zaptest"
)

func TestICONToICON(t *testing.T) {
	fmt.Println("test start")
	cfg, err := GetConfig()
	if err != nil {
		return
	}
	ctx := context.Background()
	logger := zaptest.NewLogger(t)
	iconChain1 := icon.NewIconLocalnet(t.Name(), logger, cfg.Icon.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.Icon.KeystoreFile, cfg.Icon.KeystorePassword, cfg.Icon.Contracts)
	iconChain2 := icon.NewIconLocalnet(t.Name(), logger, cfg.Counterparty.ChainConfig.GetIBCChainConfig(), chains.DefaultNumValidators, chains.DefaultNumFullNodes, cfg.Counterparty.KeystoreFile, cfg.Counterparty.KeystorePassword, cfg.Counterparty.Contracts)
	client, network := interchaintest.DockerSetup(t)

	// Log location
	f, err := interchaintest.CreateLogFile(fmt.Sprintf("%d.json", time.Now().Unix()))
	if err != nil {
		return
	}
	optionDocker := relayer.CustomDockerImage(cfg.Counterparty.ChainConfig.Images.Repository, cfg.Counterparty.ChainConfig.Images.Version, "100:1000")

	r := interchaintest.NewICONRelayerFactory(zaptest.NewLogger(t), optionDocker, relayer.ImagePull(false)).Build(
		t, client, network)
	// Reporter/logs
	rep := testreporter.NewReporter(f)
	eRep := rep.RelayerExecReporter(t)

	// Build interchain
	const ibcPath = "icon-cosmoshub"
	ic := interchaintest.NewInterchain().
		AddChain(iconChain1.(ibc.Chain)).
		AddChain(iconChain2.(ibc.Chain)).
		AddRelayer(r, "relayer")

	require.NoError(t, ic.BuildChains(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:          t.Name(),
		Client:            client,
		NetworkID:         network,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	))

	ctx, err = iconChain1.SetupIBC(ctx, "gochain")
	contracts1 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	fmt.Println(err)
	ctx, err = iconChain2.SetupIBC(ctx, "gochain")
	contracts2 := ctx.Value(chains.Mykey("Contract Names")).(chains.ContractKey)
	fmt.Println(err)

	opts := ibc.CreateChannelOptions{
		SourcePortName: "mock",
		DestPortName:   "mock",
		Order:          ibc.Unordered,
		Version:        "ics20-1",
	}

	ic.AddLink(interchaintest.InterchainLink{
		Chain1:            iconChain1.(ibc.Chain),
		Chain2:            iconChain2.(ibc.Chain),
		Relayer:           r,
		Path:              ibcPath,
		CreateChannelOpts: opts,
	})

	// Start the Relay
	require.NoError(t, ic.BuildRelayer(ctx, eRep, interchaintest.InterchainBuildOptions{
		TestName:          t.Name(),
		Client:            client,
		NetworkID:         network,
		BlockDatabaseFile: interchaintest.DefaultBlockDatabaseFilepath(),

		SkipPathCreation: false},
	))

	r.StartRelayer(ctx, eRep, ibcPath)
	nid1 := cfg.Icon.ChainConfig.ChainID
	nid2 := cfg.Counterparty.ChainConfig.ChainID
	fmt.Println(contracts1.ContractAddress)
	fmt.Println(contracts2.ContractAddress)
	iconChain1.ExecuteContract(context.Background(), contracts1.ContractAddress["connection"], "gochain", "configureChannel", `{"channelId":"channel-0", "counterpartyNid":"`+nid2+`"}`)
	iconChain2.ExecuteContract(context.Background(), contracts2.ContractAddress["connection"], "gochain", "configureChannel", `{"channelId":"channel-0", "counterpartyNid":"`+nid1+`"}`)

	_, err = iconChain1.ExecuteContract(context.Background(), contracts1.ContractAddress["dapp"], "gochain", "addConnection", `{"nid":"`+nid2+`", "source":"`+contracts1.ContractAddress["connection"]+`", "destination":"`+contracts2.ContractAddress["connection"]+`"}`)
	fmt.Println(err)
	_, err = iconChain2.ExecuteContract(context.Background(), contracts2.ContractAddress["dapp"], "gochain", "addConnection", `{"nid":"`+nid1+`", "source":"`+contracts2.ContractAddress["connection"]+`", "destination":"`+contracts1.ContractAddress["connection"]+`"}`)
	fmt.Println(err)

	msg := "Hello"
	dst := nid2 + "/" + contracts2.ContractAddress["dapp"]
	reqId, err := iconChain1.XCall(context.Background(), iconChain2, "gochain", dst, []byte(msg), nil)
	fmt.Println(reqId)
	fmt.Println(err)
	ctx, err = iconChain2.ExecuteCall(ctx, reqId)
	fmt.Println(ctx.Value("txResult"))
	fmt.Println(err)
}