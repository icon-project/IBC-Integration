package ibc.ics20;

import com.eclipsesource.json.Json;
import com.eclipsesource.json.JsonObject;
import com.eclipsesource.json.JsonValue;

import icon.proto.core.channel.Channel;
import icon.proto.core.channel.Packet;
import icon.proto.core.client.Height;
import ics20.ICS20Lib;
import score.Address;
import score.Context;
import score.DictDB;
import score.VarDB;
import score.annotation.External;
import score.annotation.Optional;
import score.annotation.Payable;
import icon.ibc.interfaces.IIBCModule;
import java.util.Arrays;


import java.math.BigInteger;

public class ICS20Transfer implements IIBCModule {
    public static final String TAG = "ICS20";
    public static final String ICS20_VERSION = "ics20-1";

    private final DictDB<String, String> destinationPort = Context.newDictDB("destinationPort", String.class);
    private final DictDB<String, String> destinationChannel = Context.newDictDB("destinationChannel", String.class);

    private final VarDB<Address> ibcHandler = Context.newVarDB("ibcHandler", Address.class);
    private final DictDB<String, Address> tokenContracts = Context.newDictDB("tokenContracts", Address.class);
    private final VarDB<Address> admin = Context.newVarDB("admin", Address.class);

    public final byte[] serializedIrc2;

    public ICS20Transfer(Address _ibcHandler, byte[] _serializeIrc2) {
        if (ibcHandler.get() == null) {
            ibcHandler.set(_ibcHandler);
            admin.set(Context.getCaller());
        }
        serializedIrc2 = _serializeIrc2;
    }

    /**
     * Set the admin address and ensure only admin can call this function.
     *
     * @param _admin the new admin address
     * @return void
     */
    @External
    public void setAdmin(Address _admin) {
        onlyAdmin();
        admin.set(_admin);
    }

    /**
     * Retrieves the admin address.
     *
     * @return the admin address
     */
    @External(readonly = true)
    public Address getAdmin() {
        return admin.get();
    }

    /**
     * Retrieves the IBC handler address.
     * 
     * @return the IBC handler address
     */
    @External(readonly = true)
    public Address getIBCAddress() {
        return ibcHandler.get();
    }

    /**
     * Retrieves the destination port for the given channel ID.
     *
     * @param channelId the source channel id
     * @return the destination port associated with the channel ID
     */
    @External(readonly = true)
    public String getDestinationPort(String channelId) {
        return destinationPort.get(channelId);
    }

    /**
     * Retrieves the destination channel for the given channel ID.
     * 
     * @param channelId the source channel id
     * @return the destination channel associated with the channel ID
     */
    @External(readonly = true)
    public String getDestinationChannel(String channelId) {
        return destinationChannel.get(channelId);
    }

    /**
     * Retrieves the token contract address for the given denom.
     * 
     * @param denom the token denom
     * @return the token contract address
     */
    @External(readonly = true)
    public Address getTokenContractAddress(String denom) {
        Context.require(tokenContracts.get(denom) != null, TAG + " : Token not registered");
        return tokenContracts.get(denom);
    }

    /**
     * Register a token contract for cosmos chain.
     * 
     * @param name
     * @param symbol
     * @param decimals
     */
    @External
    public void registerCosmosToken(String name, String symbol, int decimals) {
        onlyAdmin();
        Address tokenAddress = Context.deploy(serializedIrc2, name, symbol, decimals);
        tokenContracts.set(name, tokenAddress);
    }

    /**
     * Register a token contract for icon chain.
     * 
     * @param tokenAddress the irc2 token contract address
     */
    @External
    public void registerIconToken(Address tokenAddress) {
        onlyAdmin();
        tokenContracts.set(tokenAddress.toString(), tokenAddress);
    }

