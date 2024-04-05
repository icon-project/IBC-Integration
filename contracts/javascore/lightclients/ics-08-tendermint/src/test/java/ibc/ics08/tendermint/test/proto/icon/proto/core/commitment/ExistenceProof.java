// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/23-commitment/proofs.proto

package com.icon.proto.core.commitment;

/**
 * <pre>
 **
 *ExistenceProof takes a key and a value and a set of steps to perform on it.
 *The result of peforming all these steps will provide a "root hash", which can
 *be compared to the value in a header.
 *Since it is computationally infeasible to produce a hash collission for any of the used
 *cryptographic hash functions, if someone can provide a series of operations to transform
 *a given key and value into a root hash that matches some trusted root, these key and values
 *must be in the referenced merkle tree.
 *The only possible issue is maliablity in LeafOp, such as providing extra prefix data,
 *which should be controlled by a spec. Eg. with lengthOp as NONE,
 *prefix = FOO, key = BAR, value = CHOICE
 *and
 *prefix = F, key = OOBAR, value = CHOICE
 *would produce the same value.
 *With LengthOp this is tricker but not impossible. Which is why the "leafPrefixEqual" field
 *in the ProofSpec is valuable to prevent this mutability. And why all trees should
 *length-prefix the data before hashing it.
 * </pre>
 *
 * Protobuf type {@code icon.proto.core.commitment.ExistenceProof}
 */
