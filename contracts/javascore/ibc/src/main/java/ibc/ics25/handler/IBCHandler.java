package ibc.ics25.handler;

import java.math.BigInteger;

import score.Address;
import score.Context;
import score.annotation.External;

public class IBCHandler extends IBCHandlerPacket {

    public static final String name = "ICON IBC Handler";

    public IBCHandler() {
    }

    @External(readonly = true)
    public String name() {
        return name;
    }

    /**
     * @dev registerClient registers a new client type into the client registry
     */
    public void registerClient(String clientType, Address client) {
        onlyOwner();
        super.registerClient(clientType, client);
    }

    /**
     * @dev bindPort binds to an unallocated port, failing if the port has already
     *      been allocated.
     */
    public void bindPort(String portId, Address moduleAddress) {
        onlyOwner();
        super.bindPort(portId, moduleAddress);
    }

    /**
     * @dev setExpectedTimePerBlock sets expected time per block.
     */
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
