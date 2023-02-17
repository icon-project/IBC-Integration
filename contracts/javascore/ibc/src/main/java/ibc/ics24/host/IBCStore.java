package ibc.ics24.host;

import java.math.BigInteger;
import score.Address;
import score.Context;
import score.DictDB;
import score.VarDB;

public class IBCStore {
    public final DictDB<String, Address> clientRegistry = Context.newDictDB("clientRegistry", Address.class);
    public final DictDB<String, Address> clientImpls = Context.newDictDB("clientImpls", Address.class);
    public final VarDB<BigInteger> nextClientSequence = Context.newVarDB("nextClientSequence", BigInteger.class);



}
