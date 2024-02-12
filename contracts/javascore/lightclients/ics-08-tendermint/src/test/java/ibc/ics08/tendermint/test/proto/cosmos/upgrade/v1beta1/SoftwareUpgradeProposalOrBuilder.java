// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: cosmos/upgrade/v1beta1/upgrade.proto

package com.cosmos.upgrade.v1beta1;

@java.lang.Deprecated public interface SoftwareUpgradeProposalOrBuilder extends
    // @@protoc_insertion_point(interface_extends:cosmos.upgrade.v1beta1.SoftwareUpgradeProposal)
    com.google.protobuf.MessageLiteOrBuilder {

  /**
   * <pre>
   * title of the proposal
   * </pre>
   *
   * <code>string title = 1 [json_name = "title"];</code>
   * @return The title.
   */
  java.lang.String getTitle();
  /**
   * <pre>
   * title of the proposal
   * </pre>
   *
   * <code>string title = 1 [json_name = "title"];</code>
   * @return The bytes for title.
   */
  com.google.protobuf.ByteString
      getTitleBytes();

  /**
   * <pre>
   * description of the proposal
   * </pre>
   *
   * <code>string description = 2 [json_name = "description"];</code>
   * @return The description.
   */
  java.lang.String getDescription();
  /**
   * <pre>
   * description of the proposal
   * </pre>
   *
   * <code>string description = 2 [json_name = "description"];</code>
   * @return The bytes for description.
   */
  com.google.protobuf.ByteString
      getDescriptionBytes();

  /**
   * <pre>
   * plan of the proposal
   * </pre>
   *
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false, (.amino.dont_omitempty) = true];</code>
   * @return Whether the plan field is set.
   */
  boolean hasPlan();
  /**
   * <pre>
   * plan of the proposal
   * </pre>
   *
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false, (.amino.dont_omitempty) = true];</code>
   * @return The plan.
   */
  com.cosmos.upgrade.v1beta1.Plan getPlan();
}