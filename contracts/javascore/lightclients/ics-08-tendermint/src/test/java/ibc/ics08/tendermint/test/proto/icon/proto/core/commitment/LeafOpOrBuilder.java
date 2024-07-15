// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/23-commitment/proofs.proto

package com.icon.proto.core.commitment;

public interface LeafOpOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.commitment.LeafOp)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>.icon.proto.core.commitment.HashOp hash = 1 [json_name = "hash"];</code>
   * @return The enum numeric value on the wire for hash.
   */
  int getHashValue();
  /**
   * <code>.icon.proto.core.commitment.HashOp hash = 1 [json_name = "hash"];</code>
   * @return The hash.
   */
  com.icon.proto.core.commitment.HashOp getHash();

  /**
   * <code>.icon.proto.core.commitment.HashOp prehash_key = 2 [json_name = "prehashKey"];</code>
   * @return The enum numeric value on the wire for prehashKey.
   */
  int getPrehashKeyValue();
  /**
   * <code>.icon.proto.core.commitment.HashOp prehash_key = 2 [json_name = "prehashKey"];</code>
   * @return The prehashKey.
   */
  com.icon.proto.core.commitment.HashOp getPrehashKey();

  /**
   * <code>.icon.proto.core.commitment.HashOp prehash_value = 3 [json_name = "prehashValue"];</code>
   * @return The enum numeric value on the wire for prehashValue.
   */
  int getPrehashValueValue();
  /**
   * <code>.icon.proto.core.commitment.HashOp prehash_value = 3 [json_name = "prehashValue"];</code>
   * @return The prehashValue.
   */
  com.icon.proto.core.commitment.HashOp getPrehashValue();

  /**
   * <code>.icon.proto.core.commitment.LengthOp length = 4 [json_name = "length"];</code>
   * @return The enum numeric value on the wire for length.
   */
  int getLengthValue();
  /**
   * <code>.icon.proto.core.commitment.LengthOp length = 4 [json_name = "length"];</code>
   * @return The length.
   */
  com.icon.proto.core.commitment.LengthOp getLength();

  /**
   * <pre>
   * prefix is a fixed bytes that may optionally be included at the beginning to differentiate
   * a leaf node from an inner node.
   * </pre>
   *
   * <code>bytes prefix = 5 [json_name = "prefix"];</code>
   * @return The prefix.
   */
  com.google.protobuf.ByteString getPrefix();
}
