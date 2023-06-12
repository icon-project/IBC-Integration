package ibc.ics25.handler;

import score.Address;
import score.Context;
import score.annotation.External;

import java.math.BigInteger;

import ibc.icon.structs.messages.MsgCreateClient;

public class IBCHandler extends IBCHandlerPacket {

    public static final String name = "ICON IBC Handler";

    public IBCHandler() {
    }

    @External(readonly = true)
    public String name() {
        return name;
    }

    /**
     * registerClient registers a new client type into the client registry
     */
    @External
    public void registerClient(String clientType, Address client) {
        onlyOwner();
        super.registerClient(clientType, client);
    }

    @External
    public void createClient(MsgCreateClient msg) {
        onlyOwner();
        super.createClient(msg);
    }

    /**
     * bindPort binds to an unallocated port, failing if the port has already
     * been allocated.
     */
    @External
    public void bindPort(String portId, Address moduleAddress) {
        onlyOwner();
        addPortId(portId);
        super.bindPort(portId, moduleAddress);
    }

    /**
     * setExpectedTimePerBlock sets expected time per block.
     */
    @External
    public void setExpectedTimePerBlock(BigInteger expectedTimePerBlock) {
        onlyOwner();
        super.setExpectedTimePerBlock(expectedTimePerBlock);
    }

    public static void onlyOwner() {
        Address caller = Context.getCaller();
        Address owner = Context.getOwner();
        Context.require(caller.equals(owner), "SenderNotScoreOwner: Sender=" + caller + "Authorized=" + owner);
    }
}
