package ibc.icon.interfaces;

import foundation.icon.score.client.ScoreInterface;
import ibc.icon.structs.proto.core.channel.Channel;
import ibc.icon.structs.proto.core.channel.Counterparty;
import ibc.icon.structs.proto.core.channel.Packet;
import score.Address;

// IIBCModule defines an interface that implements all the callbacks
// that modules must define as specified in ICS-26
@ScoreInterface
public interface IIBCModule {
    // onChanOpenInit will verify that the relayer-chosen parameters
    // are valid and perform any custom INIT logic.
    // It may return an error if the chosen parameters are invalid
    // in which case the handshake is aborted.
    // If the provided version String is non-empty, OnChanOpenInit should return
    // the version String if valid or an error if the provided version is invalid.
    // If the version String is empty, OnChanOpenInit is expected to
    // return a default version String representing the version(s) it supports.
    // If there is no default version String for the application,
    // it should return an error if provided version is empty String.
    public void onChanOpenInit(
            Channel.Order order,
            String[] connectionHops,
            String portId,
            String channelId,
            Counterparty counterparty,
            String version);

    // OnChanOpenTry will verify the relayer-chosen parameters along with the
    // counterparty-chosen version String and perform custom TRY logic.
    // If the relayer-chosen parameters are invalid, the callback must return
    // an error to abort the handshake. If the counterparty-chosen version is not
    // compatible with this modules supported versions, the callback must return
    // an error to abort the handshake. If the versions are compatible, the try
    // callback
    // must select the final version String and return it to core IBC.
    // OnChanOpenTry may also perform custom initialization logic
    public void onChanOpenTry(
            Channel.Order order,
            String[] connectionHops,
            String portId,
            String channelId,
            Counterparty counterparty,
            String version,
            String counterpartyVersion);

    // OnChanOpenAck will error if the counterparty selected version String
    // is invalid to abort the handshake. It may also perform custom ACK logic.
    public void onChanOpenAck(String portId, String channelId, String counterpartyVersion);

    // OnChanOpenConfirm will perform custom CONFIRM logic and may error to abort
    // the handshake.
    public void onChanOpenConfirm(String portId, String channelId);

    public void onChanCloseInit(String portId, String channelId);

    public void onChanCloseConfirm(String portId, String channelId);

    // OnRecvPacket must return an acknowledgement that implements the
    // Acknowledgement interface.
    // In the case of an asynchronous acknowledgement, nil should be returned.
    // If the acknowledgement returned is successful, the state changes on callback
    // are written,
    // otherwise the application state changes are discarded. In either case the
    // packet is received
    // and the acknowledgement is written (in synchronous cases).
    public byte[] onRecvPacket(Packet calldata, Address relayer);

    public void onAcknowledgementPacket(Packet calldata, byte[] acknowledgement, Address relayer);
}
