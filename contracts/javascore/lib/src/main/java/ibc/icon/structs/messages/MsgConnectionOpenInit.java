package ibc.icon.structs.messages;

import java.math.BigInteger;

import ibc.icon.structs.proto.core.connection.Counterparty;

public class MsgConnectionOpenInit {
    public String clientId;
    public Counterparty counterparty;
    public BigInteger delayPeriod;
}
