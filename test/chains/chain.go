package chains

import "context"

type Chain interface {
	DeployContract(ctx context.Context, cfg interface{})
	QueryContract(ctx context.Context, cfg interface{})
	ExecuteContract(ctx context.Context, cfg interface{})
	GetBalance(ctx context.Context, cfg interface{})
	GetLastBlock(ctx context.Context, cfg interface{})
	GetBlockByHeight(ctx context.Context, cfg interface{})
}
