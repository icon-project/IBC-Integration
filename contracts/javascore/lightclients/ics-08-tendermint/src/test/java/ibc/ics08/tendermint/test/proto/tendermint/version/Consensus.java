// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/version/types.proto

package com.tendermint.version;

/**
 * <pre>
 * Consensus captures the consensus rules for processing a block in the blockchain,
 * including all blockchain data structures and the rules of the application's
 * state transition machine.
 * </pre>
 *
 * Protobuf type {@code tendermint.version.Consensus}
 */
public  final class Consensus extends
    com.google.protobuf.GeneratedMessageLite<
        Consensus, Consensus.Builder> implements
    // @@protoc_insertion_point(message_implements:tendermint.version.Consensus)
    ConsensusOrBuilder {
  private Consensus() {
  }
  public static final int BLOCK_FIELD_NUMBER = 1;
  private long block_;
  /**
   * <code>uint64 block = 1 [json_name = "block"];</code>
   * @return The block.
   */
  @java.lang.Override
  public long getBlock() {
    return block_;
  }
  /**
   * <code>uint64 block = 1 [json_name = "block"];</code>
   * @param value The block to set.
   */
  private void setBlock(long value) {
    
    block_ = value;
  }
  /**
   * <code>uint64 block = 1 [json_name = "block"];</code>
   */
  private void clearBlock() {
    
    block_ = 0L;
  }

  public static final int APP_FIELD_NUMBER = 2;
  private long app_;
  /**
   * <code>uint64 app = 2 [json_name = "app"];</code>
   * @return The app.
   */
  @java.lang.Override
  public long getApp() {
    return app_;
  }
  /**
   * <code>uint64 app = 2 [json_name = "app"];</code>
   * @param value The app to set.
   */
  private void setApp(long value) {
    
    app_ = value;
  }
  /**
   * <code>uint64 app = 2 [json_name = "app"];</code>
   */
  private void clearApp() {
    
    app_ = 0L;
  }

  public static com.tendermint.version.Consensus parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.version.Consensus parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.version.Consensus parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.version.Consensus parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.version.Consensus parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.version.Consensus parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.version.Consensus parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.version.Consensus parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.version.Consensus parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.version.Consensus parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.version.Consensus parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.version.Consensus parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.tendermint.version.Consensus prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   * Consensus captures the consensus rules for processing a block in the blockchain,
   * including all blockchain data structures and the rules of the application's
   * state transition machine.
   * </pre>
   *
   * Protobuf type {@code tendermint.version.Consensus}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.tendermint.version.Consensus, Builder> implements
      // @@protoc_insertion_point(builder_implements:tendermint.version.Consensus)
      com.tendermint.version.ConsensusOrBuilder {
    // Construct using com.tendermint.version.Consensus.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <code>uint64 block = 1 [json_name = "block"];</code>
     * @return The block.
     */
    @java.lang.Override
    public long getBlock() {
      return instance.getBlock();
    }
    /**
     * <code>uint64 block = 1 [json_name = "block"];</code>
     * @param value The block to set.
     * @return This builder for chaining.
     */
    public Builder setBlock(long value) {
      copyOnWrite();
      instance.setBlock(value);
      return this;
    }
    /**
     * <code>uint64 block = 1 [json_name = "block"];</code>
     * @return This builder for chaining.
     */
    public Builder clearBlock() {
      copyOnWrite();
      instance.clearBlock();
      return this;
    }

    /**
     * <code>uint64 app = 2 [json_name = "app"];</code>
     * @return The app.
     */
    @java.lang.Override
    public long getApp() {
      return instance.getApp();
    }
    /**
     * <code>uint64 app = 2 [json_name = "app"];</code>
     * @param value The app to set.
     * @return This builder for chaining.
     */
    public Builder setApp(long value) {
      copyOnWrite();
      instance.setApp(value);
      return this;
    }
    /**
     * <code>uint64 app = 2 [json_name = "app"];</code>
     * @return This builder for chaining.
     */
    public Builder clearApp() {
      copyOnWrite();
      instance.clearApp();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:tendermint.version.Consensus)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.tendermint.version.Consensus();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "block_",
            "app_",
          };
          java.lang.String info =
              "\u0000\u0002\u0000\u0000\u0001\u0002\u0002\u0000\u0000\u0000\u0001\u0003\u0002\u0003" +
              "";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.tendermint.version.Consensus> parser = PARSER;
        if (parser == null) {
          synchronized (com.tendermint.version.Consensus.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.tendermint.version.Consensus>(
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


  // @@protoc_insertion_point(class_scope:tendermint.version.Consensus)
  private static final com.tendermint.version.Consensus DEFAULT_INSTANCE;
  static {
    Consensus defaultInstance = new Consensus();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      Consensus.class, defaultInstance);
  }

  public static com.tendermint.version.Consensus getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<Consensus> PARSER;

  public static com.google.protobuf.Parser<Consensus> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}

