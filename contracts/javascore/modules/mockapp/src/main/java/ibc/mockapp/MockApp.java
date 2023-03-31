package ibc.mockapp;

import java.math.BigInteger;
import java.util.Map;

import ibc.icon.interfaces.IIBCModule;
import icon.proto.core.client.Height;
import icon.proto.core.channel.Packet;
import score.Address;
import score.Context;
import score.VarDB;
import score.annotation.External;

public class MockApp implements IIBCModule {

    public final Address ibcHandler;

    public MockApp(Address ibcHandler) {
        this.ibcHandler  = ibcHandler;
    }

    public static final VarDB<String> srcChan = Context.newVarDB("srcChan", String.class);
    public static final VarDB<String> srcPort = Context.newVarDB("srcPort", String.class);
    public static final VarDB<String> dstChan = Context.newVarDB("dstChan", String.class);
    public static final VarDB<String> dstPort = Context.newVarDB("dstPort", String.class);
    public static final VarDB<BigInteger> sendCount = Context.newVarDB("sendPacket", BigInteger.class);
    public static final VarDB<BigInteger> recvCount = Context.newVarDB("recvPacket", BigInteger.class);

    @External(readonly = true)
    public String name() {
        return "IBC Mock App";
    }

    @External(readonly = true)
    public Map<String, String> getInfo() {
        return Map.of(
            "srcPort", srcPort.getOrDefault(""), 
            "srcChan", srcChan.getOrDefault(""), 
            "dstPort", dstPort.getOrDefault(""), 
            "dstChan", dstChan.getOrDefault("")
        );
    }

    @External 
    public void sendPacket(byte[] data) {
        BigInteger currCount = sendCount.getOrDefault(BigInteger.ZERO);
        sendCount.set(currCount.add(BigInteger.ONE));

        Packet pct = new Packet();
        pct.setSequence(sendCount());
        pct.setData(data);
        pct.setDestinationPort(dstPort.getOrDefault("xcall"));
        pct.setDestinationChannel(dstChan.getOrDefault("channel-0"));
        pct.setSourcePort(srcPort.getOrDefault("xcall"));
        pct.setSourceChannel(srcChan.getOrDefault("channel-0"));

        Height hgt = new Height();
        hgt.setRevisionHeight(BigInteger.ZERO);
        hgt.setRevisionNumber(BigInteger.ZERO);
        pct.setTimeoutHeight(hgt);

        pct.setTimeoutTimestamp(BigInteger.ZERO);
        Context.call(this.ibcHandler, "sendPacket", pct.encode());
    }

    @External(readonly = true)
    public BigInteger sendCount() {
        return sendCount.getOrDefault(BigInteger.ZERO);
    }

    @External(readonly = true)
    public BigInteger recvCount() {
        return recvCount.getOrDefault(BigInteger.ZERO);
    }

    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version) {
        srcChan.set(channelId);
        srcPort.set(portId);
        Context.println("onChanOpenInit");
    }

    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version, String counterpartyVersion) {
        dstChan.set(channelId);
        dstPort.set(portId);
        Context.println("onChanOpenTry");
    }

    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyVersion) {
        Context.require(portId.equals(srcPort.get()));
        Context.require(channelId.equals(srcChan.get()));
        Context.println("onChanOpenAck");
    }

    @External
    public void onChanOpenConfirm(String portId, String channelId) {
        Context.require(portId.equals(dstPort.get()));
        Context.require(channelId.equals(dstChan.get()));
        Context.println("onChanOpenConfirm");
    }

    @External
    public void onChanCloseInit(String portId, String channelId) {
        Context.require(Context.getOwner().equals(Context.getCaller()), "Owner guard");
        Context.require(portId.equals(srcPort.get()));
        Context.require(channelId.equals(srcChan.get()));
        Context.println("onChanCloseInit");
    }

    @External
    public void onChanCloseConfirm(String portId, String channelId) {
        Context.println("onChanCloseConfirm");
    }

    @External
    public byte[] onRecvPacket(byte[] calldata, Address relayer) {
        BigInteger currCount = recvCount.getOrDefault(BigInteger.ZERO);
        recvCount.set(currCount.add(BigInteger.ONE));
        Context.println("onRecvPacket");
        if (calldata.length > 0) {
            return new byte[1];
        } 
        return null;
    }

    @External
    public void onAcknowledgementPacket(byte[] calldata, byte[] acknowledgement, Address relayer) {
        Context.println("onAcknowledgementPacket");
    }

    @External
    public void onTimeoutPacket(byte[] calldata, Address relayer) {
        Context.println("onTimeoutPacket");
    }

}
