package ibc.ics24.host;

import score.Address;
import score.Context;

import java.math.BigInteger;

import ibc.icon.score.util.NullChecker;

public class IBCHost extends IBCStore {
    private static final String TAG = "IBCHOST: ";
    private static final Address chainScore = Address.fromString("cx0000000000000000000000000000000000000000");

    /***
     * claimCapability allows the IBC app module to claim a capability that core IBC
     * passes to it
     *
     * @param name Name of the capability to claim
     * @param addr Address for which the capability is to be claimed
     *
     */
    public void claimCapability(byte[] name, Address addr) {
        Context.require(capabilities.get(name) == null,  TAG + "Capability already claimed");
        capabilities.set(name, addr);
    }

    /***
     * addPortId adds given portId to a list of portIds that we track
     *
     * @param portId Name of portId
     */
    public void addPortId(String portId) {
        Context.require(portId != null, TAG + "Port Id cannot be null");
        Context.require(capabilities.get(portCapabilityPath(portId)) == null,TAG + "PortId already exists");
        portIds.add(portId);
    }

    /**
     * authenticateCapability attempts to authenticate a given name from a caller.
     * It allows for a caller to check
     * that a capability does in fact correspond to a particular name.
     *
     * @param name Name of the capability to authenticate
     * @return True if the capability exists for the caller
     */
    public boolean authenticateCapability(byte[] name) {
        Address caller = Context.getCaller();
        return caller.equals(capabilities.get(name));
    }

    /**
     * lookupModules will return the IBCModule addresses bound to a given name
     *
     * @param name Name of the capability
     * @return ArrayDB of addresses having the capability
     */
    public Address lookupModules(byte[] name) {
        Address module = capabilities.get(name);
        Context.require(module != null,  "Module not found");
        return module;
    }

    /**
     * setExpectedTimePerBlock sets expected time per block
     *
     * @param expectedTimePerBlock time per block to set
     */
    public void setExpectedTimePerBlock(BigInteger expectedTimePerBlock) {
        IBCHost.expectedTimePerBlock.set(expectedTimePerBlock);
    }

    public void sendBTPMessage(String clientId, byte[] message) {
        int id = btpNetworkId.get(clientId);
        NullChecker.requireNotNull(id, "BTP network not configured");
        Context.call(chainScore, "sendBTPMessage", id, message);
    }

}
