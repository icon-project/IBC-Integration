// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/types/types.proto

package com.tendermint.types;

public interface ProposalOrBuilder extends
    // @@protoc_insertion_point(interface_extends:tendermint.types.Proposal)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>.tendermint.types.SignedMsgType type = 1 [json_name = "type"];</code>
   * @return The enum numeric value on the wire for type.
   */
  int getTypeValue();
  /**
   * <code>.tendermint.types.SignedMsgType type = 1 [json_name = "type"];</code>
   * @return The type.
   */
  com.tendermint.types.SignedMsgType getType();

  /**
   * <code>int64 height = 2 [json_name = "height"];</code>
   * @return The height.
   */
  long getHeight();

  /**
   * <code>int32 round = 3 [json_name = "round"];</code>
   * @return The round.
   */
  int getRound();

  /**
   * <code>int32 pol_round = 4 [json_name = "polRound"];</code>
   * @return The polRound.
   */
  int getPolRound();

  /**
   * <code>.tendermint.types.BlockID block_id = 5 [json_name = "blockId", (.gogoproto.nullable) = false, (.gogoproto.customname) = "BlockID"];</code>
   * @return Whether the blockId field is set.
   */
  boolean hasBlockId();
  /**
   * <code>.tendermint.types.BlockID block_id = 5 [json_name = "blockId", (.gogoproto.nullable) = false, (.gogoproto.customname) = "BlockID"];</code>
   * @return The blockId.
   */
  com.tendermint.types.BlockID getBlockId();

  /**
   * <code>.google.protobuf.Timestamp timestamp = 6 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   * @return Whether the timestamp field is set.
   */
  boolean hasTimestamp();
  /**
   * <code>.google.protobuf.Timestamp timestamp = 6 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   * @return The timestamp.
   */
  com.google.protobuf.Timestamp getTimestamp();

  /**
   * <code>bytes signature = 7 [json_name = "signature"];</code>
   * @return The signature.
   */
  com.google.protobuf.ByteString getSignature();
}
