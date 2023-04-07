package ibc.ics25.handler;

import java.util.List;

import ibc.icon.interfaces.IIBCChannelHandshake;
import ibc.icon.interfaces.IIBCModuleScoreInterface;
import ibc.icon.structs.messages.*;
import icon.proto.core.channel.Channel;
import score.annotation.EventLog;
import score.annotation.External;

public class IBCHandlerChannel extends IBCHandlerConnection implements IIBCChannelHandshake {

    @EventLog(indexed = 2)
    public void ChannelOpenInit(String portId, String channelId, byte[] channel) {

    }

    @EventLog(indexed = 2)
    public void ChannelOpenTry(String portId, String channelId, byte[] channel) {

    }

    @EventLog(indexed = 2)
    public void ChannelOpenAck(String portId, String channelId, byte[] channel) {

    }

    @EventLog(indexed = 2)
    public void ChannelOpenConfirm(String portId, String channelId, byte[] channel) {

    }

    @EventLog(indexed = 2)
    public void ChannelCloseInit(String portId, String channelId, byte[] channel) {

    }

    @EventLog(indexed = 2)
    public void ChannelCloseConfirm(String portId, String channelId, byte[] channel) {

    }

    @External
    public void channelOpenInit(MsgChannelOpenInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        // TODO optimize to not do decoding twice
        Channel channel = Channel.decode(msg.getChannel());
        String id = _channelOpenInit(msg);
        module.onChanOpenInit(
                channel.getOrdering(),
                connectionHopsToArray(channel.getConnectionHops()),
                msg.getPortId(),
                id,
                channel.getCounterparty().encode(),
                channel.getVersion());
        claimCapability(channelCapabilityPath(msg.getPortId(), id), module._address());

        ChannelOpenInit(msg.getPortId(), id, msg.getChannel());
    }

    @External
    public void channelOpenTry(MsgChannelOpenTry msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        // TODO optimize to not do decoding twice
        Channel channel = Channel.decode(msg.getChannel());
        String id = _channelOpenTry(msg);
        module.onChanOpenTry(
                channel.getOrdering(),
                connectionHopsToArray(channel.getConnectionHops()),
                msg.getPortId(),
                id,
                channel.getCounterparty().encode(),
                channel.getVersion(),
                msg.getCounterpartyVersion());
        claimCapability(channelCapabilityPath(msg.getPortId(), id), module._address());

        ChannelOpenTry(msg.getPortId(), id, msg.getChannel());
    }

    @External
    public void channelOpenAck(MsgChannelOpenAck msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        byte[] channel = _channelOpenAck(msg);
        module.onChanOpenAck(msg.getPortId(), msg.getChannelId(), msg.getCounterpartyVersion());
        ChannelOpenAck(msg.getPortId(), msg.getChannelId(), channel);
    }

    @External
    public void channelOpenConfirm(MsgChannelOpenConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        byte[] channel = _channelOpenConfirm(msg);
        module.onChanOpenConfirm(msg.getPortId(), msg.getChannelId());
        ChannelOpenConfirm(msg.getPortId(), msg.getChannelId(), channel);
    }

    @External
    public void channelCloseInit(MsgChannelCloseInit msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        byte[] channel = _channelCloseInit(msg);
        module.onChanCloseInit(msg.getPortId(), msg.getChannelId());
        ChannelCloseInit(msg.getPortId(), msg.getChannelId(), channel);

    }

    @External
    public void channelCloseConfirm(MsgChannelCloseConfirm msg) {
        IIBCModuleScoreInterface module = lookupModuleByPort(msg.getPortId());
        byte[] channel = _channelCloseConfirm(msg);
        module.onChanCloseConfirm(msg.getPortId(), msg.getChannelId());
        ChannelCloseConfirm(msg.getPortId(), msg.getChannelId(), channel);
    }

    private String[] connectionHopsToArray(List<String> hops) {
        String[] hopsArray = new String[hops.size()];
        for(int i = 0; i < hops.size(); i++){
            hopsArray[i] = hops.get(i);
        }

        return hopsArray;
    }

}
