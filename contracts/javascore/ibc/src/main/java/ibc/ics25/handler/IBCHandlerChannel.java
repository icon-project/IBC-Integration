package ibc.ics25.handler;

import ibc.icon.interfaces.IIBCChannelHandshake;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;
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
        lookupModuleByPort(msg.getPortId()).onChanOpenAck(msg.getPortId(), msg.getChannelId(),
                msg.getCounterpartyVersion());
        super.channelOpenAck(msg);
    }

    @External
    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        lookupModuleByPort(msg.getPortId()).onChanOpenConfirm(msg.getPortId(), msg.getChannelId());
        super.channelOpenConfirm(msg);
    }

    @External
    public void channelCloseInit(MsgChannelCloseInit msg) {
        lookupModuleByPort(msg.getPortId()).onChanCloseInit(msg.getPortId(), msg.getChannelId());
        super.channelCloseInit(msg);
    }

    @External
    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        lookupModuleByPort(msg.getPortId()).onChanCloseConfirm(msg.getPortId(), msg.getChannelId());
        super.channelCloseConfirm(msg);
    }

}
