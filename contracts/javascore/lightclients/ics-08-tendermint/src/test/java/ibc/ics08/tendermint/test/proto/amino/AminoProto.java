// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: amino/amino.proto

package com.amino;

public final class AminoProto {
  private AminoProto() {}
  public static void registerAllExtensions(
      com.google.protobuf.ExtensionRegistryLite registry) {
    registry.add(com.amino.AminoProto.name);
    registry.add(com.amino.AminoProto.messageEncoding);
    registry.add(com.amino.AminoProto.encoding);
    registry.add(com.amino.AminoProto.fieldName);
    registry.add(com.amino.AminoProto.dontOmitempty);
  }
  public static final int NAME_FIELD_NUMBER = 11110001;
  /**
   * <pre>
   * name is the string used when registering a concrete
   * type into the Amino type registry, via the Amino codec's
   * `RegisterConcrete()` method. This string MUST be at most 39
   * characters long, or else the message will be rejected by the
   * Ledger hardware device.
   * </pre>
   *
   * <code>extend .google.protobuf.MessageOptions { ... }</code>
   */
  public static final
    com.google.protobuf.GeneratedMessageLite.GeneratedExtension<
      com.google.protobuf.DescriptorProtos.MessageOptions,
      java.lang.String> name = com.google.protobuf.GeneratedMessageLite
          .newSingularGeneratedExtension(
        com.google.protobuf.DescriptorProtos.MessageOptions.getDefaultInstance(),
        "",
        null,
        null,
        11110001,
        com.google.protobuf.WireFormat.FieldType.STRING,
        java.lang.String.class);
  public static final int MESSAGE_ENCODING_FIELD_NUMBER = 11110002;
  /**
   * <pre>
   * encoding describes the encoding format used by Amino for the given
   * message. The field type is chosen to be a string for
   * flexibility, but it should ideally be short and expected to be
   * machine-readable, for example "base64" or "utf8_json". We
   * highly recommend to use underscores for word separation instead of spaces.
   * If left empty, then the Amino encoding is expected to be the same as the
   * Protobuf one.
   * This annotation should not be confused with the `encoding`
   * one which operates on the field level.
   * </pre>
   *
   * <code>extend .google.protobuf.MessageOptions { ... }</code>
   */
  public static final
    com.google.protobuf.GeneratedMessageLite.GeneratedExtension<
      com.google.protobuf.DescriptorProtos.MessageOptions,
      java.lang.String> messageEncoding = com.google.protobuf.GeneratedMessageLite
          .newSingularGeneratedExtension(
        com.google.protobuf.DescriptorProtos.MessageOptions.getDefaultInstance(),
        "",
        null,
        null,
        11110002,
        com.google.protobuf.WireFormat.FieldType.STRING,
        java.lang.String.class);
  public static final int ENCODING_FIELD_NUMBER = 11110003;
  /**
   * <pre>
   * encoding describes the encoding format used by Amino for
   * the given field. The field type is chosen to be a string for
   * flexibility, but it should ideally be short and expected to be
   * machine-readable, for example "base64" or "utf8_json". We
   * highly recommend to use underscores for word separation instead of spaces.
   * If left empty, then the Amino encoding is expected to be the same as the
   * Protobuf one.
   * This annotation should not be confused with the
   * `message_encoding` one which operates on the message level.
   * </pre>
   *
   * <code>extend .google.protobuf.FieldOptions { ... }</code>
   */
  public static final
    com.google.protobuf.GeneratedMessageLite.GeneratedExtension<
      com.google.protobuf.DescriptorProtos.FieldOptions,
      java.lang.String> encoding = com.google.protobuf.GeneratedMessageLite
          .newSingularGeneratedExtension(
        com.google.protobuf.DescriptorProtos.FieldOptions.getDefaultInstance(),
        "",
        null,
        null,
        11110003,
        com.google.protobuf.WireFormat.FieldType.STRING,
        java.lang.String.class);
  public static final int FIELD_NAME_FIELD_NUMBER = 11110004;
  /**
   * <pre>
   * field_name sets a different field name (i.e. key name) in
   * the amino JSON object for the given field.
   * Example:
   * message Foo {
   *   string bar = 1 [(amino.field_name) = "baz"];
   * }
   * Then the Amino encoding of Foo will be:
   * `{"baz":"some value"}`
   * </pre>
   *
   * <code>extend .google.protobuf.FieldOptions { ... }</code>
   */
  public static final
    com.google.protobuf.GeneratedMessageLite.GeneratedExtension<
      com.google.protobuf.DescriptorProtos.FieldOptions,
      java.lang.String> fieldName = com.google.protobuf.GeneratedMessageLite
          .newSingularGeneratedExtension(
        com.google.protobuf.DescriptorProtos.FieldOptions.getDefaultInstance(),
        "",
        null,
        null,
        11110004,
        com.google.protobuf.WireFormat.FieldType.STRING,
        java.lang.String.class);
  public static final int DONT_OMITEMPTY_FIELD_NUMBER = 11110005;
  /**
   * <pre>
   * dont_omitempty sets the field in the JSON object even if
   * its value is empty, i.e. equal to the Golang zero value. To learn what
   * the zero values are, see https://go.dev/ref/spec#The_zero_value.
   * Fields default to `omitempty`, which is the default behavior when this
   * annotation is unset. When set to true, then the field value in the
   * JSON object will be set, i.e. not `undefined`.
   * Example:
   * message Foo {
   *   string bar = 1;
   *   string baz = 2 [(amino.dont_omitempty) = true];
   * }
   * f := Foo{};
   * out := AminoJSONEncoder(&amp;f);
   * out == {"baz":""}
   * </pre>
   *
   * <code>extend .google.protobuf.FieldOptions { ... }</code>
   */
  public static final
    com.google.protobuf.GeneratedMessageLite.GeneratedExtension<
      com.google.protobuf.DescriptorProtos.FieldOptions,
      java.lang.Boolean> dontOmitempty = com.google.protobuf.GeneratedMessageLite
          .newSingularGeneratedExtension(
        com.google.protobuf.DescriptorProtos.FieldOptions.getDefaultInstance(),
        false,
        null,
        null,
        11110005,
        com.google.protobuf.WireFormat.FieldType.BOOL,
        java.lang.Boolean.class);

  static {
  }

  // @@protoc_insertion_point(outer_class_scope)
}
