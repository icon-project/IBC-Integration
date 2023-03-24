package integration_test

import "github.com/stretchr/objx"

type State = objx.Map

const CONTRACT_OWNERS = string("CONTRACT_OWNERS")
const WALLETS = string("WALLETS")

func NewState() objx.Map {
	return objx.Map{}
}
