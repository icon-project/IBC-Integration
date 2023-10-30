package tendermint

func (m *ConsensusState) ValidateBasic() error { return nil }
func (m *ConsensusState) ClientType() string   { return "07-tendermint" }
func (m *ConsensusState) GetTimestamp() uint64 { return 0 }
