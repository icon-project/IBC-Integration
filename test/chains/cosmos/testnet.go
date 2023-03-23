package cosmos

import (
	"context"
	"encoding/json"
	"fmt"
	"os/exec"
	"strconv"

	"github.com/icon-project/ibc-integration/test/chains"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
)

func NewCosmosTestnet(bin, keystorePath, keyPassword, defaultStepLimit, url string, scorePaths map[string]string) chains.Chain {
	return &CosmosTestnet{
		bin:              bin,
		keystorePath:     keystorePath,
		keyPassword:      keyPassword,
		scorePaths:       scorePaths,
		defaultStepLimit: defaultStepLimit,
		url:              url,
		Client:           nil,
	}
}

func (c *CosmosTestnet) DeployContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosTestnet) QueryContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosTestnet) ExecuteContract(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosTestnet) GetLastBlock(ctx context.Context) (context.Context, error) {
	var result Result
	hash, err := exec.Command(c.bin, "status", "--node", c.url).Output()
	if err != nil {
		fmt.Println(err)
	}
	err = json.Unmarshal(hash, &result)
	if err != nil {
		fmt.Println(err)
	}
	height, err := strconv.ParseUint(result.SyncInfo.LatestBlockHeight, 10, 64)
	return context.WithValue(ctx, chains.LastBlock{}, uint64(height)), err
}

func (c *CosmosTestnet) GetBlockByHeight(ctx context.Context) (context.Context, error) {
	panic("not implemented") // TODO: Implement
}

func (c *CosmosTestnet) FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error) {
	panic("not implemented") // TODO: Implement
}

// Height returns the current block height or an error if unable to get current height.
func (c *CosmosTestnet) Height(ctx context.Context) (uint64, error) {
	var result Result
	hash, err := exec.Command(c.bin, "status", "--node", c.url, "| jq -r '.SyncInfo.latest_block_height'").Output()
	if err != nil {
		fmt.Println(err)
	}
	err = json.Unmarshal(hash, &result)
	if err != nil {
		fmt.Println(err)
	}
	return uint64(0), err
}
