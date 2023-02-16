# Relayer Docs 

A relayer is an off-chain entity that facilitates the transfer of data packets across different chains. The complete implementation of this concept for the Inter-Blockchain Communication (IBC) protocol can be found on this [link](https://github.com/cosmos/relayer). We will use fork of this repository to intergrate Icon, polygon and archway. To integrate a new chain, a developer guide is available on this [link](https://github.com/cosmos/relayer/blob/main/docs/chain_implementation.md), which outlines the creation of two essential components: chainProvider and chainProcessor.

The [chainProvider](https://github.com/cosmos/relayer/blob/main/relayer/provider/provider.go#L214) is responsible for retrieving relevant data, constructing IBC messages and managing cryptographic keys. On the other hand, the [chainProcessor](https://github.com/cosmos/relayer/blob/main/relayer/processor/chain_processor.go#L11) ensures that the relayer remains in sync with the target chain. These components can be customized according to the configuration and requirements of the target chain. We can take reference from these [links](https://github.com/cosmos/relayer/tree/main/relayer/chains/cosmos).

Additionally, it may be necessary to adjust other components, such as [clientMatcher](https://github.com/cosmos/relayer/blob/main/relayer/provider/matcher.go#L28),
[processor type](https://github.com/cosmos/relayer/blob/main/relayer/strategies.go#L99) and [config cases](https://github.com/cosmos/relayer/blob/main/cmd/config.go#L394), based on the target chain. We can find the configuration example on this [link](https://github.com/cosmos/relayer/blob/main/examples/config_EXAMPLE.yaml) where specific chain configs are provided as a type and value structure. The value map should match provider config structure as shown [here](https://github.com/cosmos/relayer/blob/main/relayer/chains/cosmos/provider.go#L33). 

## Polygon

The implementation of the chainProvider and chainProcessor interfaces must be completed. In addition, a proto file along with its accompanying Go binding structure for the light client should also be made available.

### Chain Config
```
type PolygonProviderConfig struct {

	Key               string  `json:"key" yaml:"key"`
	ChainName         string  `json:"-" yaml:"-"`
	ChainID           string  `json:"chain-id" yaml:"chain-id"`
	RPCAddr           string  `json:"rpc-addr" yaml:"rpc-addr"`
	AccountPrefix     string  `json:"account-prefix" yaml:"account-prefix"`
	KeyringBackend    string  `json:"keyring-backend" yaml:"keyring-backend"`
	GasAdjustment     float64 `json:"gas-adjustment" yaml:"gas-adjustment"`
	GasPrices         string  `json:"gas-prices" yaml:"gas-prices"`
	MinGasAmount      uint64  `json:"min-gas-amount" yaml:"min-gas-amount"`
	Debug             bool    `json:"debug" yaml:"debug"`
	Timeout           string  `json:"timeout" yaml:"timeout"`
	IbcHostAddress    string  `json:"ibc_host_address,omitempty"`
	IbcHandlerAddress string  `json:"ibc_handler_address,omitempty"`

}
```

### Event Parser
 The event parser is responsible for parsing IBC specific 
Events from every new block. 
* Client IBC messages (MsgCreateClient, MsgUpdateClient, MsgUpgradeClient, MsgSubmitMisbehaviour)
* Connection handshake message (MsgConnectionOpenInit, MsgConnectionOpenTry, MsgConnectionOpenAck, MsgConnectionOpenConfirm) 
* Channel handshake message (MsgChannelOpenInit, MsgChannelOpenTry, MsgChannelOpenAck, MsgChannelOpenConfirm, MsgChannelCloseInit, MsgChannelCloseConfirm)
* Packet-flow Message (MsgTransfer, MsgRecvPacket, MsgAcknowledgement)


