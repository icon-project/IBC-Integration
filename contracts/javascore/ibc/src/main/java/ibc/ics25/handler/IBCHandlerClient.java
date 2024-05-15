package ibc.ics25.handler;

import ibc.ics04.channel.IBCPacket;
import icon.ibc.interfaces.IIBCClient;
import icon.ibc.structs.messages.MsgCreateClient;
import icon.ibc.structs.messages.MsgUpdateClient;
import score.annotation.EventLog;
import score.annotation.External;

public class IBCHandlerClient extends IBCPacket implements IIBCClient {

    @EventLog(indexed = 1)
    public void CreateClient(String identifier, byte[] clientState) {
    }


    @EventLog(indexed = 1)
    public void UpdateClient(String identifier, byte[] consensusHeight, byte[] clientMessage ) {
    }

    /**
     * createClient creates a new client state and populates it with a given
     * consensus state
     */
    public void createClient(MsgCreateClient msg) {
        String id = _createClient(msg);
        CreateClient(id, msg.getClientState());
    }

    /**
     * updateClient updates the consensus state and the state root from a
     * provided header
     */
    @External
    public void updateClient(MsgUpdateClient msg) {
        byte[] consensusHeight = _updateClient(msg);
        UpdateClient(msg.getClientId(), consensusHeight,  msg.getClientMessage());

    }

}
