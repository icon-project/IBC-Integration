package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;
import score.annotation.EventLog;
import score.annotation.External;

public abstract class IBCHandlerChannel extends IBCHandlerConnection {

    @EventLog(indexed = 1)
    public void GeneratedChannelIdentifier(String identifier) {

    }

    @External
    public String channelOpenInit(MsgChannelOpenInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);

        String id = super.channelOpenInit(msg);
        module.onChanOpenInit(
                msg.channel.channelOrdering(),
                msg.channel.getConnectionHops(),
                msg.portId,
                id,
                msg.channel.getCounterparty(),
                msg.channel.getVersion());
        claimCapability(channelCapabilityPath(msg.portId, id), module._address());

        GeneratedChannelIdentifier(id);
        return id;
    }

    @External
    public String channelOpenTry(MsgChannelOpenTry msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);

        String id = super.channelOpenTry(msg);
        module.onChanOpenTry(
                msg.channel.channelOrdering(),
                msg.channel.getConnectionHops(),
                msg.portId,
                id,
                msg.channel.getCounterparty(),
                msg.channel.getVersion(),
                msg.counterpartyVersion);
        claimCapability(channelCapabilityPath(msg.portId, id), module._address());

        GeneratedChannelIdentifier(id);
        return id;
    }

    @External
    public void channelOpenAck(MsgChannelOpenAck msg) {
        lookupModuleByPort(msg.portId).onChanOpenAck(msg.portId, msg.channelId, msg.counterpartyVersion);
        super.channelOpenAck(msg);
    }

    @External
    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        lookupModuleByPort(msg.portId).onChanOpenConfirm(msg.portId, msg.channelId);
        super.channelOpenConfirm(msg);
    }

    @External
    public void channelCloseInit(MsgChannelCloseInit msg) {
        lookupModuleByPort(msg.portId).onChanCloseInit(msg.portId, msg.channelId);
        super.channelCloseInit(msg);
    }

    @External
    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        lookupModuleByPort(msg.portId).onChanCloseConfirm(msg.portId, msg.channelId);
        super.channelCloseConfirm(msg);
    }

}
