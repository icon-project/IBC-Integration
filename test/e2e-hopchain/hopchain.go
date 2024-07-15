package e2e_hopchain

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"math/big"
	"strings"
	"testing"
	"time"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/testsuite"
	"github.com/strangelove-ventures/interchaintest/v7/ibc"
	"github.com/strangelove-ventures/interchaintest/v7/testreporter"
	"github.com/stretchr/testify/assert"
	"gopkg.in/yaml.v3"
)

type HopchainTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

const (
	IconChainName     = "icon"
	CentauriChainName = "centauri"
	ArchwayChainName  = "archway"
	iconCurrency      = "icx"
	archwayCurrency   = "stake"
)

func getLatestChannels(ctx context.Context, eRep *testreporter.RelayerExecReporter, chainID string, relayer ibc.Relayer, connectionId string) (string, string, error) {
	channels, err := relayer.GetChannels(ctx, eRep, chainID)
	if err != nil {
		return "", "", err
	}
	for _, channel := range channels {
		for _, hop := range channel.ConnectionHops {
			if strings.Contains(hop, connectionId) {
				return channel.ChannelID, channel.Counterparty.ChannelID, nil
			}
		}
	}
	latestChannel := channels[len(channels)-1]
	return latestChannel.Counterparty.ChannelID, latestChannel.ChannelID, nil
}

func getIconChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == IconChainName {
			return chain
		}
	}
	return nil
}

func getCentauriChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == CentauriChainName {
			return chain
		}
	}
	return nil
}

func getArchwayChain(chains []chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == ArchwayChainName {
			return chain
		}
	}
	return nil
}

func getDenomHash(input string) string {
	hasher := sha256.New()
	hasher.Write([]byte(input))
	hash := hasher.Sum(nil)

	return strings.ToUpper(hex.EncodeToString(hash[:]))
}

type SrcDst struct {
	ChainID      string `yaml:"chain-id"`
	ClientID     string `yaml:"client-id"`
	ConnectionID string `yaml:"connection-id"`
}

type Path struct {
	Src SrcDst `yaml:"src"`
	Dst SrcDst `yaml:"dst"`
}

type Paths struct {
	Paths map[string]Path `yaml:"paths"`
}

