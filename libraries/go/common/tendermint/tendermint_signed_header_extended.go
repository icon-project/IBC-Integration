package tendermint

func (m *SignedHeader) ValidateBasic() error { return nil }
func (m *SignedHeader) ClientType() string   { return "tendermint" }
