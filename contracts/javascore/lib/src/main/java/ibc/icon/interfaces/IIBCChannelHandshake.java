package ibc.icon.interfaces;

import ibc.icon.structs.messages.MsgChannelCloseConfirm;
import ibc.icon.structs.messages.MsgChannelCloseInit;
import ibc.icon.structs.messages.MsgChannelOpenAck;
import ibc.icon.structs.messages.MsgChannelOpenConfirm;
import ibc.icon.structs.messages.MsgChannelOpenInit;
import ibc.icon.structs.messages.MsgChannelOpenTry;

public interface IIBCChannelHandshake {
    /**
     * @dev channelOpenInit is called by a module to initiate a channel opening
     *      handshake with a module on another chain.
     */
    public String channelOpenInit(MsgChannelOpenInit msg);

    /**
     * @dev channelOpenTry is called by a module to accept the first step of a
     *      channel opening handshake initiated by a module on another chain.
     */
    public String channelOpenTry(MsgChannelOpenTry msg);

    /**
     * @dev channelOpenAck is called by the handshake-originating module to
     *      acknowledge the acceptance of the initial request by the counterparty
     *      module on the other chain.
     */
    public void channelOpenAck(MsgChannelOpenAck msg);

    /**
     * @dev channelOpenConfirm is called by the counterparty module to close their
     *      end of the channel, since the other end has been closed.
     */
    public void channelOpenConfirm(MsgChannelOpenConfirm msg);

    /**
     * @dev channelCloseInit is called by either module to close their end of the
     *      channel. Once closed, channels cannot be reopened.
     */
    public void channelCloseInit(MsgChannelCloseInit msg);

    /**
     * @dev channelCloseConfirm is called by the counterparty module to close their
     *      end of the
     *      channel, since the other end has been closed.
     */
    public void channelCloseConfirm(MsgChannelCloseConfirm msg);

}
