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
)

type HopchainTestSuite struct {
	*testsuite.E2ETestSuite
	T *testing.T
}

const IconChainName = "icon"
const CentauriChainName = "centauri"

func getLatestChannels(ctx context.Context, eRep *testreporter.RelayerExecReporter, chainID string, relayer ibc.Relayer) (string, string, error) {
	channels, err := relayer.GetChannels(ctx, eRep, chainID)
	if err != nil {
		return "", "", err
	}
	latestChannel := channels[len(channels)-1]
	return latestChannel.Counterparty.ChannelID, latestChannel.ChannelID, nil
}

func getIconChain(chains ...chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == IconChainName {
			return chain
		}
	}
	return nil
}

func getCentauriChain(chains ...chains.Chain) chains.Chain {
	for _, chain := range chains {
		if chain.(ibc.Chain).Config().Name == CentauriChainName {
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

func (h *HopchainTestSuite) TestICS20(relayer ibc.Relayer) {
	testcase := "ics20"
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	chainA, chainB := h.GetChains()
	eRep := h.GetRelayerExecReporter()
	portId := "transfer"
	centauriReceiver := "centauri1g5r2vmnp6lta9cpst4lzc4syy3kcj2ljte3tlh"
	iconReceiver := "hxb6b5791be0b5ef67063b3c10b840fb81514db2fd"
	iconChain := getIconChain(chainA, chainB)
	centauriChain := getCentauriChain(chainA, chainB)
	relaySourceChannel, relayDestinationChannel, err := getLatestChannels(ctx, eRep, centauriChain.(ibc.Chain).Config().ChainID, relayer)
	h.Require().NoError(err, "error should be nil")

	h.T.Run("send stake Ftokens for relay centauri-ibc", func(t *testing.T) {
		ibcIconDenom := "transfer/" + relaySourceChannel + "/stake"
		h.T.Run("send stake tokens from centauri to ibc ", func(t *testing.T) {
			originalBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, "stake")
			h.Require().NoError(err, "error should be nil")
			// send stake to icon
			ibcamount := fmt.Sprint(100) + "000000000000000stake"
			txnhash, err := centauriChain.SendIBCTokenTransfer(ctx, relayDestinationChannel, relayDestinationChannel, "transfer", iconReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcamount)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(20 * time.Second)
			valueTocheck := originalBalance.Sub(originalBalance, big.NewInt(100000000000000000))
			balanceCentauri, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, "stake")
			h.Require().NoError(err, "error retrieving balance")
			balanceIcon, err := iconChain.GetWalletBalance(ctx, iconReceiver, ibcIconDenom)
			h.Require().NoError(err, "error retrieving balance")
			assert.True(t, balanceCentauri.Cmp(valueTocheck) < 0, "balance should be less than calculated ")
			assert.True(t, balanceIcon.Cmp(big.NewInt(100000000000000000)) == 0, "balance should be equal in icon ")
		})

		h.T.Run("send transfer/stake tokens from icon to centauri ", func(t *testing.T) {
			ibcamount := fmt.Sprint(50) + "000000000000000" + "/transfer/" + relaySourceChannel + "/stake"
			prevCentauriChainBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, "stake")
			h.Require().NoError(err, "error should be nil")
			_, err = iconChain.SendIBCTokenTransfer(ctx, relaySourceChannel, relayDestinationChannel, portId, centauriReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcamount)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(20 * time.Second)

			balanceIcon, err := iconChain.GetWalletBalance(ctx, iconReceiver, ibcIconDenom)
			h.Require().NoError(err, "error retrieving balance")
			centauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, "stake")
			h.Require().NoError(err, "error retrieving balance")
			assert.True(t, balanceIcon.Cmp(big.NewInt(50000000000000000)) == 0, "balance should be equal in icon to 50000000000000000")
			assert.True(t, centauriChanBalance.Cmp(prevCentauriChainBalance) > 0, "balance should be greater than earlier")
		})

	})

	time.Sleep(time.Second * 20)

	h.T.Run("send IBC Ftokens for relay ibc-centauri", func(t *testing.T) {
		denom := "transfer/" + relayDestinationChannel + "/icx"
		ibcDenom := "ibc/" + getDenomHash(denom)
		h.Require().NoError(err, "error should be nil")
		h.T.Run("send icx tokens from icon to centauri ", func(t *testing.T) {
			ibcAmount := fmt.Sprint(100) + "000000000000000000" + "/icx"
			_, err := iconChain.SendIBCTokenTransfer(ctx, relaySourceChannel, relayDestinationChannel, portId, centauriReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcAmount)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(20 * time.Second)
			centauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			h.Require().NoError(err, "error retrieving balance")
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("100000000000000000000", 10)
			assert.True(t, centauriChanBalance.Cmp(valueTocheck) == 0, "balance should be equal to 100000000000000000000")
		})

		h.T.Run("send IBC icx tokens from centauri to ibc ", func(t *testing.T) {
			// send back to icon
			ibcamount := fmt.Sprint(50) + "000000000000000000" + "transfer/" + relayDestinationChannel + "/icx"
			txnhash, err := centauriChain.SendIBCTokenTransfer(ctx, relayDestinationChannel, relayDestinationChannel, "transfer", iconReceiver, centauriChain.(ibc.Chain).Config().ChainID, ibcamount)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(20 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("50000000000000000000", 10)
			centauriChanBalance, err := centauriChain.GetWalletBalance(ctx, centauriReceiver, ibcDenom)
			h.Require().NoError(err, "error retrieving balance")
			assert.True(t, centauriChanBalance.Cmp(valueTocheck) == 0, "balance should be equal to 50000000000000000000")
		})
	})

}
