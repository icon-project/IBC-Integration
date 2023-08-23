package icon

import icontypes "github.com/icon-project/icon-bridge/cmd/iconbridge/chain/icon/types"

type Query struct {
	MethodName string
	Value      Value
}

type Value struct {
	Params map[string]interface{} `json:"params,omitempty"`
}

type DebugTrace struct {
	Logs []struct {
		Level   uint   `json:"level"`
		Message string `json:"msg"`
		Ts      int    `json:"ts"`
	} `json:"logs"`
	Status icontypes.HexInt `json:"status"`
}