    /**
     * Fallback function for token transfer.
     * 
     * @param from  Sender address
     * @param value Amount
     * @param _data Data in json bytes in format of
     *              {
     *              "method": "sendFungibleTokens",
     *              "params": {
     *              "denomination": "string",
     *              "amount": "uint64",
     *              "sender": "string",
     *              "receiver": "string",
     *              "sourcePort": "string",
     *              "sourceChannel": "string",
     *              "timeoutHeight": {
     *              "latestHeight": "uint64",
     *              "revisionNumber": "uint64",
     *              },
     *              "timeoutTimestamp": "uint64",
     *              "memo":"string"
     *              }
     *              }
     * 
     */
    @External
    public void tokenFallback(Address from, BigInteger value, byte[] _data) {
        String method = "";
        JsonValue params = null;

        try {
            String data = new String(_data);
            JsonObject json = Json.parse(data).asObject();

            method = json.get("method").asString();
            params = json.get("params");
        } catch (Exception e) {
            Context.revert(TAG + " Invalid data: " + _data.toString());
        }

        if (method.equals("sendFungibleTokens")) {
            JsonObject fungibleToken = params.asObject();
            String denomination = fungibleToken.getString("denomination", "");
            BigInteger amount = BigInteger.valueOf(fungibleToken.getLong("amount", 0));
            String sender = fungibleToken.getString("sender", "");
            String receiver = fungibleToken.getString("receiver", "");
            String sourcePort = fungibleToken.getString("sourcePort", "");
            String sourceChannel = fungibleToken.getString("sourceChannel", "");
            BigInteger timeoutTimestamp = BigInteger.valueOf(fungibleToken.getLong("timeoutTimestamp", 0));
            String memo = fungibleToken.getString("memo", "");

            JsonObject timeoutHeight = fungibleToken.get("timeoutHeight").asObject();
            Height height = new Height();
            height.setRevisionNumber(BigInteger.valueOf(timeoutHeight.getLong("revisionNumber", 0)));
            height.setRevisionHeight(BigInteger.valueOf(timeoutHeight.getLong("latestHeight", 0)));

            Context.require(amount.equals(value), TAG + " : Mismatched amount");
            Context.require(sender.equals(from.toString()), TAG + " : Sender address mismatched");
            Context.require(tokenContracts.get(denomination) == Context.getCaller(),
                    TAG + " : Sender Token Contract not registered");

            sendFungibleToken(denomination, amount, sender, receiver, sourcePort, sourceChannel, height,
                    timeoutTimestamp, memo);
        } else {
            Context.revert(TAG + " : Unknown method");
        }

    }

    /**
     * Sends ICX to the specified receiver via the specified channel and port.
     *
     * @param receiver         the cross chain address of the receiver
     * @param sourcePort       the source port
     * @param sourceChannel    the source channel
     * @param timeoutHeight    the timeout height
     * @param timeoutTimestamp the timeout timestamp
     * @param memo             an optional memo
     */
    @Payable
    @External
    public void sendICX(String receiver, String sourcePort, String sourceChannel, Height timeoutHeight,
            BigInteger timeoutTimestamp, @Optional String memo) {
        Context.require(Context.getValue().compareTo(BigInteger.ZERO) > 0,
                TAG + " : ICX amount should be greater than 0");

        sendFungibleToken("icx", Context.getValue(), Context.getCaller().toString(), receiver, sourcePort,
                sourceChannel, timeoutHeight, timeoutTimestamp, memo);

    }

    /**
     * Sends a irc2 token from the sender to the receiver.
     *
     * @param denomination     the denomination of the token to send
     * @param amount           the amount of the token to send
     * @param sender           the address of the sender
     * @param receiver         the cross chain address of the receiver
     * @param sourcePort       the source port
     * @param sourceChannel    the source channel
     * @param timeoutHeight    the timeout height(latest height and revision number)
     * @param timeoutTimestamp the timeout timestamp
     * @param memo             an optional memo for the transaction
     */
    private void sendFungibleToken(String denomination, BigInteger amount, String sender, String receiver,
            String sourcePort, String sourceChannel, Height timeoutHeight, BigInteger timeoutTimestamp,
            @Optional String memo) {
        String denomPrefix = getDenomPrefix(sourcePort, sourceChannel);
        boolean isSource = !denomination.startsWith(denomPrefix);

        if (!isSource) {
            Address tokenContractAddress = getTokenContractAddress(denomination);
            Context.call(tokenContractAddress, "burn", amount);
        }

        byte[] data = ICS20Lib.marshalFungibleTokenPacketData(denomination, amount, sender, receiver, memo);

        String destPort = destinationPort.get(sourceChannel);
        String destChannel = destinationChannel.get(sourceChannel);

        if (destChannel == null || destPort == null) {
            Context.revert(TAG + " : Connection not properly Configured");
        }

        BigInteger seq = Context.call(BigInteger.class, ibcHandler.get(), "getNextSequenceSend", sourcePort,
                sourceChannel);

        Packet newPacket = new Packet();

        newPacket.setSequence(seq);
        newPacket.setSourcePort(sourcePort);
        newPacket.setSourceChannel(sourceChannel);
        newPacket.setDestinationPort(destPort);
        newPacket.setDestinationChannel(destChannel);
        newPacket.setTimeoutHeight(timeoutHeight);
        newPacket.setTimeoutTimestamp(timeoutTimestamp);
        newPacket.setData(data);

        Context.call(ibcHandler.get(), "sendPacket", newPacket.encode());
    }

