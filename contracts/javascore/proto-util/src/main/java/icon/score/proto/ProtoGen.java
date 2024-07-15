package icon.score.proto;

import java.io.File;
import java.math.BigInteger;
import java.nio.file.Path;
import java.util.Iterator;
import java.util.List;

import com.squareup.javapoet.ArrayTypeName;
import com.squareup.javapoet.ClassName;
import com.squareup.javapoet.CodeBlock;
import com.squareup.javapoet.FieldSpec;
import com.squareup.javapoet.JavaFile;
import com.squareup.javapoet.MethodSpec;
import com.squareup.javapoet.ParameterSpec;
import com.squareup.javapoet.ParameterizedTypeName;
import com.squareup.javapoet.TypeName;
import com.squareup.javapoet.TypeSpec;

import ibc.icon.score.util.ProtoMessage;

import com.google.inject.Guice;
import com.google.inject.Injector;
import io.protostuff.compiler.ParserModule;
import io.protostuff.compiler.model.EnumConstant;
import io.protostuff.compiler.model.Field;
import io.protostuff.compiler.model.Message;
import io.protostuff.compiler.model.Proto;
import io.protostuff.compiler.parser.FileReader;
import io.protostuff.compiler.parser.FileReaderFactory;
import io.protostuff.compiler.parser.Importer;
import io.protostuff.compiler.parser.ProtoContext;
import javax.lang.model.element.*;

import org.apache.commons.io.FileUtils;

public class ProtoGen {
    public static void main(String[] args) throws Exception {
            String path = "../../../proto";
            final Injector injector = Guice.createInjector(new ParserModule());

            final FileReaderFactory fileReaderFactory = injector.getInstance(FileReaderFactory.class);
            final List<Path> includePaths = List.of(
                    Path.of(path));
            final FileReader fileReader = fileReaderFactory.create(includePaths);

            final Importer importer = injector.getInstance(Importer.class);

            Iterator<File> it = FileUtils.iterateFiles(new File(path), new String[]{"proto"},
                    true);

            while (it.hasNext()) {
                File file = it.next();
                String filePath = file.getPath().replace(path + "/", "");
                if (!(
                    filePath.startsWith("ibc/lightclients/tendermint") ||
                    filePath.startsWith("tendermint/types") ||
                    filePath.startsWith("core")
                )) {
                    continue;
                }
                try {
                    final ProtoContext protoContext = importer.importFile(fileReader, filePath);
                    generate(protoContext);
                } catch (Exception e) {
                    System.out.println("Failed to generate proto: " + e);
                }

        }
    }

    private static void generate(ProtoContext protoContext ) {
        try {

            final Proto proto = protoContext.getProto();
            for (ProtoContext ctx : protoContext.getImports()) {
                generate(ctx);
            }
            String targetPath = "build/generated/sources";
            System.out.println(proto.getPackage().toString());
            System.out.println(proto.getCanonicalName());
            final List<io.protostuff.compiler.model.Enum> enums = proto.getEnums();
            for (io.protostuff.compiler.model.Enum _enum : enums) {
                JavaFile javaFile = JavaFile.builder(proto.getPackage().toString(), createEnum(_enum).build())
                        .build();

                javaFile.writeTo(new File(targetPath));
            }

            final List<Message> messages = proto.getMessages();
            for (Message message : messages) {

                JavaFile javaFile = JavaFile.builder(proto.getPackage().toString(), createMessage(message).build())
                        .build();

                javaFile.writeTo(new File(targetPath));
            }
        } catch (Exception e) {
            System.out.println("Failed to generate proto: " + e);
        }
    }
    private static TypeSpec.Builder createEnum(io.protostuff.compiler.model.Enum protoEnum) {
        TypeSpec.Builder enumSpec = TypeSpec.classBuilder(protoEnum.getName())
                .addModifiers(Modifier.PUBLIC);
        for (EnumConstant _enum : protoEnum.getConstants()) {
            FieldSpec fieldSpec = FieldSpec.builder(int.class, _enum.getName(),
                            Modifier.PUBLIC, Modifier.FINAL, Modifier.STATIC)
                    .initializer("$L", _enum.getValue()).build();
            enumSpec.addField(fieldSpec);
        }
        return enumSpec;
    }

