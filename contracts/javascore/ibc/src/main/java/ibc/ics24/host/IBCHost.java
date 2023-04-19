package ibc.ics24.host;

import score.Address;
import score.ArrayDB;
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
        ArrayDB<Address> capability = capabilities.at(name);
        int capabilitiesCount = capability.size();
        if (capabilitiesCount == 0) {
            portIds.add(name);
        }
        for (int i = 0; i < capabilitiesCount; i++) {
            Context.require(!capability.get(i).equals(addr), TAG + "Capability already claimed");
        }
        capability.add(addr);
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
        ArrayDB<Address> capability = capabilities.at(name);
        int capabilitiesCount = capability.size();
        for (int i = 0; i < capabilitiesCount; i++) {
            if (capability.get(i).equals(caller)) {
                return Boolean.TRUE;
            }
        }
        return Boolean.FALSE;
    }

    /**
     * lookupModules will return the IBCModule addresses bound to a given name
     *
     * @param name Name of the capability
     * @return ArrayDB of addresses having the capability
     */
    public ArrayDB<Address> lookupModules(byte[] name) {
        ArrayDB<Address> modules = capabilities.at(name);
        Context.require(modules.size() > 0, "Module not found");
        return modules;
    }

    /**
     * setExpectedTimePerBlock sets expected time per block
     *
     * @param expectedTimePerBlock time per block to set
     */
    public void setExpectedTimePerBlock(BigInteger expectedTimePerBlock) {
        this.expectedTimePerBlock.set(expectedTimePerBlock);
    }

    public void sendBTPMessage(String clientId, byte[] message) {
        int id = btpNetworkId.get(clientId);
        NullChecker.requireNotNull(id, "BTP network not configured");
        Context.call(chainScore, "sendBTPMessage", id, message);
    }

}
