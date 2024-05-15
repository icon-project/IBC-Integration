package ibc.ics23.commitment;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import cosmos.ics23.v1.*;

import java.io.IOException;
import java.io.InputStream;
import java.util.Map;

public class LoadOpsTestData {

    public static class LeafOpTestData {
        public LeafOp op;
        public byte[] key;
        public byte[] value;
        public boolean isErr;
        public byte[] expected;
    }

    public static Map<String, LeafOpTestData> loadLeafOpTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestLeafOpData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class InnerOpTestData {
        public InnerOp op;
        public byte[] child;
        public boolean isErr;
        public byte[] expected;
    }

    public static Map<String, InnerOpTestData> loadInnerOpTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestInnerOpData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }

    public static class DoHashTestData {
        public int hashOp;
        public String preimage;
        public String expectedHash;
    }

    public static Map<String, DoHashTestData> loadDoHashTestData() throws IOException {
        // Get the input stream using the ClassLoader
        InputStream inputStream = LoadOpsTestData.class.getResourceAsStream("/TestDoHashData.json");

        // Deserialize JSON using Jackson ObjectMapper
        ObjectMapper objectMapper = new ObjectMapper();

        return objectMapper.readValue(inputStream, new TypeReference<>() {
        });
    }
}
