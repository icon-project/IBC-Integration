// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: clients/tendermint/TendermintLight.proto

package com.ibc.lightclients.tendermint.v1;

public interface MisbehaviourOrBuilder extends
    // @@protoc_insertion_point(interface_extends:ibc.lightclients.tendermint.v1.Misbehaviour)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * ClientID is deprecated
   * </pre>
   *
   * <code>string client_id = 1 [json_name = "clientId", deprecated = true];</code>
   * @deprecated ibc.lightclients.tendermint.v1.Misbehaviour.client_id is deprecated.
   *     See clients/tendermint/TendermintLight.proto;l=70
   * @return The clientId.
   */
  @java.lang.Deprecated java.lang.String getClientId();
  /**
   * <pre>
   * ClientID is deprecated
   * </pre>
   *
   * <code>string client_id = 1 [json_name = "clientId", deprecated = true];</code>
   * @deprecated ibc.lightclients.tendermint.v1.Misbehaviour.client_id is deprecated.
   *     See clients/tendermint/TendermintLight.proto;l=70
   * @return The bytes for clientId.
   */
  @java.lang.Deprecated com.google.protobuf.ByteString
      getClientIdBytes();

  /**
   * <code>.ibc.lightclients.tendermint.v1.Header header_1 = 2 [json_name = "header1", (.gogoproto.customname) = "Header1"];</code>
   * @return Whether the header1 field is set.
   */
  boolean hasHeader1();
  /**
   * <code>.ibc.lightclients.tendermint.v1.Header header_1 = 2 [json_name = "header1", (.gogoproto.customname) = "Header1"];</code>
   * @return The header1.
   */
  com.ibc.lightclients.tendermint.v1.Header getHeader1();

  /**
   * <code>.ibc.lightclients.tendermint.v1.Header header_2 = 3 [json_name = "header2", (.gogoproto.customname) = "Header2"];</code>
   * @return Whether the header2 field is set.
   */
  boolean hasHeader2();
  /**
   * <code>.ibc.lightclients.tendermint.v1.Header header_2 = 3 [json_name = "header2", (.gogoproto.customname) = "Header2"];</code>
   * @return The header2.
   */
  com.ibc.lightclients.tendermint.v1.Header getHeader2();
}