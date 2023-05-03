package chains

const FaucetAccountKeyName = "faucet"

type ContractKey struct {
	ContractAddress map[string]string
	ContractOwner   map[string]string
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
