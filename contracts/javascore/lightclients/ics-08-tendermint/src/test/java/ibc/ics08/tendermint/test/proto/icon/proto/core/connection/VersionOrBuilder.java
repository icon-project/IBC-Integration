// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/03-connection/Connection.proto

package com.icon.proto.core.connection;

public interface VersionOrBuilder extends
    // @@protoc_insertion_point(interface_extends:icon.proto.core.connection.Version)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * unique version identifier
   * </pre>
   *
   * <code>string identifier = 1 [json_name = "identifier"];</code>
   * @return The identifier.
   */
  java.lang.String getIdentifier();
  /**
   * <pre>
   * unique version identifier
   * </pre>
   *
   * <code>string identifier = 1 [json_name = "identifier"];</code>
   * @return The bytes for identifier.
   */
  com.google.protobuf.ByteString
      getIdentifierBytes();

  /**
   * <pre>
   * list of features compatible with the specified identifier
   * </pre>
   *
   * <code>repeated string features = 2 [json_name = "features"];</code>
   * @return A list containing the features.
   */
  java.util.List<java.lang.String>
      getFeaturesList();
  /**
   * <pre>
   * list of features compatible with the specified identifier
   * </pre>
   *
   * <code>repeated string features = 2 [json_name = "features"];</code>
   * @return The count of features.
   */
  int getFeaturesCount();
  /**
   * <pre>
   * list of features compatible with the specified identifier
   * </pre>
   *
   * <code>repeated string features = 2 [json_name = "features"];</code>
   * @param index The index of the element to return.
   * @return The features at the given index.
   */
  java.lang.String getFeatures(int index);
  /**
   * <pre>
   * list of features compatible with the specified identifier
   * </pre>
   *
   * <code>repeated string features = 2 [json_name = "features"];</code>
   * @param index The index of the element to return.
   * @return The features at the given index.
   */
  com.google.protobuf.ByteString
      getFeaturesBytes(int index);
}
