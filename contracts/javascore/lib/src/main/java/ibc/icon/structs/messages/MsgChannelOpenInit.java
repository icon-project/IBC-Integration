package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.channel.Channel;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class MsgChannelOpenInit {
    public String portId;
    public Channel channel;

    public static void writeObject(ObjectWriter writer, MsgChannelOpenInit obj) {
        obj.writeObject(writer);
    }

    public static MsgChannelOpenInit readObject(ObjectReader reader) {
        MsgChannelOpenInit obj = new MsgChannelOpenInit();
        reader.beginList();
        obj.portId = reader.readString();
        obj.channel = reader.read(Channel.class);

        reader.end();
        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(this.portId);
        writer.write(this.channel);

        writer.end();
    }

    public static MsgChannelOpenInit fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return MsgChannelOpenInit.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        MsgChannelOpenInit.writeObject(writer, this);
        return writer.toByteArray();
    }

    public String getPortId() {
        return portId;
    }

    public void setPortId(String portId) {
        this.portId = portId;
    }

    public Channel getChannel() {
        return channel;
    }

    public void setChannel(Channel channel) {
        this.channel = channel;
    }
}