package chains

const (
	FaucetAccountKeyName = "faucet"
	BASE_PATH            = "BASE_PATH"
)

type ContractKey struct {
	ContractAddress map[string]string
	ContractOwner   map[string]string
}

type XCallConnection struct {
	KeyName                string
	ConnectionId           string
	ClientId               string
	CounterpartyNid        string
	PortId                 string
	CounterPartyPortId     string
	CounterPartyConnection string
	TimeoutHeight          string
}

type XCallResponse struct {
	SerialNo  string
	RequestID string
	Data      string
}

type LastBlock struct{}

type ContractName struct {
	ContractName string
}

type InitMessage struct {
	InitMsg string
}

type Param struct {
	Data string
}

type Query struct {
	Method string
	Param  string
}

type Mykey string

type Admins struct {
	Admin map[string]string
}

type AdminKey string

var Response interface{}

type MinimumGasPriceEntity struct {
	Denom  string `json:"denom"`
	Amount string `json:"amount"`
}
