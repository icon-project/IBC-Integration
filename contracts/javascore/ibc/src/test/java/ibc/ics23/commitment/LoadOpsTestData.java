package ibc.ics23.commitment;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import icon.proto.core.commitment.InnerOp;
import icon.proto.core.commitment.LeafOp;

import java.io.IOException;
import java.io.InputStream;
import java.util.Map;

public class LoadOpsTestData {

    public static class LeafOpTestStruct {
        public LeafOp op;
        public byte[] key;
        public byte[] value;
        public boolean isErr;
        public byte[] expected;
    }

    public static Map<String, LeafOpTestStruct> loadLeafOpTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestLeafOpData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class InnerOpTestStruct {
        public InnerOp op;
        public byte[] child;
        public boolean isErr;
        public byte[] expected;
    }

    public static Map<String, InnerOpTestStruct> loadInnerOpTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestInnerOpData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class DoHashTestStruct {
        public int hashOp;
        public String preimage;
        public String expectedHash;
    }

    public static Map<String, DoHashTestStruct> loadDoHashTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestDoHashData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }
}
