// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: tendermint/crypto/proof.proto

package com.tendermint.crypto;

/**
 * Protobuf type {@code tendermint.crypto.Proof}
 */
public  final class Proof extends
    com.google.protobuf.GeneratedMessageLite<
        Proof, Proof.Builder> implements
    // @@protoc_insertion_point(message_implements:tendermint.crypto.Proof)
    ProofOrBuilder {
  private Proof() {
    leafHash_ = com.google.protobuf.ByteString.EMPTY;
    aunts_ = emptyProtobufList();
  }
  public static final int TOTAL_FIELD_NUMBER = 1;
  private long total_;
  /**
   * <code>int64 total = 1 [json_name = "total"];</code>
   * @return The total.
   */
  @java.lang.Override
  public long getTotal() {
    return total_;
  }
  /**
   * <code>int64 total = 1 [json_name = "total"];</code>
   * @param value The total to set.
   */
  private void setTotal(long value) {
    
    total_ = value;
  }
  /**
   * <code>int64 total = 1 [json_name = "total"];</code>
   */
  private void clearTotal() {
    
    total_ = 0L;
  }

  public static final int INDEX_FIELD_NUMBER = 2;
  private long index_;
  /**
   * <code>int64 index = 2 [json_name = "index"];</code>
   * @return The index.
   */
  @java.lang.Override
  public long getIndex() {
    return index_;
  }
  /**
   * <code>int64 index = 2 [json_name = "index"];</code>
   * @param value The index to set.
   */
  private void setIndex(long value) {
    
    index_ = value;
  }
  /**
   * <code>int64 index = 2 [json_name = "index"];</code>
   */
  private void clearIndex() {
    
    index_ = 0L;
  }

  public static final int LEAF_HASH_FIELD_NUMBER = 3;
  private com.google.protobuf.ByteString leafHash_;
  /**
   * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
   * @return The leafHash.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getLeafHash() {
    return leafHash_;
  }
  /**
   * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
   * @param value The leafHash to set.
   */
  private void setLeafHash(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    leafHash_ = value;
  }
  /**
   * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
   */
  private void clearLeafHash() {
    
    leafHash_ = getDefaultInstance().getLeafHash();
  }

  public static final int AUNTS_FIELD_NUMBER = 4;
  private com.google.protobuf.Internal.ProtobufList<com.google.protobuf.ByteString> aunts_;
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @return A list containing the aunts.
   */
  @java.lang.Override
  public java.util.List<com.google.protobuf.ByteString>
      getAuntsList() {
    return aunts_;
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @return The count of aunts.
   */
  @java.lang.Override
  public int getAuntsCount() {
    return aunts_.size();
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @param index The index of the element to return.
   * @return The aunts at the given index.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getAunts(int index) {
    return aunts_.get(index);
  }
  private void ensureAuntsIsMutable() {
    com.google.protobuf.Internal.ProtobufList<com.google.protobuf.ByteString> tmp = aunts_;
    if (!tmp.isModifiable()) {
      aunts_ =
          com.google.protobuf.GeneratedMessageLite.mutableCopy(tmp);
     }
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @param index The index to set the value at.
   * @param value The aunts to set.
   */
  private void setAunts(
      int index, com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  ensureAuntsIsMutable();
    aunts_.set(index, value);
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @param value The aunts to add.
   */
  private void addAunts(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  ensureAuntsIsMutable();
    aunts_.add(value);
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   * @param values The aunts to add.
   */
  private void addAllAunts(
      java.lang.Iterable<? extends com.google.protobuf.ByteString> values) {
    ensureAuntsIsMutable();
    com.google.protobuf.AbstractMessageLite.addAll(
        values, aunts_);
  }
  /**
   * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
   */
  private void clearAunts() {
    aunts_ = emptyProtobufList();
  }

  public static com.tendermint.crypto.Proof parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.crypto.Proof parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.tendermint.crypto.Proof parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.crypto.Proof parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.crypto.Proof parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.tendermint.crypto.Proof parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.tendermint.crypto.Proof prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * Protobuf type {@code tendermint.crypto.Proof}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.tendermint.crypto.Proof, Builder> implements
      // @@protoc_insertion_point(builder_implements:tendermint.crypto.Proof)
      com.tendermint.crypto.ProofOrBuilder {
    // Construct using com.tendermint.crypto.Proof.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <code>int64 total = 1 [json_name = "total"];</code>
     * @return The total.
     */
    @java.lang.Override
    public long getTotal() {
      return instance.getTotal();
    }
    /**
     * <code>int64 total = 1 [json_name = "total"];</code>
     * @param value The total to set.
     * @return This builder for chaining.
     */
    public Builder setTotal(long value) {
      copyOnWrite();
      instance.setTotal(value);
      return this;
    }
    /**
     * <code>int64 total = 1 [json_name = "total"];</code>
     * @return This builder for chaining.
     */
    public Builder clearTotal() {
      copyOnWrite();
      instance.clearTotal();
      return this;
    }

    /**
     * <code>int64 index = 2 [json_name = "index"];</code>
     * @return The index.
     */
    @java.lang.Override
    public long getIndex() {
      return instance.getIndex();
    }
    /**
     * <code>int64 index = 2 [json_name = "index"];</code>
     * @param value The index to set.
     * @return This builder for chaining.
     */
    public Builder setIndex(long value) {
      copyOnWrite();
      instance.setIndex(value);
      return this;
    }
    /**
     * <code>int64 index = 2 [json_name = "index"];</code>
     * @return This builder for chaining.
     */
    public Builder clearIndex() {
      copyOnWrite();
      instance.clearIndex();
      return this;
    }

    /**
     * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
     * @return The leafHash.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getLeafHash() {
      return instance.getLeafHash();
    }
    /**
     * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
     * @param value The leafHash to set.
     * @return This builder for chaining.
     */
    public Builder setLeafHash(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setLeafHash(value);
      return this;
    }
    /**
     * <code>bytes leaf_hash = 3 [json_name = "leafHash"];</code>
     * @return This builder for chaining.
     */
    public Builder clearLeafHash() {
      copyOnWrite();
      instance.clearLeafHash();
      return this;
    }

    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @return A list containing the aunts.
     */
    @java.lang.Override
    public java.util.List<com.google.protobuf.ByteString>
        getAuntsList() {
      return java.util.Collections.unmodifiableList(
          instance.getAuntsList());
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @return The count of aunts.
     */
    @java.lang.Override
    public int getAuntsCount() {
      return instance.getAuntsCount();
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @param index The index of the element to return.
     * @return The aunts at the given index.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getAunts(int index) {
      return instance.getAunts(index);
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @param value The aunts to set.
     * @return This builder for chaining.
     */
    public Builder setAunts(
        int index, com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setAunts(index, value);
      return this;
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @param value The aunts to add.
     * @return This builder for chaining.
     */
    public Builder addAunts(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.addAunts(value);
      return this;
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @param values The aunts to add.
     * @return This builder for chaining.
     */
    public Builder addAllAunts(
        java.lang.Iterable<? extends com.google.protobuf.ByteString> values) {
      copyOnWrite();
      instance.addAllAunts(values);
      return this;
    }
    /**
     * <code>repeated bytes aunts = 4 [json_name = "aunts"];</code>
     * @return This builder for chaining.
     */
    public Builder clearAunts() {
      copyOnWrite();
      instance.clearAunts();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:tendermint.crypto.Proof)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.tendermint.crypto.Proof();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "total_",
            "index_",
            "leafHash_",
            "aunts_",
          };
          java.lang.String info =
              "\u0000\u0004\u0000\u0000\u0001\u0004\u0004\u0000\u0001\u0000\u0001\u0002\u0002\u0002" +
              "\u0003\n\u0004\u001c";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.tendermint.crypto.Proof> parser = PARSER;
        if (parser == null) {
          synchronized (com.tendermint.crypto.Proof.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.tendermint.crypto.Proof>(
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


  // @@protoc_insertion_point(class_scope:tendermint.crypto.Proof)
  private static final com.tendermint.crypto.Proof DEFAULT_INSTANCE;
  static {
    Proof defaultInstance = new Proof();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      Proof.class, defaultInstance);
  }

  public static com.tendermint.crypto.Proof getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<Proof> PARSER;

  public static com.google.protobuf.Parser<Proof> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}