func (h *HopchainTestSuite) TestICS20(relayer ibc.Relayer) {
	testcase := "ics20"
	time.Sleep(15 * time.Second)
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	createdChains := h.GetChains()
	eRep := h.GetRelayerExecReporter()
	result := relayer.Exec(ctx, eRep, []string{"cat", "/home/relayer/.relayer/config/config.yaml"}, []string{})
	var paths Paths
	yaml.Unmarshal(result.Stdout, &paths)
	portId := "transfer"

	iconChain := getIconChain(createdChains)
	centauriChain := getCentauriChain(createdChains)
	archwayChain := getArchwayChain(createdChains)

	_, centauriReceiver := centauriChain.GetSenderReceiverAddress()
	_, archwayReceiver := archwayChain.GetSenderReceiverAddress()
	iconSender, iconReceiver := iconChain.GetSenderReceiverAddress()

	centToIconChannel, iconToCentChannel, err := getLatestChannels(ctx, eRep, centauriChain.(ibc.Chain).Config().ChainID, relayer, paths.Paths[testsuite.CentauriIconRelayPath].Src.ConnectionID)
	h.Require().NoError(err, "error should be nil")
	centToArchChanel, archwayToCentChannel, err := getLatestChannels(ctx, eRep, centauriChain.(ibc.Chain).Config().ChainID, relayer, paths.Paths[testsuite.CentauriArchwayRelayPath].Src.ConnectionID)
	h.Require().NoError(err, "error should be nil")

	h.T.Run("send icon Ftokens for relay ibc-centauri", func(t *testing.T) {
		denom := "transfer/" + centToIconChannel + "/" + iconCurrency
		ibcDenom := "ibc/" + getDenomHash(denom)
		h.Require().NoError(err, "error should be nil")
		h.T.Run("send icx tokens from icon to centauri ", func(t *testing.T) {
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, iconCurrency)
			h.Require().NoError(err, "error retrieving balance")
			fmt.Println("Initial Icon Balance ", initialIconBalance)
			initialCentauriBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			fmt.Println("Initial Centauri Balance ", initialCentauriBalance)
			h.Require().NoError(err, "error retrieving balance")
			ibcAmount := fmt.Sprint(1000) + "/" + iconCurrency
			_, err = iconChain.SendIBCTokenTransfer(ctx, iconToCentChannel, centToArchChanel, portId, iconSender, centauriReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcAmount, false)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(40 * time.Second)
			newCentauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			fmt.Println("New Centauri Balance ", newCentauriChanBalance)
			h.Require().NoError(err, "error retrieving balance")
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("1000", 10)
			balanceIncreased := new(big.Int).Sub(newCentauriChanBalance, initialCentauriBalance)
			assert.True(t, balanceIncreased.Cmp(valueTocheck) == 0, "balance increment should be equal to 1000")
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, iconCurrency)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			// as gas needs to be paid from the same account
			assert.True(t, newIconBalance.Cmp(initialIconBalance) == 0, "icon balance should be same as balance transferred from main wallet")
		})

		h.T.Run("send ibc/../icx tokens from centauri to icon", func(t *testing.T) {
			// send back to icon
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, iconCurrency)
			h.Require().NoError(err, "error should be nil")
			initialCentauriBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Icon Balance", initialIconBalance)
			fmt.Println("Initial Centauri Balance", initialCentauriBalance)
			ibcamount := fmt.Sprint(500) + ibcDenom
			txnhash, err := centauriChain.SendIBCTokenTransfer(ctx, centToIconChannel, centToIconChannel, "transfer", centauriReceiver, iconReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcamount, false)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(40 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("500", 10)
			newCentauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			h.Require().NoError(err, "error retrieving balance")
			fmt.Println("New Centauri Balance", newCentauriChanBalance)
			centauriBalanceDecreased := new(big.Int).Sub(initialCentauriBalance, newCentauriChanBalance)
			assert.True(t, centauriBalanceDecreased.Cmp(valueTocheck) == 0, "balance should be equal to 500")
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, iconCurrency)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			increasedIconBalance := new(big.Int).Sub(newIconBalance, initialIconBalance)
			assert.True(t, increasedIconBalance.Cmp(valueTocheck) == 0, "icon balance should be increased")
		})
	})

	h.T.Run("send stake Ftokens for relay centauri-ibc", func(t *testing.T) {
		denom := "transfer/" + iconToCentChannel + "/stake"
		h.Require().NoError(err, "error should be nil")
		// register cosmos token
		iconChain.RegisterToken(ctx, denom, archwayCurrency, "2")
		h.T.Run("send stake tokens from centauri to ibc", func(t *testing.T) {
			// send back to icon
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Icon Balance", initialIconBalance)
			initialCentauriChainBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, archwayCurrency)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Centauri Balance", initialCentauriChainBalance)
			ibcamount := fmt.Sprint(1000) + archwayCurrency
			txnhash, err := centauriChain.SendIBCTokenTransfer(ctx, centToIconChannel, iconToCentChannel, "transfer", archwayReceiver, iconSender, centauriChain.(ibc.Chain).Config().ChainID, ibcamount, false)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(40 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("1000", 10)
			newCentauriChainBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, archwayCurrency)
			h.Require().NoError(err, "error retrieving balance")
			centauriBalanceDecreased := new(big.Int).Sub(initialCentauriChainBalance, newCentauriChainBalance)
			assert.True(t, centauriBalanceDecreased.Cmp(valueTocheck) > 0, "decreased balance should be more than 1000 plus fee")
			fmt.Println("Decreased centauri balance", centauriBalanceDecreased)
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			iconBalanceIncreased := new(big.Int).Sub(newIconBalance, initialIconBalance)
			fmt.Println("Increased icon balance", iconBalanceIncreased)
			assert.True(t, iconBalanceIncreased.Cmp(valueTocheck) == 0, "balance should be equal to 1000")
		})

		h.T.Run("send ibc/../stake tokens from icon to centauri ", func(t *testing.T) {
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			initialCentauriBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, archwayCurrency)
			fmt.Println("Centauri Initial Balance ", initialCentauriBalance)
			h.Require().NoError(err, "error retrieving balance")
			ibcAmount := fmt.Sprint(500) + denom
			// need to call token contract tokenFallback method
			_, err = iconChain.SendIBCTokenTransfer(ctx, iconToCentChannel, centToArchChanel, portId, iconSender, centauriReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcAmount, false)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(40 * time.Second)
			newCentauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, archwayCurrency)
			fmt.Println("New centauri Balance ", newCentauriChanBalance)
			h.Require().NoError(err, "error retrieving balance")
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("500", 10)
			balanceIncreased := new(big.Int).Sub(newCentauriChanBalance, initialCentauriBalance)
			assert.True(t, balanceIncreased.Cmp(valueTocheck) == 0, "balance should be equal")
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			decreasedIconBalance := new(big.Int).Sub(initialIconBalance, newIconBalance)
			fmt.Println("Increased icon balance", decreasedIconBalance)
			assert.True(t, decreasedIconBalance.Cmp(valueTocheck) == 0, "decreased balance should be equal to 500")
		})
	})

	h.T.Run("send icon Ftokens for relay ibc-archway via centauri", func(t *testing.T) {
		denom := "transfer/" + archwayToCentChannel + "/transfer/" + centToIconChannel + "/icx"
		ibcDenom := "ibc/" + getDenomHash(denom)
		h.Require().NoError(err, "error should be nil")
		h.T.Run("send icx tokens from icon to centauri ", func(t *testing.T) {
			initialArchwayBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, ibcDenom)
			fmt.Println("Initial archway Balance ", initialArchwayBalance)
			h.Require().NoError(err, "error retrieving balance")
			ibcAmount := fmt.Sprint(1000) + "/icx"
			_, err = iconChain.SendIBCTokenTransfer(ctx, iconToCentChannel, centToArchChanel, portId, iconSender, archwayReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcAmount, true)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(40 * time.Second)
			archwayChanBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, ibcDenom)
			fmt.Println("New archway Balance ", archwayChanBalance)
			h.Require().NoError(err, "error retrieving balance")
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("1000", 10)
			balanceIncreased := new(big.Int).Sub(archwayChanBalance, initialArchwayBalance)
			assert.True(t, balanceIncreased.Cmp(valueTocheck) == 0, "balance should be equal to 1000")
		})

		h.T.Run("send ibc/../icx tokens from archway to ibc via centauri", func(t *testing.T) {
			// send back to icon
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, "icx")
			h.Require().NoError(err, "error should be nil")
			initialArchwayChainBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, ibcDenom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Icon Balance", initialIconBalance)
			fmt.Println("Initial archway Balance", initialArchwayChainBalance)
			ibcamount := fmt.Sprint(500) + ibcDenom
			txnhash, err := archwayChain.SendIBCTokenTransfer(ctx, archwayToCentChannel, centToIconChannel, "transfer", archwayReceiver, iconReceiver, archwayChain.(ibc.Chain).Config().ChainID, ibcamount, true)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(40 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("500", 10)
			newArchwayChainBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, ibcDenom)
			h.Require().NoError(err, "error retrieving balance")
			archwayBalanceDecreased := new(big.Int).Sub(initialArchwayChainBalance, newArchwayChainBalance)
			assert.True(t, archwayBalanceDecreased.Cmp(valueTocheck) == 0, "balance should be equal to 500")
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconReceiver, "icx")
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			increasedIconBalance := new(big.Int).Sub(newIconBalance, initialIconBalance)
			assert.True(t, increasedIconBalance.Cmp(valueTocheck) == 0, "icon balance should be increased by 500")
		})
	})

	h.T.Run("send stake Ftokens for relay archway-ibc via centauri", func(t *testing.T) {
		// denom := "transfer/channel-0/transfer/channel-3/stake"
		denom := "transfer/" + iconToCentChannel + "/transfer/" + centToArchChanel + "/stake"
		// ibcDenom := "ibc/" + getDenomHash(denom)
		h.Require().NoError(err, "error should be nil")
		// register cosmos token
		iconChain.RegisterToken(ctx, denom, archwayCurrency, "2")
		h.T.Run("send stake tokens from archway to ibc via centauri", func(t *testing.T) {
			// send back to icon
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Icon Balance", initialIconBalance)
			initialArchwayChainBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, archwayCurrency)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Initial Archway Balance", initialArchwayChainBalance)
			ibcamount := fmt.Sprint(1000) + archwayCurrency
			txnhash, err := archwayChain.SendIBCTokenTransfer(ctx, archwayToCentChannel, centToIconChannel, "transfer", archwayReceiver, iconSender, archwayChain.(ibc.Chain).Config().ChainID, ibcamount, true)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(40 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("1000", 10)
			newArchwayChainBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, archwayCurrency)
			h.Require().NoError(err, "error retrieving balance")
			archwayBalanceDecreased := new(big.Int).Sub(initialArchwayChainBalance, newArchwayChainBalance)
			assert.True(t, archwayBalanceDecreased.Cmp(valueTocheck) > 0, "decreased balance should be more than 1000 plus fee")
			fmt.Println("Decreased archway balance", archwayBalanceDecreased)
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			iconBalanceIncreased := new(big.Int).Sub(newIconBalance, initialIconBalance)
			fmt.Println("Increased icon balance", iconBalanceIncreased)
			assert.True(t, iconBalanceIncreased.Cmp(valueTocheck) == 0, "balance should be equal to 1000")
		})

		h.T.Run("send ibc/../stake tokens from icon to archway ", func(t *testing.T) {
			initialIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			initialArchwayBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, archwayCurrency)
			fmt.Println("Archway Initial Balance ", initialArchwayBalance)
			h.Require().NoError(err, "error retrieving balance")
			ibcAmount := fmt.Sprint(500) + denom
			// need to call token contract tokenFallback method
			_, err = iconChain.SendIBCTokenTransfer(ctx, iconToCentChannel, centToArchChanel, portId, iconSender, archwayReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcAmount, true)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(40 * time.Second)
			newArchwayChanBalance, err := archwayChain.GetWalletBalance(ctx, archwayReceiver, archwayCurrency)
			fmt.Println("New archway Balance ", newArchwayChanBalance)
			h.Require().NoError(err, "error retrieving balance")
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("500", 10)
			balanceIncreased := new(big.Int).Sub(newArchwayChanBalance, initialArchwayBalance)
			assert.True(t, balanceIncreased.Cmp(valueTocheck) == 0, "balance should be equal")
			newIconBalance, err := iconChain.GetWalletBalance(ctx, iconSender, denom)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("New Icon Balance", newIconBalance)
			decreasedIconBalance := new(big.Int).Sub(initialIconBalance, newIconBalance)
			fmt.Println("Increased icon balance", decreasedIconBalance)
			assert.True(t, decreasedIconBalance.Cmp(valueTocheck) == 0, "decreased balance should be equal to 500")
		})
	})

}
