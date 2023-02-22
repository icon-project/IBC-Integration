package ibc.ics24.host;

import ibc.icon.score.util.StringUtil;
import score.Context;

import java.math.BigInteger;

/**
 * // Commitment path generators that comply with
 * <a href=
 * "https://githu.com/cosmos/ibc/tree/main/spec/core/ics-024-host-requirements#path-space">path-space</a>
 */
public class IBCCommitment {
    private static final String KECCAK256 = "keccak-256";
    private static final String SHA256 = "sha-256";

    public static byte[] keccak256(byte[] msg) {
        return Context.hash(KECCAK256, msg);
    }

    public static byte[] sha256(byte[] msg) {
        return Context.hash(SHA256, msg);
    }

    public static byte[] clientStatePath(String clientId) {
        return StringUtil.encodePacked("clients/", clientId, "/clientState");
    }

    public static byte[] consensusStatePath(String clientId, BigInteger revisionNumber, BigInteger revisionHeight) {
        return StringUtil.encodePacked("clients/", clientId, "/consensusStates/", revisionNumber, "-", revisionHeight);
    }

    public static byte[] connectionPath(String connectionId) {
        return StringUtil.encodePacked("connections/", connectionId);
    }

    public static byte[] channelPath(String portId, String channelId) {
        return StringUtil.encodePacked("channelEnds/ports/", portId, "/channels/", channelId);
    }

    public static byte[] packetCommitmentPath(String portId, String channelId, BigInteger sequence) {
        return StringUtil.encodePacked("commitments/ports/", portId, "/channels/", channelId, "/sequences/", sequence);
    }

    public static byte[] packetAcknowledgementCommitmentPath(String portId, String channelId, BigInteger sequence) {
        return StringUtil.encodePacked("acks/ports/", portId, "/channels/", channelId, "/sequences/", sequence);
    }

    public static byte[] packetReceiptCommitmentPath(String portId, String channelId, BigInteger sequence) {
        return StringUtil.encodePacked("receipts/ports/", portId, "/channels/", channelId, "/sequences/", sequence);
    }

    public static byte[] nextSequenceRecvCommitmentPath(String portId, String channelId) {
        return StringUtil.encodePacked("nextSequenceRecv/ports/", portId, "/channels/", channelId);
    }

    // Key Generators for Commitment DBs

    public static byte[] clientStateCommitmentKey(String clientId) {
        return Context.hash(KECCAK256, clientStatePath(clientId));
    }

    public static byte[] consensusStateCommitmentKey(String clientId, BigInteger revisionNumber,
            BigInteger revisionHeight) {
        return Context.hash(KECCAK256, consensusStatePath(clientId, revisionNumber, revisionHeight));
    }

    public static byte[] connectionCommitmentKey(String connectionId) {
        return Context.hash(KECCAK256, connectionPath(connectionId));
    }

    public static byte[] channelCommitmentKey(String portId, String channelId) {
        return Context.hash(KECCAK256, channelPath(portId, channelId));
    }

    public static byte[] packetCommitmentKey(String portId, String channelId, BigInteger sequence) {
        return Context.hash(KECCAK256, packetCommitmentPath(portId, channelId, sequence));
    }

    public static byte[] packetAcknowledgementCommitmentKey(String portId, String channelId, BigInteger sequence) {
        return Context.hash(KECCAK256, packetAcknowledgementCommitmentPath(portId, channelId, sequence));
    }

    public static byte[] packetReceiptCommitmentKey(String portId, String channelId, BigInteger sequence) {
        return Context.hash(KECCAK256, packetReceiptCommitmentPath(portId, channelId, sequence));
    }

    public static byte[] nextSequenceRecvCommitmentKey(String portId, String channelId) {
        return Context.hash(KECCAK256, nextSequenceRecvCommitmentPath(portId, channelId));
    }

}
