// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: core/23-commitment/proofs.proto

package com.icon.proto.core.commitment;

/**
 * <pre>
 * Use BatchEntry not CommitmentProof, to avoid recursion
 * </pre>
 *
 * Protobuf type {@code icon.proto.core.commitment.BatchEntry}
 */
public  final class BatchEntry extends
    com.google.protobuf.GeneratedMessageLite<
        BatchEntry, BatchEntry.Builder> implements
    // @@protoc_insertion_point(message_implements:icon.proto.core.commitment.BatchEntry)
    BatchEntryOrBuilder {
  private BatchEntry() {
  }
  private int proofCase_ = 0;
  private java.lang.Object proof_;
  public enum ProofCase {
    EXIST(1),
    NONEXIST(2),
    PROOF_NOT_SET(0);
    private final int value;
    private ProofCase(int value) {
      this.value = value;
    }
    /**
     * @deprecated Use {@link #forNumber(int)} instead.
     */
    @java.lang.Deprecated
    public static ProofCase valueOf(int value) {
      return forNumber(value);
    }

    public static ProofCase forNumber(int value) {
      switch (value) {
        case 1: return EXIST;
        case 2: return NONEXIST;
        case 0: return PROOF_NOT_SET;
        default: return null;
      }
    }
    public int getNumber() {
      return this.value;
    }
  };

  @java.lang.Override
  public ProofCase
  getProofCase() {
    return ProofCase.forNumber(
        proofCase_);
  }

  private void clearProof() {
    proofCase_ = 0;
    proof_ = null;
  }

  public static final int EXIST_FIELD_NUMBER = 1;
  /**
   * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
   */
  @java.lang.Override
  public boolean hasExist() {
    return proofCase_ == 1;
  }
  /**
   * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
   */
  @java.lang.Override
  public com.icon.proto.core.commitment.ExistenceProof getExist() {
    if (proofCase_ == 1) {
       return (com.icon.proto.core.commitment.ExistenceProof) proof_;
    }
    return com.icon.proto.core.commitment.ExistenceProof.getDefaultInstance();
  }
  /**
   * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
   */
  private void setExist(com.icon.proto.core.commitment.ExistenceProof value) {
    value.getClass();
  proof_ = value;
    proofCase_ = 1;
  }
  /**
   * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
   */
  private void mergeExist(com.icon.proto.core.commitment.ExistenceProof value) {
    value.getClass();
  if (proofCase_ == 1 &&
        proof_ != com.icon.proto.core.commitment.ExistenceProof.getDefaultInstance()) {
      proof_ = com.icon.proto.core.commitment.ExistenceProof.newBuilder((com.icon.proto.core.commitment.ExistenceProof) proof_)
          .mergeFrom(value).buildPartial();
    } else {
      proof_ = value;
    }
    proofCase_ = 1;
  }
  /**
   * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
   */
  private void clearExist() {
    if (proofCase_ == 1) {
      proofCase_ = 0;
      proof_ = null;
    }
  }

  public static final int NONEXIST_FIELD_NUMBER = 2;
  /**
   * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
   */
  @java.lang.Override
  public boolean hasNonexist() {
    return proofCase_ == 2;
  }
  /**
   * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
   */
  @java.lang.Override
  public com.icon.proto.core.commitment.NonExistenceProof getNonexist() {
    if (proofCase_ == 2) {
       return (com.icon.proto.core.commitment.NonExistenceProof) proof_;
    }
    return com.icon.proto.core.commitment.NonExistenceProof.getDefaultInstance();
  }
  /**
   * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
   */
  private void setNonexist(com.icon.proto.core.commitment.NonExistenceProof value) {
    value.getClass();
  proof_ = value;
    proofCase_ = 2;
  }
  /**
   * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
   */
  private void mergeNonexist(com.icon.proto.core.commitment.NonExistenceProof value) {
    value.getClass();
  if (proofCase_ == 2 &&
        proof_ != com.icon.proto.core.commitment.NonExistenceProof.getDefaultInstance()) {
      proof_ = com.icon.proto.core.commitment.NonExistenceProof.newBuilder((com.icon.proto.core.commitment.NonExistenceProof) proof_)
          .mergeFrom(value).buildPartial();
    } else {
      proof_ = value;
    }
    proofCase_ = 2;
  }
  /**
   * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
   */
  private void clearNonexist() {
    if (proofCase_ == 2) {
      proofCase_ = 0;
      proof_ = null;
    }
  }

  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      java.nio.ByteBuffer data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      java.nio.ByteBuffer data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      com.google.protobuf.ByteString data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      com.google.protobuf.ByteString data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(byte[] data)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      byte[] data,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws com.google.protobuf.InvalidProtocolBufferException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, data, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(java.io.InputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseDelimitedFrom(java.io.InputStream input)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseDelimitedFrom(
      java.io.InputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return parseDelimitedFrom(DEFAULT_INSTANCE, input, extensionRegistry);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      com.google.protobuf.CodedInputStream input)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input);
  }
  public static com.icon.proto.core.commitment.BatchEntry parseFrom(
      com.google.protobuf.CodedInputStream input,
      com.google.protobuf.ExtensionRegistryLite extensionRegistry)
      throws java.io.IOException {
    return com.google.protobuf.GeneratedMessageLite.parseFrom(
        DEFAULT_INSTANCE, input, extensionRegistry);
  }

  public static Builder newBuilder() {
    return (Builder) DEFAULT_INSTANCE.createBuilder();
  }
  public static Builder newBuilder(com.icon.proto.core.commitment.BatchEntry prototype) {
    return (Builder) DEFAULT_INSTANCE.createBuilder(prototype);
  }

  /**
   * <pre>
   * Use BatchEntry not CommitmentProof, to avoid recursion
   * </pre>
   *
   * Protobuf type {@code icon.proto.core.commitment.BatchEntry}
   */
  public static final class Builder extends
      com.google.protobuf.GeneratedMessageLite.Builder<
        com.icon.proto.core.commitment.BatchEntry, Builder> implements
      // @@protoc_insertion_point(builder_implements:icon.proto.core.commitment.BatchEntry)
      com.icon.proto.core.commitment.BatchEntryOrBuilder {
    // Construct using com.icon.proto.core.commitment.BatchEntry.newBuilder()
    private Builder() {
      super(DEFAULT_INSTANCE);
    }

    @java.lang.Override
    public ProofCase
        getProofCase() {
      return instance.getProofCase();
    }

    public Builder clearProof() {
      copyOnWrite();
      instance.clearProof();
      return this;
    }


    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    @java.lang.Override
    public boolean hasExist() {
      return instance.hasExist();
    }
    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    @java.lang.Override
    public com.icon.proto.core.commitment.ExistenceProof getExist() {
      return instance.getExist();
    }
    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    public Builder setExist(com.icon.proto.core.commitment.ExistenceProof value) {
      copyOnWrite();
      instance.setExist(value);
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    public Builder setExist(
        com.icon.proto.core.commitment.ExistenceProof.Builder builderForValue) {
      copyOnWrite();
      instance.setExist(builderForValue.build());
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    public Builder mergeExist(com.icon.proto.core.commitment.ExistenceProof value) {
      copyOnWrite();
      instance.mergeExist(value);
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.ExistenceProof exist = 1 [json_name = "exist"];</code>
     */
    public Builder clearExist() {
      copyOnWrite();
      instance.clearExist();
      return this;
    }

    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    @java.lang.Override
    public boolean hasNonexist() {
      return instance.hasNonexist();
    }
    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    @java.lang.Override
    public com.icon.proto.core.commitment.NonExistenceProof getNonexist() {
      return instance.getNonexist();
    }
    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    public Builder setNonexist(com.icon.proto.core.commitment.NonExistenceProof value) {
      copyOnWrite();
      instance.setNonexist(value);
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    public Builder setNonexist(
        com.icon.proto.core.commitment.NonExistenceProof.Builder builderForValue) {
      copyOnWrite();
      instance.setNonexist(builderForValue.build());
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    public Builder mergeNonexist(com.icon.proto.core.commitment.NonExistenceProof value) {
      copyOnWrite();
      instance.mergeNonexist(value);
      return this;
    }
    /**
     * <code>.icon.proto.core.commitment.NonExistenceProof nonexist = 2 [json_name = "nonexist"];</code>
     */
    public Builder clearNonexist() {
      copyOnWrite();
      instance.clearNonexist();
      return this;
    }

    // @@protoc_insertion_point(builder_scope:icon.proto.core.commitment.BatchEntry)
  }
  @java.lang.Override
  @java.lang.SuppressWarnings({"unchecked", "fallthrough"})
  protected final java.lang.Object dynamicMethod(
      com.google.protobuf.GeneratedMessageLite.MethodToInvoke method,
      java.lang.Object arg0, java.lang.Object arg1) {
    switch (method) {
      case NEW_MUTABLE_INSTANCE: {
        return new com.icon.proto.core.commitment.BatchEntry();
      }
      case NEW_BUILDER: {
        return new Builder();
      }
      case BUILD_MESSAGE_INFO: {
          java.lang.Object[] objects = new java.lang.Object[] {
            "proof_",
            "proofCase_",
            com.icon.proto.core.commitment.ExistenceProof.class,
            com.icon.proto.core.commitment.NonExistenceProof.class,
          };
          java.lang.String info =
              "\u0000\u0002\u0001\u0000\u0001\u0002\u0002\u0000\u0000\u0000\u0001<\u0000\u0002<" +
              "\u0000";
          return newMessageInfo(DEFAULT_INSTANCE, info, objects);
      }
      // fall through
      case GET_DEFAULT_INSTANCE: {
        return DEFAULT_INSTANCE;
      }
      case GET_PARSER: {
        com.google.protobuf.Parser<com.icon.proto.core.commitment.BatchEntry> parser = PARSER;
        if (parser == null) {
          synchronized (com.icon.proto.core.commitment.BatchEntry.class) {
            parser = PARSER;
            if (parser == null) {
              parser =
                  new DefaultInstanceBasedParser<com.icon.proto.core.commitment.BatchEntry>(
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


  // @@protoc_insertion_point(class_scope:icon.proto.core.commitment.BatchEntry)
  private static final com.icon.proto.core.commitment.BatchEntry DEFAULT_INSTANCE;
  static {
    BatchEntry defaultInstance = new BatchEntry();
    // New instances are implicitly immutable so no need to make
    // immutable.
    DEFAULT_INSTANCE = defaultInstance;
    com.google.protobuf.GeneratedMessageLite.registerDefaultInstance(
      BatchEntry.class, defaultInstance);
  }

  public static com.icon.proto.core.commitment.BatchEntry getDefaultInstance() {
    return DEFAULT_INSTANCE;
  }

  private static volatile com.google.protobuf.Parser<BatchEntry> PARSER;

  public static com.google.protobuf.Parser<BatchEntry> parser() {
    return DEFAULT_INSTANCE.getParserForType();
  }
}

