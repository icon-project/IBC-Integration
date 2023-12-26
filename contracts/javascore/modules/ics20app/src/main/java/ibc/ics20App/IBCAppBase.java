package ibc.ics20app;

import ibc.icon.interfaces.IIBCModule;
import score.Address;
import score.Context;
import score.VarDB;
import score.annotation.External;

public abstract class IBCAppBase  implements IIBCModule {

    private static final VarDB<Address> IBC_ADDRESS = Context.newVarDB("IBC_ADDRESS", Address.class);

    @External
    public void setIBCAddress(Address ibcAddress) {
        Context.require(Context.getCaller().equals(Context.getOwner()), "Only owner can set up the address");
        IBC_ADDRESS.set(ibcAddress);
    }

    @External(readonly = true)
    public Address getIBCAddress() {
        return IBC_ADDRESS.get();
    }

    public void onlyIBC() {
        Context.require(Context.getCaller().equals(getIBCAddress()), "ICS20App: Caller is not IBC Contract");
    }

}
