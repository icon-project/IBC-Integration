package relayer

type iconConfig struct {
	Type  string `json:"type"`
	Value struct {
		Key               string `json:"key"`
		ChainID           string `json:"chain-id"`
		RPCAddr           string `json:"rpc-addr"`
		Timeout           string `json:"timeout"`
		Keystore          string `json:"keystore"`
		Password          string `json:"password"`
		IconNetworkID     int    `json:"icon-network-id"`
		BtpNetworkID      int    `json:"btp-network-id"`
		StartHeight       int    `json:"start-height"`
		IbcHandlerAddress string `json:"ibc-handler-address"`
	} `json:"value"`
}