// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/crypto/proof.proto

package com.tendermint.crypto;

public interface ProofOrBuilder extends
    // @@protoc_insertion_point(interface_extends:tendermint.crypto.Proof)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>int64 total = 1 [json_name = "total"];</code>
   * @return The total.
   */
  long getTotal();

  /**
   * <code>int64 index = 2 [json_name = "index"];</code>
   * @return The index.
   */
  long getIndex();

  /**
   * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
   * @return The leafHash.
   */
  com.google.protobuf.ByteString getLeafHash();

  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @return A list containing the aunts.
   */
  java.util.List<com.google.protobuf.ByteString> getAuntsList();
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @return The count of aunts.
   */
  int getAuntsCount();
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @param index The index of the element to return.
   * @return The aunts at the given index.
   */
  com.google.protobuf.ByteString getAunts(int index);
}