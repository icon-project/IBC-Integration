// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/04-channel/Channel.proto

package com.icon.proto.core.channel;

public interface PacketStateOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.channel.PacketState)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * channel port identifier.
   * </pre>
   *
   * <code>string port_id = 1 [json_name = "portId"];</code>
   * @return The portId.
   */
  java.lang.String getPortId();
  /**
   * <pre>
   * channel port identifier.
   * </pre>
   *
   * <code>string port_id = 1 [json_name = "portId"];</code>
   * @return The bytes for portId.
   */
  com.google.protobuf.ByteString
      getPortIdBytes();

  /**
   * <pre>
   * channel unique identifier.
   * </pre>
   *
   * <code>string channel_id = 2 [json_name = "channelId"];</code>
   * @return The channelId.
   */
  java.lang.String getChannelId();
  /**
   * <pre>
   * channel unique identifier.
   * </pre>
   *
   * <code>string channel_id = 2 [json_name = "channelId"];</code>
   * @return The bytes for channelId.
   */
  com.google.protobuf.ByteString
      getChannelIdBytes();

  /**
   * <pre>
   * packet sequence.
   * </pre>
   *
   * <code>uint64 sequence = 3 [json_name = "sequence"];</code>
   * @return The sequence.
   */
  long getSequence();

  /**
   * <pre>
   * embedded data that represents packet state.
   * </pre>
   *
   * <code>bytes data = 4 [json_name = "data"];</code>
   * @return The data.
   */
  com.google.protobuf.ByteString getData();
}