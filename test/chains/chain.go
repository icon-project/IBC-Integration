package chains

import (
	"bytes"
	"context"
	"fmt"
	"os"

	"github.com/cosmos/gogoproto/proto"
	conntypes "github.com/cosmos/ibc-go/v7/modules/core/03-connection/types"
	"github.com/icon-project/ibc-integration/test/internal/blockdb"
	"github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"
)

const (
	DefaultNumValidators = 1
	DefaultNumFullNodes  = 1
)

type Chain interface {
	DeployContract(ctx context.Context, keyName string) (context.Context, error)
	QueryContract(ctx context.Context, contractAddress, methodName, params string) (context.Context, error)
	ExecuteContract(ctx context.Context, contractAddress, keyName, methodName, param string) (context.Context, error)
	GetLastBlock(ctx context.Context) (context.Context, error)
	GetBlockByHeight(ctx context.Context) (context.Context, error)
	FindTxs(ctx context.Context, height uint64) ([]blockdb.Tx, error)
	BuildWallets(ctx context.Context, keyName string) error
	SetupIBC(ctx context.Context, keyName string) (context.Context, error)
	SetupXCall(ctx context.Context, portId, keyName string) error
	ConfigureBaseConnection(ctx context.Context, connection XCallConnection) (context.Context, error)
	XCall(ctx context.Context, targetChain Chain, keyName, _to string, data, rollback []byte) (string, string, string, error)
	EOAXCall(ctx context.Context, targetChain Chain, keyName, _to string, data []byte, sources, destinations []string) (string, string, string, error)
	ExecuteCall(ctx context.Context, reqId, data string) (context.Context, error)
	ExecuteRollback(ctx context.Context, sn string) (context.Context, error)
	FindCallMessage(ctx context.Context, startHeight int64, from, to, sn string) (string, string, error)
	FindCallResponse(ctx context.Context, startHeight int64, sn string) (string, error)
	OverrideConfig(key string, value any)
	GetIBCAddress(key string) string
	DeployXCallMockApp(ctx context.Context, connection XCallConnection) error
	PreGenesis() error
	GetClientState(context.Context, int) (context.Context, error)
	GetClientName(int) string
	GetClientsCount(context.Context) (int, error)
	GetConnectionState(context.Context, int) (*conntypes.ConnectionEnd, error)
	GetNextConnectionSequence(context.Context) (int, error)
}

func GetEnvOrDefault(key, defaultValue string) string {
	if value := os.Getenv(key); value != "" {
		return value
	}
	return defaultValue
}

func HexBytesToProtoUnmarshal(encoded types.HexBytes, v proto.Message) ([]byte, error) {
	inputBytes, err := encoded.Value()
	if err != nil {
		return nil, fmt.Errorf("error unmarshalling HexByte: %s", err)
	}

	if bytes.Equal(inputBytes, make([]byte, 0)) {
		return nil, fmt.Errorf("encoded hexbyte is empty: %s", inputBytes)
	}

	if err := proto.Unmarshal(inputBytes, v); err != nil {
		return nil, err

	}
	return inputBytes, nil

}
