// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/23-commitment/commitment.proto

package com.icon.proto.core.commitment;

public interface MerklePathOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.commitment.MerklePath)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>repeated string key_path = 1 [json_name = "keyPath"];</code>
   * @return A list containing the keyPath.
   */
  java.util.List<java.lang.String>
      getKeyPathList();
  /**
   * <code>repeated string key_path = 1 [json_name = "keyPath"];</code>
   * @return The count of keyPath.
   */
  int getKeyPathCount();
  /**
   * <code>repeated string key_path = 1 [json_name = "keyPath"];</code>
   * @param index The index of the element to return.
   * @return The keyPath at the given index.
   */
  java.lang.String getKeyPath(int index);
  /**
   * <code>repeated string key_path = 1 [json_name = "keyPath"];</code>
   * @param index The index of the element to return.
   * @return The keyPath at the given index.
   */
  com.google.protobuf.ByteString
      getKeyPathBytes(int index);
}