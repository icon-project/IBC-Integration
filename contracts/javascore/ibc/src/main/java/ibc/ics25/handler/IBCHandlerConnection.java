package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCConnection;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import score.annotation.EventLog;
import score.annotation.External;

public class IBCHandlerConnection extends IBCHandlerClient implements IIBCConnection {

    @EventLog(indexed = 1)
    public void ConnectionOpenInit(String clientId, String connectionId, byte[] counterparty) {
    }

    @EventLog(indexed = 1)
    public void ConnectionOpenTry(String clientId, String connectionId, byte[] counterparty) {
    }

    @EventLog(indexed = 1)
    public void ConnectionOpenAck(String connectionId, byte[] connection) {
    }

    @EventLog(indexed = 1)
    public void ConnectionOpenConfirm(String connectionId, byte[] connection) {
    }

    @External
    public void connectionOpenInit(MsgConnectionOpenInit msg) {
        String id = _connectionOpenInit(msg);
        ConnectionOpenInit(msg.getClientId(), id, msg.getCounterparty());
    }

    @External
    public void connectionOpenTry(MsgConnectionOpenTry msg) {
        String id = _connectionOpenTry(msg);
        ConnectionOpenTry(msg.getClientId(), id, msg.getCounterparty());
    }

    @External
    public void connectionOpenAck(MsgConnectionOpenAck msg) {
        byte[] connection = _connectionOpenAck(msg);
        ConnectionOpenAck(msg.getConnectionId(), connection);
    }

    @External
    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg) {
        byte[] connection = _connectionOpenConfirm(msg);
        ConnectionOpenConfirm(msg.getConnectionId(), connection);
    }
}