public  final class ExistenceProof extends
    com.google.protobuf.GeneratedMessageLite<
        ExistenceProof, ExistenceProof.Builder> implements
    // @@protoc_insertion_point(message_implements:icon.proto.core.commitment.ExistenceProof)
    ExistenceProofOrBuilder {
  private ExistenceProof() {
    key_ = com.google.protobuf.ByteString.EMPTY;
    value_ = com.google.protobuf.ByteString.EMPTY;
    path_ = emptyProtobufList();
  }
  public static final int KEY_FIELD_NUMBER = 1;
  private com.google.protobuf.ByteString key_;
  /**
   * <code>bytes key = 1 [json_name = "key"];</code>
   * @return The key.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getKey() {
    return key_;
  }
  /**
   * <code>bytes key = 1 [json_name = "key"];</code>
   * @param value The key to set.
   */
  private void setKey(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    key_ = value;
  }
  /**
   * <code>bytes key = 1 [json_name = "key"];</code>
   */
  private void clearKey() {
    
    key_ = getDefaultInstance().getKey();
  }

  public static final int VALUE_FIELD_NUMBER = 2;
  private com.google.protobuf.ByteString value_;
  /**
   * <code>bytes value = 2 [json_name = "value"];</code>
   * @return The value.
   */
  @java.lang.Override
  public com.google.protobuf.ByteString getValue() {
    return value_;
  }
  /**
   * <code>bytes value = 2 [json_name = "value"];</code>
   * @param value The value to set.
   */
  private void setValue(com.google.protobuf.ByteString value) {
    java.lang.Class<?> valueClass = value.getClass();
  
    value_ = value;
  }
  /**
   * <code>bytes value = 2 [json_name = "value"];</code>
   */
  private void clearValue() {
    
    value_ = getDefaultInstance().getValue();
  }

  public static final int LEAF_FIELD_NUMBER = 3;
  private com.icon.proto.core.commitment.LeafOp leaf_;
  /**
   * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
   */
  @java.lang.Override
  public boolean hasLeaf() {
    return leaf_ != null;
  }
  /**
   * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
   */
  @java.lang.Override
  public com.icon.proto.core.commitment.LeafOp getLeaf() {
    return leaf_ == null ? com.icon.proto.core.commitment.LeafOp.getDefaultInstance() : leaf_;
  }
  /**
   * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
   */
  private void setLeaf(com.icon.proto.core.commitment.LeafOp value) {
    value.getClass();
  leaf_ = value;
    
    }
  /**
   * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
   */
  @java.lang.SuppressWarnings({"ReferenceEquality"})
  private void mergeLeaf(com.icon.proto.core.commitment.LeafOp value) {
    value.getClass();
  if (leaf_ != null &&
        leaf_ != com.icon.proto.core.commitment.LeafOp.getDefaultInstance()) {
      leaf_ =
        com.icon.proto.core.commitment.LeafOp.newBuilder(leaf_).mergeFrom(value).buildPartial();
    } else {
      leaf_ = value;
    }
    
  }
  /**
   * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
   */
  private void clearLeaf() {  leaf_ = null;
    
  }

  public static final int PATH_FIELD_NUMBER = 4;
  private com.google.protobuf.Internal.ProtobufList<com.icon.proto.core.commitment.InnerOp> path_;
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  @java.lang.Override
  public java.util.List<com.icon.proto.core.commitment.InnerOp> getPathList() {
    return path_;
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  public java.util.List<? extends com.icon.proto.core.commitment.InnerOpOrBuilder> 
      getPathOrBuilderList() {
    return path_;
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  @java.lang.Override
  public int getPathCount() {
    return path_.size();
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  @java.lang.Override
  public com.icon.proto.core.commitment.InnerOp getPath(int index) {
    return path_.get(index);
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  public com.icon.proto.core.commitment.InnerOpOrBuilder getPathOrBuilder(
      int index) {
    return path_.get(index);
  }
  private void ensurePathIsMutable() {
    com.google.protobuf.Internal.ProtobufList<com.icon.proto.core.commitment.InnerOp> tmp = path_;
    if (!tmp.isModifiable()) {
      path_ =
          com.google.protobuf.GeneratedMessageLite.mutableCopy(tmp);
     }
  }

  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void setPath(
      int index, com.icon.proto.core.commitment.InnerOp value) {
    value.getClass();
  ensurePathIsMutable();
    path_.set(index, value);
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void addPath(com.icon.proto.core.commitment.InnerOp value) {
    value.getClass();
  ensurePathIsMutable();
    path_.add(value);
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void addPath(
      int index, com.icon.proto.core.commitment.InnerOp value) {
    value.getClass();
  ensurePathIsMutable();
    path_.add(index, value);
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void addAllPath(
      java.lang.Iterable<? extends com.icon.proto.core.commitment.InnerOp> values) {
    ensurePathIsMutable();
    com.google.protobuf.AbstractMessageLite.addAll(
        values, path_);
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void clearPath() {
    path_ = emptyProtobufList();
  }
  /**
   * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
   */
  private void removePath(int index) {
    ensurePathIsMutable();
    path_.remove(index);
  }

  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.ExistenceProof parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.icon.proto.core.commitment.ExistenceProof prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   **
   *ExistenceProof takes a key and a value and a set of steps to perform on it.
   *The result of peforming all these steps will provide a "root hash", which can
   *be compared to the value in a header.
   *Since it is computationally infeasible to produce a hash collission for any of the used
   *cryptographic hash functions, if someone can provide a series of operations to transform
   *a given key and value into a root hash that matches some trusted root, these key and values
   *must be in the referenced merkle tree.
   *The only possible issue is maliablity in LeafOp, such as providing extra prefix data,
   *which should be controlled by a spec. Eg. with lengthOp as NONE,
   *prefix = FOO, key = BAR, value = CHOICE
   *and
   *prefix = F, key = OOBAR, value = CHOICE
   *would produce the same value.
   *With LengthOp this is tricker but not impossible. Which is why the "leafPrefixEqual" field
   *in the ProofSpec is valuable to prevent this mutability. And why all trees should
   *length-prefix the data before hashing it.
   * </pre>
   *
   * Protobuf type {@code icon.proto.core.commitment.ExistenceProof}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.icon.proto.core.commitment.ExistenceProof, Builder> implements
      // @@protoc_insertion_point(builder_implements:icon.proto.core.commitment.ExistenceProof)
      com.icon.proto.core.commitment.ExistenceProofOrBuilder {
    // Construct using com.icon.proto.core.commitment.ExistenceProof.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }


    /**
     * <code>bytes key = 1 [json_name = "key"];</code>
     * @return The key.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getKey() {
      return instance.getKey();
    }
    /**
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
     * <code>bytes key = 1 [json_name = "key"];</code>
     * @return This builder for chaining.
     */
    public Builder clearKey() {
      copyOnWrite();
      instance.clearKey();
      return this;
    }

    /**
     * <code>bytes value = 2 [json_name = "value"];</code>
     * @return The value.
     */
    @java.lang.Override
    public com.google.protobuf.ByteString getValue() {
      return instance.getValue();
    }
    /**
     * <code>bytes value = 2 [json_name = "value"];</code>
     * @param value The value to set.
     * @return This builder for chaining.
     */
    public Builder setValue(com.google.protobuf.ByteString value) {
      copyOnWrite();
      instance.setValue(value);
      return this;
    }
    /**
     * <code>bytes value = 2 [json_name = "value"];</code>
     * @return This builder for chaining.
     */
    public Builder clearValue() {
      copyOnWrite();
      instance.clearValue();
      return this;
    }

    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    @java.lang.Override
    public boolean hasLeaf() {
      return instance.hasLeaf();
    }
    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    @java.lang.Override
    public com.icon.proto.core.commitment.LeafOp getLeaf() {
      return instance.getLeaf();
    }
    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    public Builder setLeaf(com.icon.proto.core.commitment.LeafOp value) {
      copyOnWrite();
      instance.setLeaf(value);
      return this;
      }
    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    public Builder setLeaf(
        com.icon.proto.core.commitment.LeafOp.Builder builderForValue) {
      copyOnWrite();
      instance.setLeaf(builderForValue.build());
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    public Builder mergeLeaf(com.icon.proto.core.commitment.LeafOp value) {
      copyOnWrite();
      instance.mergeLeaf(value);
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.LeafOp leaf = 3 [json_name = "leaf"];</code>
     */
    public Builder clearLeaf() {  copyOnWrite();
      instance.clearLeaf();
      return this;
    }

    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    @java.lang.Override
    public java.util.List<com.icon.proto.core.commitment.InnerOp> getPathList() {
      return java.util.Collections.unmodifiableList(
          instance.getPathList());
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    @java.lang.Override
    public int getPathCount() {
      return instance.getPathCount();
    }/**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    @java.lang.Override
    public com.icon.proto.core.commitment.InnerOp getPath(int index) {
      return instance.getPath(index);
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder setPath(
        int index, com.icon.proto.core.commitment.InnerOp value) {
      copyOnWrite();
      instance.setPath(index, value);
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder setPath(
        int index, com.icon.proto.core.commitment.InnerOp.Builder builderForValue) {
      copyOnWrite();
      instance.setPath(index,
          builderForValue.build());
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder addPath(com.icon.proto.core.commitment.InnerOp value) {
      copyOnWrite();
      instance.addPath(value);
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder addPath(
        int index, com.icon.proto.core.commitment.InnerOp value) {
      copyOnWrite();
      instance.addPath(index, value);
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder addPath(
        com.icon.proto.core.commitment.InnerOp.Builder builderForValue) {
      copyOnWrite();
      instance.addPath(builderForValue.build());
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder addPath(
        int index, com.icon.proto.core.commitment.InnerOp.Builder builderForValue) {
      copyOnWrite();
      instance.addPath(index,
          builderForValue.build());
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder addAllPath(
        java.lang.Iterable<? extends com.icon.proto.core.commitment.InnerOp> values) {
      copyOnWrite();
      instance.addAllPath(values);
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder clearPath() {
      copyOnWrite();
      instance.clearPath();
      return this;
    }
    /**
     * <code>repeated .icon.proto.core.commitment.InnerOp path = 4 [json_name = "path"];</code>
     */
    public Builder removePath(int index) {
      copyOnWrite();
      instance.removePath(index);
      return this;
    }

    // @@protoc_insertion_point(builder_scope:icon.proto.core.commitment.ExistenceProof)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.icon.proto.core.commitment.ExistenceProof();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "key_",
            "value_",
            "leaf_",
            "path_",
            com.icon.proto.core.commitment.InnerOp.class,
          };
          java.lang.String info =
              "\u0000\u0004\u0000\u0000\u0001\u0004\u0004\u0000\u0001\u0000\u0001\n\u0002\n\u0003" +
              "\t\u0004\u001b";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.icon.proto.core.commitment.ExistenceProof> parser = PARSER;
        if (parser == null) {
          synchronized (com.icon.proto.core.commitment.ExistenceProof.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.icon.proto.core.commitment.ExistenceProof>(
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


  // @@protoc_insertion_point(class_scope:icon.proto.core.commitment.ExistenceProof)
  private static final com.icon.proto.core.commitment.ExistenceProof DEFAULT_INSTANCE;
  static {
    ExistenceProof defaultInstance = new ExistenceProof();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      ExistenceProof.class, defaultInstance);
  }

  public static com.icon.proto.core.commitment.ExistenceProof getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<ExistenceProof> PARSER;

  public static com.google.protobuf.Parser<ExistenceProof> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}
