package ibc.icon.interfaces;

import ibc.icon.structs.messages.*;

public interface IIBCChannelHandshake {
    /**
     * {@code @dev} channelOpenInit is called by a module to initiate a channel opening
     * handshake with a module on another chain.
     */
    String channelOpenInit(MsgChannelOpenInit msg);

    /**
     * {@code @dev} channelOpenTry is called by a module to accept the first step of a
     * channel opening handshake initiated by a module on another chain.
     */
    String channelOpenTry(MsgChannelOpenTry msg);

    /**
     * {@code @dev} channelOpenAck is called by the handshake-originating module to
     * acknowledge the acceptance of the initial request by the counterparty
     * module on the other chain.
     */
    void channelOpenAck(MsgChannelOpenAck msg);

    /**
     * {@code @dev} channelOpenConfirm is called by the counterparty module to close their
     * end of the channel, since the other end has been closed.
     */
    void channelOpenConfirm(MsgChannelOpenConfirm msg);

    /**
     * {@code @dev} channelCloseInit is called by either module to close their end of the
     * channel. Once closed, channels cannot be reopened.
     */
    void channelCloseInit(MsgChannelCloseInit msg);

    /**
     * {@code @dev} channelCloseConfirm is called by the counterparty module to close their
     * end of the
     * channel, since the other end has been closed.
     */
    void channelCloseConfirm(MsgChannelCloseConfirm msg);

}
