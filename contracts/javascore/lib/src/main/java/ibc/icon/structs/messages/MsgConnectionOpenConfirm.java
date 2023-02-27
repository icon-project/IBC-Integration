
package ibc.icon.structs.messages;

import ibc.icon.structs.proto.core.client.Height;

public class MsgConnectionOpenConfirm {
    public String connectionId;
    public byte[] proofAck;
    public Height proofHeight;
}