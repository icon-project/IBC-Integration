package ibc.ics25.handler;

import java.math.BigInteger;

import ibc.icon.structs.Height;
import ibc.icon.structs.Packet;
import score.Context;
import score.ByteArrayObjectWriter;
import score.annotation.EventLog;
import score.annotation.External;

public class IBCHandler {

    public IBCHandler() {}

    @External(readonly = true)
    public String name() {
        return "Handler";
    }

    @EventLog(indexed = 1)
    public void SendPacket(byte[] packetData) {}

    @External()
    public void sendEvent() {
        Packet p = new Packet();
        p.sourcePort = "xcall";
        p.sourceChannel = "channel-0";
        p.destinationChannel = "channel-1";
        p.destinationPort = "xcall";
        p.sequence = BigInteger.ONE;
        p.timestamp = BigInteger.valueOf(Context.getBlockTimestamp()).divide(BigInteger.valueOf(1000000L));
        
        Height h = new Height();
        h.revisionHeight = BigInteger.valueOf(9999999L);
        h.revisionNumber = BigInteger.TWO;
        
        p.timeoutHeight = h;
        p.data = new byte[0];

        byte[] b = p.toBytes();
 
        SendPacket(b);
    }

    
}