    /**
     * Handles the reception of a packet
     *
     * @param packet  the byte array representation of the packet to be processed
     * @param relayer the address of the relayer
     * @return a byte array representing the acknowledgement of the packet
     *         processing
     */
    @External
    public byte[] onRecvPacket(byte[] packet, Address relayer) {
        onlyIBC();
        Packet packetDb = Packet.decode(packet);
        ICS20Lib.FungibleTokenPacketData data;

        try {
            data = ICS20Lib.unmarshalFungibleTokenPacketData(packetDb.getData());
            Context.require(!data.denom.equals("") && !data.receiver.equals("") && !data.sender.equals("")
                    && data.amount.compareTo(BigInteger.ZERO) > 0);
        } catch (Exception e) {
            return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }

        String denomPrefix = getDenomPrefix(packetDb.getSourcePort(), packetDb.getSourceChannel());
        boolean isSource = data.denom.startsWith(denomPrefix);

        byte[] ack = ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON;

        if (!checkIfReceiverIsAddress(data.receiver)) {
            return ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }

        Address receiverAddr = Address.fromString(data.receiver);

        try {
            if (isSource) {
                String denomOnly = data.denom.substring(denomPrefix.length());
                handleSourceToken(denomOnly, receiverAddr, data.amount, data.memo);
            } else {
                denomPrefix = getDenomPrefix(packetDb.getDestinationPort(), packetDb.getDestinationChannel());
                String prefixedDenom = denomPrefix + data.denom;
                handleDestinationToken(prefixedDenom, receiverAddr, data.amount);
            }
        } catch (Exception e) {
            ack = ICS20Lib.FAILED_ACKNOWLEDGEMENT_JSON;
        }

        return ack;
    }

    /**
     * Handles the acknowledgement of a packet.
     *
     * @param packet          the packet being acknowledged
     * @param acknowledgement the acknowledgement received
     * @param relayer         the relayer of the packet
     */
    @External
    public void onAcknowledgementPacket(byte[] packet, byte[] acknowledgement, Address relayer) {
        onlyIBC();
        if (!Arrays.equals(acknowledgement,ICS20Lib.SUCCESSFUL_ACKNOWLEDGEMENT_JSON)) {
            Packet packetDb = Packet.decode(packet);
            refundTokens(packetDb);
        }
    }

    /**
     * Handles the timeout of a packet by refunding the tokens associated with the
     * packet.
     *
     * @param packet  the encoded packet data
     * @param relayer the address of the relayer
     */
    @External
    public void onTimeoutPacket(byte[] packet, Address relayer) {
        Packet packetDb = Packet.decode(packet);
        refundTokens(packetDb);
    }

    /**
     * Refunds tokens based on the provided packet.
     *
     * @param packet the packet containing the token data
     */
    private void refundTokens(Packet packet) {
        ICS20Lib.FungibleTokenPacketData data = ICS20Lib.unmarshalFungibleTokenPacketData(packet.getData());

        String denomPrefix = getDenomPrefix(packet.getSourcePort(), packet.getSourceChannel());
        boolean isSource = !data.denom.startsWith(denomPrefix);

        Address sender = Address.fromString(data.sender);

        if (isSource) {
            handleSourceToken(data.denom, sender, data.amount, data.memo);
        } else {
            handleDestinationToken(data.denom, sender, data.amount);
        }
    }

    private void handleSourceToken(String denom, Address address, BigInteger amount, String memo) {
        if (isNativeAsset(denom)) {
            Context.transfer(address, amount);
        } else {
            Address tokenContractAddress = getTokenContractAddress(denom);
            Context.call(tokenContractAddress, "transfer", address, amount, memo.getBytes());
        }
    }

