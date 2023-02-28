package ibc.ics05.port;

import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.score.util.StringUtil;
import score.Address;
import score.ArrayDB;

/**
 * @dev ModuleManager is an abstract contract that provides the functions
 *      defined in [ICS
 *      5](https://github.com/cosmos/ibc/tree/main/spec/core/ics-005-port-allocation)
 *      and [ICS
 *      26](https://github.com/cosmos/ibc/blob/main/spec/core/ics-005-port-module/README.md).
 */
public abstract class ModuleManager {
    /**
     * @dev bindPort binds to an unallocated port, failing if the port has already
     *      been allocated.
     */
    public void bindPort(String portId, Address moduleAddress) {
        claimCapability(portCapabilityPath(portId), moduleAddress);
    }

    /**
     * @dev lookupModuleByPort will return the IBCModule along with the capability
     *      associated with a given portID
     */
    public IIBCModuleScoreInterface lookupModuleByPort(String portId) {
        ArrayDB<Address> modules = lookupModules(portCapabilityPath(portId));
        return new IIBCModuleScoreInterface(modules.get(0));

    }

    /**
     * @dev lookupModuleByChannel will return the IBCModule along with the
     *      capability associated with a given channel defined by its portID and
     *      channelID
     */
    public IIBCModuleScoreInterface lookupModuleByChannel(String portId, String channelId) {
        ArrayDB<Address> modules = lookupModules(channelCapabilityPath(portId, channelId));
        return new IIBCModuleScoreInterface(modules.get(0));
    }

    /**
     * @dev portCapabilityPath returns the path under which owner module address
     *      associated with a port should be stored.
     */
    public byte[] portCapabilityPath(String portId) {
        return StringUtil.encodePacked(portId);
    }

    /**
     * @dev channelCapabilityPath returns the path under which module address
     *      associated with a port and channel should be stored.
     */
    public byte[] channelCapabilityPath(String portId, String channelId) {
        return StringUtil.encodePacked(portId, "/", channelId);
    }

    /**
     * @dev claimCapability allows the IBC app module to claim a capability that
     *      core IBC passes to it
     */
    public abstract void claimCapability(byte[] name, Address addr);

    /**
     * @dev authenticateCapability attempts to authenticate a given name from a
     *      caller.
     *      It allows for a caller to check that a capability does in fact
     *      correspond to a particular name.
     */
    public abstract boolean authenticateCapability(byte[] name);

    /**
     * @dev lookupModule will return the IBCModule address bound to a given name.
     *      Currently, the function returns only one module.
     */
    public abstract ArrayDB<Address> lookupModules(byte[] name);

}
