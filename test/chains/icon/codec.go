package icon

import (
	"github.com/cosmos/cosmos-sdk/codec"
	"github.com/cosmos/cosmos-sdk/codec/types"
	codectypes "github.com/cosmos/cosmos-sdk/codec/types"
	"github.com/cosmos/ibc-go/v7/modules/core/exported"
	"github.com/icon-project/ibc-integration/libraries/go/common/icon"
	"github.com/icon-project/ibc-integration/libraries/go/common/tendermint"
)

func RegisterInterfaces(registry codectypes.InterfaceRegistry) {
	registry.RegisterImplementations(
		(*exported.ClientState)(nil),
		&tendermint.ClientState{},
		&icon.ClientState{},
	)
	registry.RegisterImplementations(
		(*exported.ConsensusState)(nil),
		&tendermint.ConsensusState{},
		&icon.ConsensusState{},
	)
	registry.RegisterImplementations(
		(*exported.ClientMessage)(nil),
		&icon.SignedHeader{},
	)
}

func MakeCodec() *codec.ProtoCodec {
	interfaceRegistry := types.NewInterfaceRegistry()
	RegisterInterfaces(interfaceRegistry)
	return codec.NewProtoCodec(interfaceRegistry)
}
