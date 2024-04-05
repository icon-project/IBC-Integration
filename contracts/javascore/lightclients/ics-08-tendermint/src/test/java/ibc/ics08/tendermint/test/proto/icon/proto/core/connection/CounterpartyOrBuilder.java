// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/03-connection/Connection.proto

package com.icon.proto.core.connection;

public interface CounterpartyOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.connection.Counterparty)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * identifies the client on the counterparty chain associated with a given
   * connection.
   * </pre>
   *
   * <code>string client_id = 1 [json_name = "clientId"];</code>
   * @return The clientId.
   */
  java.lang.String getClientId();
  /**
   * <pre>
   * identifies the client on the counterparty chain associated with a given
   * connection.
   * </pre>
   *
   * <code>string client_id = 1 [json_name = "clientId"];</code>
   * @return The bytes for clientId.
   */
  com.google.protobuf.ByteString
      getClientIdBytes();

  /**
   * <pre>
   * identifies the connection end on the counterparty chain associated with a
   * given connection.
   * </pre>
   *
   * <code>string connection_id = 2 [json_name = "connectionId"];</code>
   * @return The connectionId.
   */
  java.lang.String getConnectionId();
  /**
   * <pre>
   * identifies the connection end on the counterparty chain associated with a
   * given connection.
   * </pre>
   *
   * <code>string connection_id = 2 [json_name = "connectionId"];</code>
   * @return The bytes for connectionId.
   */
  com.google.protobuf.ByteString
      getConnectionIdBytes();

  /**
   * <pre>
   * commitment merkle prefix of the counterparty chain.
   * </pre>
   *
   * <code>.icon.proto.core.commitment.MerklePrefix prefix = 3 [json_name = "prefix"];</code>
   * @return Whether the prefix field is set.
   */
  boolean hasPrefix();
  /**
   * <pre>
   * commitment merkle prefix of the counterparty chain.
   * </pre>
   *
   * <code>.icon.proto.core.commitment.MerklePrefix prefix = 3 [json_name = "prefix"];</code>
   * @return The prefix.
   */
  com.icon.proto.core.commitment.MerklePrefix getPrefix();
}