    private static TypeSpec.Builder createMessage(Message protoMessage) {
        TypeSpec.Builder messageSpec = TypeSpec.classBuilder(protoMessage.getName())
                .addModifiers(Modifier.PUBLIC)
                .superclass(ProtoMessage.class);

        final List<io.protostuff.compiler.model.Enum> enums = protoMessage.getEnums();
        for (io.protostuff.compiler.model.Enum _enum : enums) {
            messageSpec.addType(createEnum(_enum).addModifiers(Modifier.STATIC).build());
        }

        final List<Message> messages = protoMessage.getMessages();
        for (Message message : messages) {
            messageSpec.addType(createMessage(message).addModifiers(Modifier.STATIC).build());
        }

        MethodSpec.Builder encodingSpec = MethodSpec.methodBuilder("encode")
                .returns(byte[].class)
                .addModifiers(Modifier.PUBLIC)
                .addCode("return $T.join(\n$>", ibc.icon.score.util.ByteUtil.class);

        MethodSpec.Builder decodingSpec = MethodSpec.methodBuilder("decode")
                .returns(ClassName.bestGuess(protoMessage.getName()))
                .addModifiers(Modifier.PUBLIC)
                .addModifiers(Modifier.STATIC)
                .addParameter(ParameterSpec.builder(ArrayTypeName.get(byte[].class), "data").build())
                .addStatement("$L obj = new $L()", protoMessage.getName(), protoMessage.getName())
                .addStatement("int index = 0")
                .addStatement("int order")
                .addStatement("int length = data.length");

        CodeBlock.Builder decodingLoop = CodeBlock.builder()
                .beginControlFlow("while (index < length)")
                .addStatement("order = data[index] >> 3")
                .addStatement("index++")
                .beginControlFlow("switch(order)");
        boolean isFirst = true;
        for (Field field : protoMessage.getFields()) {
            // BASE
            messageSpec.addField(generateFieldSpec(field));
            messageSpec.addMethod(getGetter(field));
            messageSpec.addMethod(getSetter(field));

            // ENCODING
            if (!isFirst) {
                encodingSpec.addCode(",\n");
            }

            encodingSpec.addCode("$T.$L($L, this.$L)",
                    ibc.icon.score.util.Proto.class, getEncoder(field), field.getIndex(), toCamel(field.getName()));

            // DECODING
            decodingLoop.beginControlFlow("case " + field.getIndex() + ":");
            decodingLoop.add(generateDecoder(field));
            decodingLoop.endControlFlow();
            isFirst = false;

        }

        decodingLoop.endControlFlow();
        decodingLoop.endControlFlow();

        decodingSpec.addCode(decodingLoop.build());
        decodingSpec.addStatement("return obj");

        encodingSpec.addCode("$<);");
        messageSpec.addMethod(encodingSpec.build());
        messageSpec.addMethod(decodingSpec.build());

        return messageSpec;

    }

    private static MethodSpec getSetter(Field field) {
        TypeName typeName = getTypeName(field);
        if (field.isRepeated()) {
            typeName = ParameterizedTypeName.get(ClassName.get(List.class), getTypeName(field));
        }

        String name = toCamel(field.getName());
        String functionName = name.substring(0, 1).toUpperCase() + name.substring(1);
        return MethodSpec.methodBuilder("set" + functionName)
                .addParameter(typeName, name)
                .addModifiers(Modifier.PUBLIC)
                .addStatement("this.$L = $L", name, name).build();

    }

    private static MethodSpec getGetter(Field field) {
        TypeName typeName = getTypeName(field);
        if (field.isRepeated()) {
            typeName = ParameterizedTypeName.get(ClassName.get(List.class), getTypeName(field));
        }

        String name = toCamel(field.getName());
        String functionName = name.substring(0, 1).toUpperCase() + name.substring(1);
        return MethodSpec.methodBuilder("get" + functionName)
                .returns(typeName)
                .addModifiers(Modifier.PUBLIC)
                .addStatement("return this.$L", name).build();
    }

    private static CodeBlock generateDecoder(Field field) {
        CodeBlock.Builder decodingBlock = CodeBlock.builder();

        TypeName _responseType = getDecoderResponseType(field);

        boolean isVarIntArray = field.isRepeated() && _responseType.equals(ClassName.get(BigInteger.class));

        if (isVarIntArray) {
            _responseType = ParameterizedTypeName.get(ClassName.get(List.class), _responseType);

        }
        TypeName responseType = ParameterizedTypeName.get(ClassName.get(ibc.icon.score.util.Proto.DecodeResponse.class),
                _responseType);
        decodingBlock.addStatement("$>$T resp = $T.$L(data, index)", responseType, ibc.icon.score.util.Proto.class,
                getDecoder(field));
        decodingBlock.addStatement("index = resp.index");
        String result = "resp.res";
        if (field.getType().isMessage()) {
            result = field.getTypeName() + ".decode(resp.res)";
        }

        if (isVarIntArray) {
            decodingBlock.addStatement("obj.$L.addAll($L)", toCamel(field.getName()), result);
        } else if (field.isRepeated()) {
            decodingBlock.addStatement("obj.$L.add($L)", toCamel(field.getName()), result);
        } else {
            decodingBlock.addStatement("obj.$L = $L", toCamel(field.getName()), result);

        }
        decodingBlock.addStatement("break");
        decodingBlock.add("$<");

        return decodingBlock.build();

    }

