package tendermint

import "time"

func (m *ConsensusState) ValidateBasic() error { return nil }
func (m *ConsensusState) ClientType() string   { return "icAon" }

// this returns the timestamp in nanoseconds
func (m *ConsensusState) GetTimestamp() uint64 {
	secondsToNano := time.Second * time.Duration(m.Timestamp.Seconds) // convert seconds to nanoseconds
	return uint64(secondsToNano + time.Duration(m.Timestamp.Nanos))
}
