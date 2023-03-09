package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCChannelHandshake;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.structs.messages.*;
import icon.proto.core.channel.Channel;
import score.annotation.EventLog;
import score.annotation.External;

public abstract class IBCHandlerChannel extends IBCHandlerConnection implements IIBCChannelHandshake {

    @EventLog(indexed = 1)
    public void GeneratedChannelIdentifier(String identifier) {

    }

    @External
    public String channelOpenInit(MsgChannelOpenInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        // TODO optimize to not do decoding twice
        Channel channel = msg.getChannel();
        String id = super.channelOpenInit(msg);
        module.onChanOpenInit(
                channel.getOrdering(),
                channel.getConnectionHops(),
                msg.getPortId(),
                id,
                channel.getCounterparty().encode(),
                channel.getVersion());
        claimCapability(channelCapabilityPath(msg.getPortId(), id), module._address());

        GeneratedChannelIdentifier(id);
        return id;
    }

    @External
    public String channelOpenTry(MsgChannelOpenTry msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        // TODO optimize to not do decoding twice
        Channel channel = msg.getChannel();
        String id = super.channelOpenTry(msg);
        module.onChanOpenTry(
                channel.getOrdering(),
                channel.getConnectionHops(),
                msg.getPortId(),
                id,
                channel.getCounterparty().encode(),
                channel.getVersion(),
                msg.getCounterpartyVersion());
        claimCapability(channelCapabilityPath(msg.getPortId(), id), module._address());

        GeneratedChannelIdentifier(id);
        return id;
    }

    @External
    public void channelOpenAck(MsgChannelOpenAck msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        super.channelOpenAck(msg);
        module.onChanOpenAck(msg.getPortId(), msg.getChannelId(), msg.getCounterpartyVersion());
    }

    @External
    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        super.channelOpenConfirm(msg);
        module.onChanOpenConfirm(msg.getPortId(), msg.getChannelId());
    }

    @External
    public void channelCloseInit(MsgChannelCloseInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        super.channelCloseInit(msg);
        module.onChanCloseInit(msg.getPortId(), msg.getChannelId());
    }

    @External
    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        super.channelCloseConfirm(msg);
        module.onChanCloseConfirm(msg.getPortId(), msg.getChannelId());
    }

}
