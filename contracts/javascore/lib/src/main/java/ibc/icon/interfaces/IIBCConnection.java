package ibc.icon.interfaces;

import ibc.icon.structs.messages.MsgConnectionOpenAck;
import ibc.icon.structs.messages.MsgConnectionOpenConfirm;
import ibc.icon.structs.messages.MsgConnectionOpenInit;
import ibc.icon.structs.messages.MsgConnectionOpenTry;

public interface IIBCConnection {

    public String connectionOpenInit(MsgConnectionOpenInit msg);

    public String connectionOpenTry(MsgConnectionOpenTry msg);

    public void connectionOpenAck(MsgConnectionOpenAck msg);

    public void connectionOpenConfirm(MsgConnectionOpenConfirm msg);
}
