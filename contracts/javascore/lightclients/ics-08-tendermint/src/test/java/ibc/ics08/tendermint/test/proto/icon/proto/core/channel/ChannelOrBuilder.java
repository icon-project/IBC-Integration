// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/04-channel/Channel.proto

package com.icon.proto.core.channel;

public interface ChannelOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.channel.Channel)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * current state of the channel end
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.State state = 1 [json_name = "state"];</code>
   * @return The enum numeric value on the wire for state.
   */
  int getStateValue();
  /**
   * <pre>
   * current state of the channel end
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.State state = 1 [json_name = "state"];</code>
   * @return The state.
   */
  com.icon.proto.core.channel.Channel.State getState();

  /**
   * <pre>
   * whether the channel is ordered or unordered
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.Order ordering = 2 [json_name = "ordering"];</code>
   * @return The enum numeric value on the wire for ordering.
   */
  int getOrderingValue();
  /**
   * <pre>
   * whether the channel is ordered or unordered
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.Order ordering = 2 [json_name = "ordering"];</code>
   * @return The ordering.
   */
  com.icon.proto.core.channel.Channel.Order getOrdering();

  /**
   * <pre>
   * counterparty channel end
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.Counterparty counterparty = 3 [json_name = "counterparty"];</code>
   * @return Whether the counterparty field is set.
   */
  boolean hasCounterparty();
  /**
   * <pre>
   * counterparty channel end
   * </pre>
   *
   * <code>.icon.proto.core.channel.Channel.Counterparty counterparty = 3 [json_name = "counterparty"];</code>
   * @return The counterparty.
   */
  com.icon.proto.core.channel.Channel.Counterparty getCounterparty();

  /**
   * <pre>
   * list of connection identifiers, in order, along which packets sent on
   * this channel will travel
   * </pre>
   *
   * <code>repeated string connection_hops = 4 [json_name = "connectionHops"];</code>
   * @return A list containing the connectionHops.
   */
  java.util.List<java.lang.String>
      getConnectionHopsList();
  /**
   * <pre>
   * list of connection identifiers, in order, along which packets sent on
   * this channel will travel
   * </pre>
   *
   * <code>repeated string connection_hops = 4 [json_name = "connectionHops"];</code>
   * @return The count of connectionHops.
   */
  int getConnectionHopsCount();
  /**
   * <pre>
   * list of connection identifiers, in order, along which packets sent on
   * this channel will travel
   * </pre>
   *
   * <code>repeated string connection_hops = 4 [json_name = "connectionHops"];</code>
   * @param index The index of the element to return.
   * @return The connectionHops at the given index.
   */
  java.lang.String getConnectionHops(int index);
  /**
   * <pre>
   * list of connection identifiers, in order, along which packets sent on
   * this channel will travel
   * </pre>
   *
   * <code>repeated string connection_hops = 4 [json_name = "connectionHops"];</code>
   * @param index The index of the element to return.
   * @return The connectionHops at the given index.
   */
  com.google.protobuf.ByteString
      getConnectionHopsBytes(int index);

  /**
   * <pre>
   * opaque channel version, which is agreed upon during the handshake
   * </pre>
   *
   * <code>string version = 5 [json_name = "version"];</code>
   * @return The version.
   */
  java.lang.String getVersion();
  /**
   * <pre>
   * opaque channel version, which is agreed upon during the handshake
   * </pre>
   *
   * <code>string version = 5 [json_name = "version"];</code>
   * @return The bytes for version.
   */
  com.google.protobuf.ByteString
      getVersionBytes();
}