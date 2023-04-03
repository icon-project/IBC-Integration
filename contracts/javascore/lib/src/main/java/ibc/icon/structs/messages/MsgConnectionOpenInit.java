package ibc.icon.structs.messages;

import java.math.BigInteger;

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

    public byte[] getCounterparty() {
        return counterparty;
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
