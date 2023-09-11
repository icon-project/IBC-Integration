package chains

const (
	//read only method
	HasPacketReceipt          = "has-packet-receipt"
	GetNextSequenceReceive    = "get-next-sequence-receive"
	GetClientState            = "get-client-state"
	GetNextClientSequence     = "get-next-client-sequence"
	GetConnection             = "get-connection"
	GetNextConnectionSequence = "get-next-connection-sequence"
	GetChannel                = "get-channel"
	GetNextChannelSequence    = "get-next-channel-sequence"

	//execute methods
	BindPort = "bind_port"

	//mock dapp
	SendMessage = "send_message"
)
