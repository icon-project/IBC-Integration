package ibc.ics25.handler;

import ibc.icon.structs.messages.MsgCreateClient;
import ibc.icon.structs.messages.MsgUpdateClient;
import ibc.ics04.channel.IBCPacket;
import score.annotation.EventLog;
import score.annotation.External;

public abstract class IBCHandlerClient extends IBCPacket {

    @EventLog(indexed = 1)
    public void GeneratedClientIdentifier(String identifier) {
    }

    /**
     * createClient creates a new client state and populates it with a given
     * consensus state
     */
    @External
    public String createClient(MsgCreateClient msg) {
        String id = super.createClient(msg);
        GeneratedClientIdentifier(id);

        return id;
    }

    /**
     * updateClient updates the consensus state and the state root from a
     * provided header
     */
    @External
    public void updateClient(MsgUpdateClient msg) {
        super.updateClient(msg);
    }

}
