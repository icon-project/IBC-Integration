package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCConnection;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import score.annotation.EventLog;
import score.annotation.External;

public abstract class IBCHandlerConnection extends IBCHandlerClient implements IIBCConnection {

    @EventLog
    public void GeneratedConnectionIdentifier(String identifier) {
    }

    @External
    public String connectionOpenInit(MsgConnectionOpenInit msg) {
        String id = super.connectionOpenInit(msg);
        GeneratedConnectionIdentifier(id);

        return id;
    }

    @External
    public String connectionOpenTry(MsgConnectionOpenTry msg) {
        String id = super.connectionOpenTry(msg);
        GeneratedConnectionIdentifier(id);

        return id;
    }

    @External
    public void connectionOpenAck(MsgConnectionOpenAck msg) {
        super.connectionOpenAck(msg);
    }

    @External
    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg) {
        super.connectionOpenConfirm(msg);
    }
}