    private void handleDestinationToken(String denom, Address address, BigInteger amount) {
        Address tokenContractAddress = getTokenContractAddress(denom);
        Context.call(tokenContractAddress, "mint", address, amount);
    }

    /**
     * Initializes the channel opening process.
     *
     * @param order          the order of the channel
     * @param connectionHops the connection hops for the channel
     * @param portId         the port ID for the channel
     * @param channelId      the channel ID
     * @param counterpartyPb the counterparty information
     * @param version        the version of the channel
     */
    @External
    public void onChanOpenInit(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version) {
        onlyIBC();
        Context.require(order == Channel.Order.ORDER_UNORDERED, TAG + " : must be unordered");
        Context.require(version.equals(ICS20_VERSION), TAG + " : version should be same with ICS20_VERSION");
        Channel.Counterparty counterparty = Channel.Counterparty.decode(counterpartyPb);
        destinationPort.set(channelId, counterparty.getPortId());
    }

    /**
     * Channel Opening Process
     *
     * @param order               the order of the channel
     * @param connectionHops      an array of connection hops
     * @param portId              the port ID
     * @param channelId           the channel ID
     * @param counterpartyPb      the counterparty in protobuf format
     * @param version             the version
     * @param counterPartyVersion the counterparty version
     */
    @External
    public void onChanOpenTry(int order, String[] connectionHops, String portId, String channelId,
            byte[] counterpartyPb, String version, String counterPartyVersion) {
        onlyIBC();
        Context.require(order == Channel.Order.ORDER_UNORDERED, TAG + " : must be unordered");
        Context.require(counterPartyVersion.equals(ICS20_VERSION),
                TAG + " : version should be same with ICS20_VERSION");
        Channel.Counterparty counterparty = Channel.Counterparty.decode(counterpartyPb);
        destinationPort.set(channelId, counterparty.getPortId());
        destinationChannel.set(channelId, counterparty.getChannelId());
    }

    /**
     * Handles the acknowledged by the counterparty.
     *
     * @param portId                the identifier of the port on this chain
     * @param channelId             the identifier of the channel that was opened
     * @param counterpartyChannelId the identifier of the channel on the
     *                              counterparty chain
     * @param counterPartyVersion   the version of the ICS20 protocol used by the
     *                              counterparty
     */
    @External
    public void onChanOpenAck(String portId, String channelId, String counterpartyChannelId,
            String counterPartyVersion) {
        onlyIBC();
        Context.require(counterPartyVersion.equals(ICS20_VERSION),
                TAG + " : version should be same with ICS20_VERSION");
        destinationChannel.set(channelId, counterpartyChannelId);
    }

    /**
     * Handles the confirmation of a channel.
     * 
     * @param portId    the identifier of the port on this chain
     * @param channelId the identifier of the channel that was opened
     */
    @External
    public void onChanOpenConfirm(String portId, String channelId) {
        onlyIBC();
    }

    /**
     * Handles the closure of a channel.
     * 
     * @param portId    the identifier of the port on this chain
     * @param channelId the identifier of the channel that was opened
     */
    @External
    public void onChanCloseInit(String portId, String channelId) {
        Context.revert(TAG + " : Not Allowed");
    }

    /**
     * Handles the closing of a channel.
     * 
     * @param portId    the identifier of the port on this chain
     * @param channelId the identifier of the channel that was opened
     */
    @External
    public void onChanCloseConfirm(String portId, String channelId) {
        onlyIBC();
    }

    private static String getDenomPrefix(String port, String channel) {
        return port + "/" + channel + "/";
    }

    private void onlyAdmin() {
        Context.require(Context.getCaller().equals(admin.get()), TAG + " : Caller is not admin");
    }

    private void onlyIBC() {
        Context.require(Context.getCaller().equals(getIBCAddress()), TAG + " : Caller is not IBC Contract");
    }

    private boolean isNativeAsset(String denom) {
        return denom.equals("icx");
    }

    private static boolean checkIfReceiverIsAddress(String receiver) {
        try {
            Address.fromString(receiver);
            return true;
        } catch (Exception e) {
            return false;
        }
    }

}
