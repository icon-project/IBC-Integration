// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: ibc/core/client/v1/client.proto

package com.ibc.core.client.v1;

/**
 * <pre>
 * UpgradeProposal is a gov Content type for initiating an IBC breaking
 * upgrade.
 * </pre>
 *
 * Protobuf type {@code ibc.core.client.v1.UpgradeProposal}
 */
public  final class UpgradeProposal extends
    com.google.protobuf.GeneratedMessageLite<
        UpgradeProposal, UpgradeProposal.Builder> implements
    // @@protoc_insertion_point(message_implements:ibc.core.client.v1.UpgradeProposal)
    UpgradeProposalOrBuilder {
  private UpgradeProposal() {
    title_ = "";
    description_ = "";
  }
  public static final int TITLE_FIELD_NUMBER = 1;
  private java.lang.String title_;
  /**
   * <code>string title = 1 [json_name = "title"];</code>
   * @return The title.
   */
  @java.lang.Override
  public java.lang.String getTitle() {
    return title_;
  }
  /**
   * <code>string title = 1 [json_name = "title"];</code>
   * @return The bytes for title.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getTitleBytes() {
    return com.google.protobuf.ByteString.copyFromUtf8(title_);
  }
  /**
   * <code>string title = 1 [json_name = "title"];</code>
   * @param value The title to set.
   */
  private void setTitle(
      java.lang.String value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    title_ = value;
  }
  /**
   * <code>string title = 1 [json_name = "title"];</code>
   */
  private void clearTitle() {
    
    title_ = getDefaultInstance().getTitle();
  }
  /**
   * <code>string title = 1 [json_name = "title"];</code>
   * @param value The bytes for title to set.
   */
  private void setTitleBytes(
      com.google.protobuf.ByteString value) {
    checkByteStringIsUtf8(value);
    title_ = value.toStringUtf8();
    
  }

  public static final int DESCRIPTION_FIELD_NUMBER = 2;
  private java.lang.String description_;
  /**
   * <code>string description = 2 [json_name = "description"];</code>
   * @return The description.
   */
  @java.lang.Override
  public java.lang.String getDescription() {
    return description_;
  }
  /**
   * <code>string description = 2 [json_name = "description"];</code>
   * @return The bytes for description.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString
      getDescriptionBytes() {
    return com.google.protobuf.ByteString.copyFromUtf8(description_);
  }
  /**
   * <code>string description = 2 [json_name = "description"];</code>
   * @param value The description to set.
   */
  private void setDescription(
      java.lang.String value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    description_ = value;
  }
  /**
   * <code>string description = 2 [json_name = "description"];</code>
   */
  private void clearDescription() {
    
    description_ = getDefaultInstance().getDescription();
  }
  /**
   * <code>string description = 2 [json_name = "description"];</code>
   * @param value The bytes for description to set.
   */
  private void setDescriptionBytes(
      com.google.protobuf.ByteString value) {
    checkByteStringIsUtf8(value);
    description_ = value.toStringUtf8();
    
  }

  public static final int PLAN_FIELD_NUMBER = 3;
  private com.cosmos.upgrade.v1beta1.Plan plan_;
  /**
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
   */
  @java.lang.Override
  public boolean hasPlan() {
    return plan_ != null;
  }
  /**
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
   */
  @java.lang.Override
  public com.cosmos.upgrade.v1beta1.Plan getPlan() {
    return plan_ == null ? com.cosmos.upgrade.v1beta1.Plan.getDefaultInstance() : plan_;
  }
  /**
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
   */
  private void setPlan(com.cosmos.upgrade.v1beta1.Plan value) {
    value.getClass();
  plan_ = value;
    
    }
  /**
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergePlan(com.cosmos.upgrade.v1beta1.Plan value) {
    value.getClass();
  if (plan_ != null &&
        plan_ != com.cosmos.upgrade.v1beta1.Plan.getDefaultInstance()) {
      plan_ =
        com.cosmos.upgrade.v1beta1.Plan.newBuilder(plan_).mergeFrom(value).buildPartial();
    } else {
      plan_ = value;
    }
    
  }
  /**
   * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
   */
  private void clearPlan() {  plan_ = null;
    
  }

  public static final int UPGRADED_CLIENT_STATE_FIELD_NUMBER = 4;
  private com.google.protobuf.Any upgradedClientState_;
  /**
   * <pre>
   * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
   * This will make the chain commit to the correct upgraded (self) client state
   * before the upgrade occurs, so that connecting chains can verify that the
   * new upgraded client is valid by verifying a proof on the previous version
   * of the chain. This will allow IBC connections to persist smoothly across
   * planned chain upgrades
   * </pre>
   *
   * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
   */
  @java.lang.Override
  public boolean hasUpgradedClientState() {
    return upgradedClientState_ != null;
  }
  /**
   * <pre>
   * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
   * This will make the chain commit to the correct upgraded (self) client state
   * before the upgrade occurs, so that connecting chains can verify that the
   * new upgraded client is valid by verifying a proof on the previous version
   * of the chain. This will allow IBC connections to persist smoothly across
   * planned chain upgrades
   * </pre>
   *
   * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
   */
  @java.lang.Override
  public com.google.protobuf.Any getUpgradedClientState() {
    return upgradedClientState_ == null ? com.google.protobuf.Any.getDefaultInstance() : upgradedClientState_;
  }
  /**
   * <pre>
   * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
   * This will make the chain commit to the correct upgraded (self) client state
   * before the upgrade occurs, so that connecting chains can verify that the
   * new upgraded client is valid by verifying a proof on the previous version
   * of the chain. This will allow IBC connections to persist smoothly across
   * planned chain upgrades
   * </pre>
   *
   * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
   */
  private void setUpgradedClientState(com.google.protobuf.Any value) {
    value.getClass();
  upgradedClientState_ = value;
    
    }
  /**
   * <pre>
   * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
   * This will make the chain commit to the correct upgraded (self) client state
   * before the upgrade occurs, so that connecting chains can verify that the
   * new upgraded client is valid by verifying a proof on the previous version
   * of the chain. This will allow IBC connections to persist smoothly across
   * planned chain upgrades
   * </pre>
   *
   * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergeUpgradedClientState(com.google.protobuf.Any value) {
    value.getClass();
  if (upgradedClientState_ != null &&
        upgradedClientState_ != com.google.protobuf.Any.getDefaultInstance()) {
      upgradedClientState_ =
        com.google.protobuf.Any.newBuilder(upgradedClientState_).mergeFrom(value).buildPartial();
    } else {
      upgradedClientState_ = value;
    }
    
  }
  /**
   * <pre>
   * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
   * This will make the chain commit to the correct upgraded (self) client state
   * before the upgrade occurs, so that connecting chains can verify that the
   * new upgraded client is valid by verifying a proof on the previous version
   * of the chain. This will allow IBC connections to persist smoothly across
   * planned chain upgrades
   * </pre>
   *
   * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
   */
  private void clearUpgradedClientState() {  upgradedClientState_ = null;
    
  }

  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.ibc.core.client.v1.UpgradeProposal parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.ibc.core.client.v1.UpgradeProposal prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   * UpgradeProposal is a gov Content type for initiating an IBC breaking
   * upgrade.
   * </pre>
   *
   * Protobuf type {@code ibc.core.client.v1.UpgradeProposal}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.ibc.core.client.v1.UpgradeProposal, Builder> implements
      // @@protoc_insertion_point(builder_implements:ibc.core.client.v1.UpgradeProposal)
      com.ibc.core.client.v1.UpgradeProposalOrBuilder {
    // Construct using com.ibc.core.client.v1.UpgradeProposal.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <code>string title = 1 [json_name = "title"];</code>
     * @return The title.
     */
    @java.lang.Override
    public java.lang.String getTitle() {
      return instance.getTitle();
    }
    /**
     * <code>string title = 1 [json_name = "title"];</code>
     * @return The bytes for title.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString
        getTitleBytes() {
      return instance.getTitleBytes();
    }
    /**
     * <code>string title = 1 [json_name = "title"];</code>
     * @param value The title to set.
     * @return This builder for chaining.
     */
    public Builder setTitle(
        java.lang.String value) {
      copyOnWrite();
      instance.setTitle(value);
      return this;
    }
    /**
     * <code>string title = 1 [json_name = "title"];</code>
     * @return This builder for chaining.
     */
    public Builder clearTitle() {
      copyOnWrite();
      instance.clearTitle();
      return this;
    }
    /**
     * <code>string title = 1 [json_name = "title"];</code>
     * @param value The bytes for title to set.
     * @return This builder for chaining.
     */
    public Builder setTitleBytes(
        com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setTitleBytes(value);
      return this;
    }

    /**
     * <code>string description = 2 [json_name = "description"];</code>
     * @return The description.
     */
    @java.lang.Override
    public java.lang.String getDescription() {
      return instance.getDescription();
    }
    /**
     * <code>string description = 2 [json_name = "description"];</code>
     * @return The bytes for description.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString
        getDescriptionBytes() {
      return instance.getDescriptionBytes();
    }
    /**
     * <code>string description = 2 [json_name = "description"];</code>
     * @param value The description to set.
     * @return This builder for chaining.
     */
    public Builder setDescription(
        java.lang.String value) {
      copyOnWrite();
      instance.setDescription(value);
      return this;
    }
    /**
     * <code>string description = 2 [json_name = "description"];</code>
     * @return This builder for chaining.
     */
    public Builder clearDescription() {
      copyOnWrite();
      instance.clearDescription();
      return this;
    }
    /**
     * <code>string description = 2 [json_name = "description"];</code>
     * @param value The bytes for description to set.
     * @return This builder for chaining.
     */
    public Builder setDescriptionBytes(
        com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setDescriptionBytes(value);
      return this;
    }

    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    @java.lang.Override
    public boolean hasPlan() {
      return instance.hasPlan();
    }
    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    @java.lang.Override
    public com.cosmos.upgrade.v1beta1.Plan getPlan() {
      return instance.getPlan();
    }
    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    public Builder setPlan(com.cosmos.upgrade.v1beta1.Plan value) {
      copyOnWrite();
      instance.setPlan(value);
      return this;
      }
    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    public Builder setPlan(
        com.cosmos.upgrade.v1beta1.Plan.Builder builderForValue) {
      copyOnWrite();
      instance.setPlan(builderForValue.build());
      return this;
    }
    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    public Builder mergePlan(com.cosmos.upgrade.v1beta1.Plan value) {
      copyOnWrite();
      instance.mergePlan(value);
      return this;
    }
    /**
     * <code>.cosmos.upgrade.v1beta1.Plan plan = 3 [json_name = "plan", (.gogoproto.nullable) = false];</code>
     */
    public Builder clearPlan() {  copyOnWrite();
      instance.clearPlan();
      return this;
    }

    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    @java.lang.Override
    public boolean hasUpgradedClientState() {
      return instance.hasUpgradedClientState();
    }
    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    @java.lang.Override
    public com.google.protobuf.Any getUpgradedClientState() {
      return instance.getUpgradedClientState();
    }
    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    public Builder setUpgradedClientState(com.google.protobuf.Any value) {
      copyOnWrite();
      instance.setUpgradedClientState(value);
      return this;
      }
    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    public Builder setUpgradedClientState(
        com.google.protobuf.Any.Builder builderForValue) {
      copyOnWrite();
      instance.setUpgradedClientState(builderForValue.build());
      return this;
    }
    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    public Builder mergeUpgradedClientState(com.google.protobuf.Any value) {
      copyOnWrite();
      instance.mergeUpgradedClientState(value);
      return this;
    }
    /**
     * <pre>
     * An UpgradedClientState must be provided to perform an IBC breaking upgrade.
     * This will make the chain commit to the correct upgraded (self) client state
     * before the upgrade occurs, so that connecting chains can verify that the
     * new upgraded client is valid by verifying a proof on the previous version
     * of the chain. This will allow IBC connections to persist smoothly across
     * planned chain upgrades
     * </pre>
     *
     * <code>.google.protobuf.Any upgraded_client_state = 4 [json_name = "upgradedClientState"];</code>
     */
    public Builder clearUpgradedClientState() {  copyOnWrite();
      instance.clearUpgradedClientState();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:ibc.core.client.v1.UpgradeProposal)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.ibc.core.client.v1.UpgradeProposal();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "title_",
            "description_",
            "plan_",
            "upgradedClientState_",
          };
          java.lang.String info =
              "\u0000\u0004\u0000\u0000\u0001\u0004\u0004\u0000\u0000\u0000\u0001\u0208\u0002\u0208" +
              "\u0003\t\u0004\t";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.ibc.core.client.v1.UpgradeProposal> parser = PARSER;
        if (parser == null) {
          synchronized (com.ibc.core.client.v1.UpgradeProposal.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.ibc.core.client.v1.UpgradeProposal>(
                      DEFAULT_INSTANCE);
              PARSER = parser;
            }
          }
        }
        return parser;
    }
    case GET_MEMOIZED_IS_INITIALIZED: {
      return (byte) 1;
    }
    case SET_MEMOIZED_IS_INITIALIZED: {
      return null;
    }
    }
    throw new UnsupportedOperationException();
  }


  // @@protoc_insertion_point(class_scope:ibc.core.client.v1.UpgradeProposal)
  private static final com.ibc.core.client.v1.UpgradeProposal DEFAULT_INSTANCE;
  static {
    UpgradeProposal defaultInstance = new UpgradeProposal();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      UpgradeProposal.class, defaultInstance);
  }

  public static com.ibc.core.client.v1.UpgradeProposal getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<UpgradeProposal> PARSER;

  public static com.google.protobuf.Parser<UpgradeProposal> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}
