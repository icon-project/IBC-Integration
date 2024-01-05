// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: cosmos/ics23/v1/proofs.proto

package com.cosmos.ics23.v1;

/**
 * <pre>
 *NonExistenceProof takes a proof of two neighbors, one left of the desired key,
 *one right of the desired key. If both proofs are valid AND they are neighbors,
 *then there is no valid proof for the given key.
 * </pre>
 *
 * Protobuf type {@code cosmos.ics23.v1.NonExistenceProof}
 */
public  final class NonExistenceProof extends
    com.google.protobuf.GeneratedMessageLite<
        NonExistenceProof, NonExistenceProof.Builder> implements
    // @@protoc_insertion_point(message_implements:cosmos.ics23.v1.NonExistenceProof)
    NonExistenceProofOrBuilder {
  private NonExistenceProof() {
    key_ = com.google.protobuf.ByteString.EMPTY;
  }
  public static final int KEY_FIELD_NUMBER = 1;
  private com.google.protobuf.ByteString key_;
  /**
   * <pre>
   * TODO: remove this as unnecessary??? we prove a range
   * </pre>
   *
   * <code>bytes key = 1 [json_name = "key"];</code>
   * @return The key.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getKey() {
    return key_;
  }
  /**
   * <pre>
   * TODO: remove this as unnecessary??? we prove a range
   * </pre>
   *
   * <code>bytes key = 1 [json_name = "key"];</code>
   * @param value The key to set.
   */
  private void setKey(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    key_ = value;
  }
  /**
   * <pre>
   * TODO: remove this as unnecessary??? we prove a range
   * </pre>
   *
   * <code>bytes key = 1 [json_name = "key"];</code>
   */
  private void clearKey() {
    
    key_ = getDefaultInstance().getKey();
  }

  public static final int LEFT_FIELD_NUMBER = 2;
  private com.cosmos.ics23.v1.ExistenceProof left_;
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
   */
  @java.lang.Override
  public boolean hasLeft() {
    return left_ != null;
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
   */
  @java.lang.Override
  public com.cosmos.ics23.v1.ExistenceProof getLeft() {
    return left_ == null ? com.cosmos.ics23.v1.ExistenceProof.getDefaultInstance() : left_;
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
   */
  private void setLeft(com.cosmos.ics23.v1.ExistenceProof value) {
    value.getClass();
  left_ = value;
    
    }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergeLeft(com.cosmos.ics23.v1.ExistenceProof value) {
    value.getClass();
  if (left_ != null &&
        left_ != com.cosmos.ics23.v1.ExistenceProof.getDefaultInstance()) {
      left_ =
        com.cosmos.ics23.v1.ExistenceProof.newBuilder(left_).mergeFrom(value).buildPartial();
    } else {
      left_ = value;
    }
    
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
   */
  private void clearLeft() {  left_ = null;
    
  }

  public static final int RIGHT_FIELD_NUMBER = 3;
  private com.cosmos.ics23.v1.ExistenceProof right_;
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
   */
  @java.lang.Override
  public boolean hasRight() {
    return right_ != null;
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
   */
  @java.lang.Override
  public com.cosmos.ics23.v1.ExistenceProof getRight() {
    return right_ == null ? com.cosmos.ics23.v1.ExistenceProof.getDefaultInstance() : right_;
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
   */
  private void setRight(com.cosmos.ics23.v1.ExistenceProof value) {
    value.getClass();
  right_ = value;
    
    }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergeRight(com.cosmos.ics23.v1.ExistenceProof value) {
    value.getClass();
  if (right_ != null &&
        right_ != com.cosmos.ics23.v1.ExistenceProof.getDefaultInstance()) {
      right_ =
        com.cosmos.ics23.v1.ExistenceProof.newBuilder(right_).mergeFrom(value).buildPartial();
    } else {
      right_ = value;
    }
    
  }
  /**
   * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
   */
  private void clearRight() {  right_ = null;
    
  }

  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.cosmos.ics23.v1.NonExistenceProof parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.cosmos.ics23.v1.NonExistenceProof prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   *NonExistenceProof takes a proof of two neighbors, one left of the desired key,
   *one right of the desired key. If both proofs are valid AND they are neighbors,
   *then there is no valid proof for the given key.
   * </pre>
   *
   * Protobuf type {@code cosmos.ics23.v1.NonExistenceProof}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.cosmos.ics23.v1.NonExistenceProof, Builder> implements
      // @@protoc_insertion_point(builder_implements:cosmos.ics23.v1.NonExistenceProof)
      com.cosmos.ics23.v1.NonExistenceProofOrBuilder {
    // Construct using com.cosmos.ics23.v1.NonExistenceProof.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <pre>
     * TODO: remove this as unnecessary??? we prove a range
     * </pre>
     *
     * <code>bytes key = 1 [json_name = "key"];</code>
     * @return The key.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getKey() {
      return instance.getKey();
    }
    /**
     * <pre>
     * TODO: remove this as unnecessary??? we prove a range
     * </pre>
     *
     * <code>bytes key = 1 [json_name = "key"];</code>
     * @param value The key to set.
     * @return This builder for chaining.
     */
    public Builder setKey(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setKey(value);
      return this;
    }
    /**
     * <pre>
     * TODO: remove this as unnecessary??? we prove a range
     * </pre>
     *
     * <code>bytes key = 1 [json_name = "key"];</code>
     * @return This builder for chaining.
     */
    public Builder clearKey() {
      copyOnWrite();
      instance.clearKey();
      return this;
    }

    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    @java.lang.Override
    public boolean hasLeft() {
      return instance.hasLeft();
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    @java.lang.Override
    public com.cosmos.ics23.v1.ExistenceProof getLeft() {
      return instance.getLeft();
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    public Builder setLeft(com.cosmos.ics23.v1.ExistenceProof value) {
      copyOnWrite();
      instance.setLeft(value);
      return this;
      }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    public Builder setLeft(
        com.cosmos.ics23.v1.ExistenceProof.Builder builderForValue) {
      copyOnWrite();
      instance.setLeft(builderForValue.build());
      return this;
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    public Builder mergeLeft(com.cosmos.ics23.v1.ExistenceProof value) {
      copyOnWrite();
      instance.mergeLeft(value);
      return this;
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof left = 2 [json_name = "left"];</code>
     */
    public Builder clearLeft() {  copyOnWrite();
      instance.clearLeft();
      return this;
    }

    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    @java.lang.Override
    public boolean hasRight() {
      return instance.hasRight();
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    @java.lang.Override
    public com.cosmos.ics23.v1.ExistenceProof getRight() {
      return instance.getRight();
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    public Builder setRight(com.cosmos.ics23.v1.ExistenceProof value) {
      copyOnWrite();
      instance.setRight(value);
      return this;
      }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    public Builder setRight(
        com.cosmos.ics23.v1.ExistenceProof.Builder builderForValue) {
      copyOnWrite();
      instance.setRight(builderForValue.build());
      return this;
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    public Builder mergeRight(com.cosmos.ics23.v1.ExistenceProof value) {
      copyOnWrite();
      instance.mergeRight(value);
      return this;
    }
    /**
     * <code>.cosmos.ics23.v1.ExistenceProof right = 3 [json_name = "right"];</code>
     */
    public Builder clearRight() {  copyOnWrite();
      instance.clearRight();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:cosmos.ics23.v1.NonExistenceProof)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.cosmos.ics23.v1.NonExistenceProof();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "key_",
            "left_",
            "right_",
          };
          java.lang.String info =
              "\u0000\u0003\u0000\u0000\u0001\u0003\u0003\u0000\u0000\u0000\u0001\n\u0002\t\u0003" +
              "\t";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.cosmos.ics23.v1.NonExistenceProof> parser = PARSER;
        if (parser == null) {
          synchronized (com.cosmos.ics23.v1.NonExistenceProof.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.cosmos.ics23.v1.NonExistenceProof>(
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


  // @@protoc_insertion_point(class_scope:cosmos.ics23.v1.NonExistenceProof)
  private static final com.cosmos.ics23.v1.NonExistenceProof DEFAULT_INSTANCE;
  static {
    NonExistenceProof defaultInstance = new NonExistenceProof();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      NonExistenceProof.class, defaultInstance);
  }

  public static com.cosmos.ics23.v1.NonExistenceProof getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<NonExistenceProof> PARSER;

  public static com.google.protobuf.Parser<NonExistenceProof> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}