    private static FieldSpec generateFieldSpec(Field field) {
        if (field.isRepeated()) {
            TypeName typeName = ParameterizedTypeName.get(ClassName.get(List.class), getTypeName(field));
            return FieldSpec.builder(typeName, toCamel(field.getName()), Modifier.PRIVATE)
                    .initializer("new $T<>()", scorex.util.ArrayList.class).build();
        }
        FieldSpec.Builder fieldSpec = FieldSpec.builder(getTypeName(field), toCamel(field.getName()), Modifier.PRIVATE);
        if (field.getType().isEnum()) {
            return fieldSpec.initializer("0").build();
        }

        if (field.getType().isMessage()) {
            fieldSpec.initializer("new $L()", getTypeName(field));
            return fieldSpec.build();
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
            case "fixed32":
            case "fixed64":
            case "sfixed32":
            case "sfixed64":
                return fieldSpec.initializer("BigInteger.ZERO").build();
            case "bool":
                return fieldSpec.initializer("false").build();
            case "string":
                return fieldSpec.initializer("\"\"").build();
            case "bytes":
                return fieldSpec.initializer("new byte[0]").build();
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }

    }

    private static String getDecoder(Field field) {
        if (field.getType().isMessage()) {
            return "decodeBytes";
        }
        if (field.getType().isEnum()) {
            return "decodeEnum";
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
                return field.isRepeated() ? "decodeVarIntArray" : "decodeVarInt";
            case "fixed32":
            case "sfixed32":
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
            case "fixed64":
            case "sfixed64":
                return "decodeFixed64";
            case "bool":
                return "decodeBoolean";
            case "string":
                return "decodeString";
            case "bytes":
                return "decodeBytes";
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }
    }

    private static String getEncoder(Field field) {
        if (field.isRepeated()) {
            return getArrayEncoder(field);
        }

        if (field.getType().isMessage() || field.getType().isEnum()) {
            return "encode";
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
                return "encode";
            case "fixed32":
            case "sfixed32":
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
            case "fixed64":
            case "sfixed64":
                return "encodeFixed64";
            case "bool":
                return "encode";
            case "string":
                return "encode";
            case "bytes":
                return "encode";
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }
    }

    private static String getArrayEncoder(Field field) {
        if (field.getType().isMessage()) {
            return "encodeMessageArray";
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
            case "ScalarType":
                return "encodeVarIntArray";
            case "fixed32":
            case "sfixed32":
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
            case "fixed64":
            case "sfixed64":
                return "encodeFixed64Array";
            case "bool":
                return "encodeBooleanArray";
            case "string":
                return "encodeStringArray";
            case "bytes":
                return "encodeBytesArray";
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }
    }

    private static TypeName getTypeName(Field field) {
        if (field.getType().isMessage()) {

            return ClassName.bestGuess(field.getTypeName());
        }

        if (field.getType().isEnum() && !field.isRepeated()) {
            return TypeName.INT;
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
            case "fixed32":
            case "fixed64":
            case "sfixed32":
            case "sfixed64":
            case "ScalarType":
                return ClassName.get(BigInteger.class);
            case "bool":
                return TypeName.BOOLEAN;
            case "string":
                return ClassName.get(String.class);
            case "bytes":
                return ArrayTypeName.get(byte[].class);
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }
    }

    private static TypeName getDecoderResponseType(Field field) {
        if (field.getType().isMessage()) {
            return ArrayTypeName.get(byte[].class);
        }

        if (field.getType().isEnum()) {
            return ClassName.get(Integer.class);
        }

        switch (field.getTypeName()) {
            case "int":
            case "int32":
            case "int64":
            case "uint32":
            case "uint64":
            case "sint32":
            case "sint64":
            case "fixed32":
            case "fixed64":
            case "sfixed32":
            case "sfixed64":
                return ClassName.get(BigInteger.class);
            case "bool":
                return ClassName.get(Boolean.class);
            case "string":
                return ClassName.get(String.class);
            case "bytes":
                return ArrayTypeName.get(byte[].class);
            default:
                throw new IllegalArgumentException("Type currently not supported " + field.getTypeName());
        }
    }

    private static String toCamel(String s) {
        String[] words = s.split("[\\W_]+");
        StringBuilder builder = new StringBuilder();
        for (int i = 0; i < words.length; i++) {
            String word = words[i];
            if (i == 0) {
                word = word.isEmpty() ? word : word.toLowerCase();
            } else {
                word = word.isEmpty() ? word
                        : Character.toUpperCase(word.charAt(0)) +
                                word.substring(1).toLowerCase();
            }
            builder.append(word);
        }

        return builder.toString();
    }
}