// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/types/types.proto

package com.tendermint.types;

/**
 * <pre>
 * CommitSig is a part of the Vote included in a Commit.
 * </pre>
 *
 * Protobuf type {@code tendermint.types.CommitSig}
 */
public  final class CommitSig extends
    com.google.protobuf.GeneratedMessageLite<
        CommitSig, CommitSig.Builder> implements
    // @@protoc_insertion_point(message_implements:tendermint.types.CommitSig)
    CommitSigOrBuilder {
  private CommitSig() {
    validatorAddress_ = com.google.protobuf.ByteString.EMPTY;
    signature_ = com.google.protobuf.ByteString.EMPTY;
  }
  public static final int BLOCK_ID_FLAG_FIELD_NUMBER = 1;
  private int blockIdFlag_;
  /**
   * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
   * @return The enum numeric value on the wire for blockIdFlag.
   */
  @java.lang.Override
  public int getBlockIdFlagValue() {
    return blockIdFlag_;
  }
  /**
   * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
   * @return The blockIdFlag.
   */
  @java.lang.Override
  public com.tendermint.types.BlockIDFlag getBlockIdFlag() {
    com.tendermint.types.BlockIDFlag result = com.tendermint.types.BlockIDFlag.forNumber(blockIdFlag_);
    return result == null ? com.tendermint.types.BlockIDFlag.UNRECOGNIZED : result;
  }
  /**
   * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
   * @param value The enum numeric value on the wire for blockIdFlag to set.
   */
  private void setBlockIdFlagValue(int value) {
      blockIdFlag_ = value;
  }
  /**
   * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
   * @param value The blockIdFlag to set.
   */
  private void setBlockIdFlag(com.tendermint.types.BlockIDFlag value) {
    blockIdFlag_ = value.getNumber();
    
  }
  /**
   * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
   */
  private void clearBlockIdFlag() {
    
    blockIdFlag_ = 0;
  }

  public static final int VALIDATOR_ADDRESS_FIELD_NUMBER = 2;
  private com.google.protobuf.ByteString validatorAddress_;
  /**
   * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
   * @return The validatorAddress.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getValidatorAddress() {
    return validatorAddress_;
  }
  /**
   * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
   * @param value The validatorAddress to set.
   */
  private void setValidatorAddress(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    validatorAddress_ = value;
  }
  /**
   * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
   */
  private void clearValidatorAddress() {
    
    validatorAddress_ = getDefaultInstance().getValidatorAddress();
  }

  public static final int TIMESTAMP_FIELD_NUMBER = 3;
  private com.google.protobuf.Timestamp timestamp_;
  /**
   * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   */
  @java.lang.Override
  public boolean hasTimestamp() {
    return timestamp_ != null;
  }
  /**
   * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   */
  @java.lang.Override
  public com.google.protobuf.Timestamp getTimestamp() {
    return timestamp_ == null ? com.google.protobuf.Timestamp.getDefaultInstance() : timestamp_;
  }
  /**
   * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   */
  private void setTimestamp(com.google.protobuf.Timestamp value) {
    value.getClass();
  timestamp_ = value;
    
    }
  /**
   * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergeTimestamp(com.google.protobuf.Timestamp value) {
    value.getClass();
  if (timestamp_ != null &&
        timestamp_ != com.google.protobuf.Timestamp.getDefaultInstance()) {
      timestamp_ =
        com.google.protobuf.Timestamp.newBuilder(timestamp_).mergeFrom(value).buildPartial();
    } else {
      timestamp_ = value;
    }
    
  }
  /**
   * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
   */
  private void clearTimestamp() {  timestamp_ = null;
    
  }

  public static final int SIGNATURE_FIELD_NUMBER = 4;
  private com.google.protobuf.ByteString signature_;
  /**
   * <code>bytes signature = 4 [json_name = "signature"];</code>
   * @return The signature.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getSignature() {
    return signature_;
  }
  /**
   * <code>bytes signature = 4 [json_name = "signature"];</code>
   * @param value The signature to set.
   */
  private void setSignature(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    signature_ = value;
  }
  /**
   * <code>bytes signature = 4 [json_name = "signature"];</code>
   */
  private void clearSignature() {
    
    signature_ = getDefaultInstance().getSignature();
  }

  public static com.tendermint.types.CommitSig parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.types.CommitSig parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.types.CommitSig parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.types.CommitSig parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.types.CommitSig parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.types.CommitSig parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.tendermint.types.CommitSig prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   * CommitSig is a part of the Vote included in a Commit.
   * </pre>
   *
   * Protobuf type {@code tendermint.types.CommitSig}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.tendermint.types.CommitSig, Builder> implements
      // @@protoc_insertion_point(builder_implements:tendermint.types.CommitSig)
      com.tendermint.types.CommitSigOrBuilder {
    // Construct using com.tendermint.types.CommitSig.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
     * @return The enum numeric value on the wire for blockIdFlag.
     */
    @java.lang.Override
    public int getBlockIdFlagValue() {
      return instance.getBlockIdFlagValue();
    }
    /**
     * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
     * @param value The blockIdFlag to set.
     * @return This builder for chaining.
     */
    public Builder setBlockIdFlagValue(int value) {
      copyOnWrite();
      instance.setBlockIdFlagValue(value);
      return this;
    }
    /**
     * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
     * @return The blockIdFlag.
     */
    @java.lang.Override
    public com.tendermint.types.BlockIDFlag getBlockIdFlag() {
      return instance.getBlockIdFlag();
    }
    /**
     * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
     * @param value The enum numeric value on the wire for blockIdFlag to set.
     * @return This builder for chaining.
     */
    public Builder setBlockIdFlag(com.tendermint.types.BlockIDFlag value) {
      copyOnWrite();
      instance.setBlockIdFlag(value);
      return this;
    }
    /**
     * <code>.tendermint.types.BlockIDFlag block_id_flag = 1 [json_name = "blockIdFlag"];</code>
     * @return This builder for chaining.
     */
    public Builder clearBlockIdFlag() {
      copyOnWrite();
      instance.clearBlockIdFlag();
      return this;
    }

    /**
     * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
     * @return The validatorAddress.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getValidatorAddress() {
      return instance.getValidatorAddress();
    }
    /**
     * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
     * @param value The validatorAddress to set.
     * @return This builder for chaining.
     */
    public Builder setValidatorAddress(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setValidatorAddress(value);
      return this;
    }
    /**
     * <code>bytes validator_address = 2 [json_name = "validatorAddress"];</code>
     * @return This builder for chaining.
     */
    public Builder clearValidatorAddress() {
      copyOnWrite();
      instance.clearValidatorAddress();
      return this;
    }

    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    @java.lang.Override
    public boolean hasTimestamp() {
      return instance.hasTimestamp();
    }
    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    @java.lang.Override
    public com.google.protobuf.Timestamp getTimestamp() {
      return instance.getTimestamp();
    }
    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    public Builder setTimestamp(com.google.protobuf.Timestamp value) {
      copyOnWrite();
      instance.setTimestamp(value);
      return this;
      }
    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    public Builder setTimestamp(
        com.google.protobuf.Timestamp.Builder builderForValue) {
      copyOnWrite();
      instance.setTimestamp(builderForValue.build());
      return this;
    }
    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    public Builder mergeTimestamp(com.google.protobuf.Timestamp value) {
      copyOnWrite();
      instance.mergeTimestamp(value);
      return this;
    }
    /**
     * <code>.google.protobuf.Timestamp timestamp = 3 [json_name = "timestamp", (.gogoproto.nullable) = false, (.gogoproto.stdtime) = true];</code>
     */
    public Builder clearTimestamp() {  copyOnWrite();
      instance.clearTimestamp();
      return this;
    }

    /**
     * <code>bytes signature = 4 [json_name = "signature"];</code>
     * @return The signature.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getSignature() {
      return instance.getSignature();
    }
    /**
     * <code>bytes signature = 4 [json_name = "signature"];</code>
     * @param value The signature to set.
     * @return This builder for chaining.
     */
    public Builder setSignature(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setSignature(value);
      return this;
    }
    /**
     * <code>bytes signature = 4 [json_name = "signature"];</code>
     * @return This builder for chaining.
     */
    public Builder clearSignature() {
      copyOnWrite();
      instance.clearSignature();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:tendermint.types.CommitSig)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.tendermint.types.CommitSig();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "blockIdFlag_",
            "validatorAddress_",
            "timestamp_",
            "signature_",
          };
          java.lang.String info =
              "\u0000\u0004\u0000\u0000\u0001\u0004\u0004\u0000\u0000\u0000\u0001\f\u0002\n\u0003" +
              "\t\u0004\n";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.tendermint.types.CommitSig> parser = PARSER;
        if (parser == null) {
          synchronized (com.tendermint.types.CommitSig.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.tendermint.types.CommitSig>(
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


  // @@protoc_insertion_point(class_scope:tendermint.types.CommitSig)
  private static final com.tendermint.types.CommitSig DEFAULT_INSTANCE;
  static {
    CommitSig defaultInstance = new CommitSig();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      CommitSig.class, defaultInstance);
  }

  public static com.tendermint.types.CommitSig getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<CommitSig> PARSER;

  public static com.google.protobuf.Parser<CommitSig> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}

