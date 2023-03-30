package ibc.icon.structs.messages;

import java.math.BigInteger;

import icon.proto.core.connection.Counterparty;

public class MsgConnectionOpenInit {
    private String clientId;
    private byte[] counterparty;
    private BigInteger delayPeriod;

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public byte[] getCounterpartyRaw() {
        return counterparty;
    }

    public Counterparty getCounterparty() {
        return Counterparty.decode(counterparty);
    }

    public void setCounterparty(byte[] counterparty) {
        this.counterparty = counterparty;
    }

    public BigInteger getDelayPeriod() {
        return delayPeriod;
    }

    public void setDelayPeriod(BigInteger delayPeriod) {
        this.delayPeriod = delayPeriod;
    }

}
