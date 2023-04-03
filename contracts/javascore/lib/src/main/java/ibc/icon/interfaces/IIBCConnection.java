package ibc.icon.interfaces;

import foundation.icon.score.client.ScoreClient;
import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;
import score.annotation.EventLog;

@ScoreClient
public interface IIBCConnection {
    @EventLog(indexed = 1)
    public void ConnectionOpenInit(String clientId, String connectionId, byte[] counterparty);

    @EventLog(indexed = 1)
    public void ConnectionOpenTry(String clientId, String connectionId, byte[] counterparty);
    
    @EventLog(indexed = 1)
    public void ConnectionOpenAck(String connectionId, byte[] connection);

    @EventLog(indexed = 1)
    public void ConnectionOpenConfirm(String connectionId, byte[] connection);


    public void connectionOpenInit(MsgConnectionOpenInit msg);

    public void connectionOpenTry(MsgConnectionOpenTry msg);

    public void connectionOpenAck(MsgConnectionOpenAck msg);

    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg);
}
