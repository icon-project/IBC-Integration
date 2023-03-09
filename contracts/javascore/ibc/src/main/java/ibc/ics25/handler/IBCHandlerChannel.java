package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.structs.messages.*;
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
                msg.channel.getOrdering(),
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
                msg.channel.getOrdering(),
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
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);
        super.channelOpenAck(msg);
        module.onChanOpenAck(msg.portId, msg.channelId, msg.counterpartyVersion);
    }

    @External
    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);
        super.channelOpenConfirm(msg);
        module.onChanOpenConfirm(msg.portId, msg.channelId);
    }

    @External
    public void channelCloseInit(MsgChannelCloseInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);
        super.channelCloseInit(msg);
        module.onChanCloseInit(msg.portId, msg.channelId);
    }

    @External
    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.portId);
        super.channelCloseConfirm(msg);
        module.onChanCloseConfirm(msg.portId, msg.channelId);
    }

}
