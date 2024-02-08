package e2e_hopchain

import (
	"context"
	"fmt"
	"math/big"
	"regexp"
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

type AddressBalance struct {
	Address  string
	Balances []Balance `mapstructure:"balance"`
}

type Balance struct {
	Denom  string
	Amount *big.Int
}

const IconChainName = "icon"
const CentauriChainName = "centauri"

func getBalance(balanceStdout string) AddressBalance {
	balanceSplit := strings.Split(balanceStdout, "\n")
	address := ""
	var balances []Balance
	for _, balanceInfo := range balanceSplit {
		if strings.HasPrefix(balanceInfo, "address") {
			// Define regular expressions to extract address, ICX balance, and stake balance
			addressRegex := regexp.MustCompile(`address\s*{([^}]*)}`)
			balanceRegex := regexp.MustCompile(`balance\s(.*)`)

			// Extract address
			addressMatches := addressRegex.FindStringSubmatch(balanceInfo)
			address = addressMatches[1]

			// Extract ICX balance and stake balance
			balanceMatches := balanceRegex.FindStringSubmatch(balanceInfo)
			re := regexp.MustCompile(`(\d+)([a-zA-Z\/\-\d+]+)`)

			// Find all matches in the input string
			matches := re.FindAllStringSubmatch(balanceMatches[0], -1)

			// Iterate over matches and populate the map
			for _, match := range matches {
				amount := new(big.Int)
				amount.SetString(match[1], 10)
				// currencyBalance[match[2]] = amount
				balances = append(balances, Balance{
					Denom:  match[2],
					Amount: amount,
				})
			}

		}
	}
	return AddressBalance{
		Address:  address,
		Balances: balances,
	}
}

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

func getChainAddressBalance(ctx context.Context, eRep *testreporter.RelayerExecReporter, relayer ibc.Relayer, chain chains.Chain) AddressBalance {
	balanceCommands := []string{"rly", "query", "balance", chain.(ibc.Chain).Config().ChainID}
	balanceResult := relayer.Exec(ctx, eRep, balanceCommands, nil)
	balanceInfo := getBalance(string(balanceResult.Stdout[:]))
	return balanceInfo
}

func getChainBalance(balances []Balance, denom string) Balance {
	for _, bal := range balances {
		if bal.Denom == denom {
			return bal
		}
	}
	return Balance{}
}

func (h *HopchainTestSuite) TestICS20(relayer ibc.Relayer) {
	testcase := "ics20"
	// portId := "transfer"
	ctx := context.WithValue(context.TODO(), "testcase", testcase)
	chainA, chainB := h.GetChains()
	eRep := h.GetRelayerExecReporter()
	portId := "transfer"
	h.T.Run("send IBC Ftokens for relay centauri-ibc", func(t *testing.T) {
		iconChain := getIconChain(chainA, chainB)
		centauriChain := getCentauriChain(chainA, chainB)
		iconAccountBalance := getChainAddressBalance(ctx, eRep, relayer, iconChain)
		cenauriAccountBalance := getChainAddressBalance(ctx, eRep, relayer, centauriChain)
		centauriReceiver := cenauriAccountBalance.Address
		iconReceiver := iconAccountBalance.Address
		relaySourceChannel, relayDestinationChannel, err := getLatestChannels(ctx, eRep, centauriChain.(ibc.Chain).Config().ChainID, relayer)
		denom := "transfer/" + relayDestinationChannel + "/icx"
		h.Require().NoError(err, "error should be nil")
		h.T.Run("send IBC Ftokens from ibc to centauri ", func(t *testing.T) {
			_, err := iconChain.SendIBCTokenTransfer(ctx, relaySourceChannel, relayDestinationChannel, portId, centauriReceiver, centauriChain.(ibc.Chain).Config().ChainID, 100)
			h.Require().NoError(err, "error should be nil")
			// give some time for txn to settle
			time.Sleep(10 * time.Second)
			iconAccountBalance = getChainAddressBalance(ctx, eRep, relayer, iconChain)
			cenauriAccountBalance = getChainAddressBalance(ctx, eRep, relayer, centauriChain)

			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("100000000000000000000", 10)

			centauriChanBalance := getChainBalance(cenauriAccountBalance.Balances, denom)
			assert.True(t, centauriChanBalance.Amount.Cmp(valueTocheck) == 0, "balance should be equal to 100000000000000000000")
		})

		h.T.Run("send IBC Ftokens from centauri to ibc ", func(t *testing.T) {
			// send back to icon
			txnhash, err := centauriChain.SendIBCTokenTransfer(ctx, relayDestinationChannel, relayDestinationChannel, "transfer", iconReceiver, centauriChain.(ibc.Chain).Config().ChainID, 50)
			h.Require().NoError(err, "error should be nil")
			fmt.Println("Txn hash is ", txnhash)
			time.Sleep(10 * time.Second)
			valueTocheck := new(big.Int)
			valueTocheck, _ = valueTocheck.SetString("50000000000000000000", 10)
			iconAccountBalance = getChainAddressBalance(ctx, eRep, relayer, iconChain)
			cenauriAccountBalance = getChainAddressBalance(ctx, eRep, relayer, centauriChain)
			centauriChanBalance := getChainBalance(cenauriAccountBalance.Balances, denom)
			assert.True(t, centauriChanBalance.Amount.Cmp(valueTocheck) == 0, "balance should be equal to 50000000000000000000")
		})
	})
}
