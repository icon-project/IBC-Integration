// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/types/types.proto

package com.tendermint.types;

public interface TxProofOrBuilder extends
    // @@protoc_insertion_point(interface_extends:tendermint.types.TxProof)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>bytes root_hash = 1 [json_name = "rootHash"];</code>
   * @return The rootHash.
   */
  com.google.protobuf.ByteString getRootHash();

  /**
   * <code>bytes data = 2 [json_name = "data"];</code>
   * @return The data.
   */
  com.google.protobuf.ByteString getData();

  /**
   * <code>.tendermint.crypto.Proof proof = 3 [json_name = "proof"];</code>
   * @return Whether the proof field is set.
   */
  boolean hasProof();
  /**
   * <code>.tendermint.crypto.Proof proof = 3 [json_name = "proof"];</code>
   * @return The proof.
   */
  com.tendermint.crypto.Proof getProof();
}
