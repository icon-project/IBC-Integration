package foundation.icon.btp.xcall;

import foundation.icon.btp.xcall.data.CSMessage;
import foundation.icon.btp.xcall.data.CSMessageRequest;
import foundation.icon.btp.xcall.data.CallRequest;
import foundation.icon.btp.xcall.interfaces.CallService;
import ibc.icon.interfaces.IIBCModule;
import java.math.BigInteger;
import score.Address;
import score.Context;
import score.DictDB;
import score.VarDB;

public abstract class AbstractCallService implements CallService, IIBCModule {


    protected static final int MAX_DATA_SIZE = 2048;
    protected static final int MAX_ROLLBACK_SIZE = 1024;

    protected final VarDB<Address> ibcHandler = Context.newVarDB("ibcHandler", Address.class);
    protected final VarDB<BigInteger> sn = Context.newVarDB("sn", BigInteger.class);
    public static final VarDB<BigInteger> recvCount = Context.newVarDB("recvPacket", BigInteger.class);
    protected final VarDB<BigInteger> reqId = Context.newVarDB("reqId", BigInteger.class);
    protected final VarDB<BigInteger> timeoutHeight = Context.newVarDB("timeoutHeight", BigInteger.class);

    protected final DictDB<BigInteger, CallRequest> requests = Context.newDictDB("requests", CallRequest.class);
    protected final DictDB<BigInteger, CSMessageRequest> proxyReqs = Context.newDictDB("proxyReqs",
            CSMessageRequest.class);

    // for fee-related operations
    protected final VarDB<Address> admin = Context.newVarDB("admin", Address.class);
    protected final VarDB<Address> feeHandler = Context.newVarDB("feeHandler", Address.class);
    protected final VarDB<BigInteger> protocolFee = Context.newVarDB("protocolFee", BigInteger.class);
    protected final VarDB<String> sourcePort = Context.newVarDB("sourcePort", String.class);
    protected final VarDB<String> sourceChannel = Context.newVarDB("sourceChannel", String.class);
    protected final VarDB<String> destinationPort = Context.newVarDB("destinationPort", String.class);
    protected final VarDB<String> destinationChannel = Context.newVarDB("destinationChannel", String.class);


    protected byte[] createMessage(int type, byte[] data){
        CSMessage message=new CSMessage(type,data);
        return message.toBytes();
    }
}
