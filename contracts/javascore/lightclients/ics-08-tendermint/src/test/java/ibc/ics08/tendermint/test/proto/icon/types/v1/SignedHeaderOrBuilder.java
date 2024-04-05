// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: icon/types/v1/types.proto

package com.icon.types.v1;

public interface SignedHeaderOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.types.v1.SignedHeader)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <code>.icon.types.v1.BTPHeader header = 1 [json_name = "header"];</code>
   * @return Whether the header field is set.
   */
  boolean hasHeader();
  /**
   * <code>.icon.types.v1.BTPHeader header = 1 [json_name = "header"];</code>
   * @return The header.
   */
  com.icon.types.v1.BTPHeader getHeader();

  /**
   * <code>repeated bytes signatures = 2 [json_name = "signatures"];</code>
   * @return A list containing the signatures.
   */
  java.util.List<com.google.protobuf.ByteString> getSignaturesList();
  /**
   * <code>repeated bytes signatures = 2 [json_name = "signatures"];</code>
   * @return The count of signatures.
   */
  int getSignaturesCount();
  /**
   * <code>repeated bytes signatures = 2 [json_name = "signatures"];</code>
   * @param index The index of the element to return.
   * @return The signatures at the given index.
   */
  com.google.protobuf.ByteString getSignatures(int index);

  /**
   * <code>repeated bytes currentValidators = 3 [json_name = "currentValidators"];</code>
   * @return A list containing the currentValidators.
   */
  java.util.List<com.google.protobuf.ByteString> getCurrentValidatorsList();
  /**
   * <code>repeated bytes currentValidators = 3 [json_name = "currentValidators"];</code>
   * @return The count of currentValidators.
   */
  int getCurrentValidatorsCount();
  /**
   * <code>repeated bytes currentValidators = 3 [json_name = "currentValidators"];</code>
   * @param index The index of the element to return.
   * @return The currentValidators at the given index.
   */
  com.google.protobuf.ByteString getCurrentValidators(int index);

  /**
   * <code>uint64 trusted_height = 4 [json_name = "trustedHeight"];</code>
   * @return The trustedHeight.
   */
  long getTrustedHeight();
}