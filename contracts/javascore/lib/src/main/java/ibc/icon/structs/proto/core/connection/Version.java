package ibc.icon.structs.proto.core.connection;

import java.util.Arrays;
import java.util.List;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

public class Version {
    public String identifier;
    public String[] features;

    public static void writeObject(ObjectWriter writer, Version obj) {
        obj.writeObject(writer);
    }

    public static Version readObject(ObjectReader reader) {
        Version obj = new Version();
        reader.beginList();
        obj.identifier = reader.readString();
        reader.beginList();
        String[] features = null;
        List<String> featuresList = new ArrayList<>();
        while (reader.hasNext()) {
            byte[] featureElementBytes = reader.readNullable(byte[].class);
            if (featureElementBytes != null) {
                ObjectReader featureElementReader = Context.newByteArrayObjectReader("RLPn", featureElementBytes);
                featuresList.add(featureElementReader.read(String.class));
            }
        }

        features = new String[featuresList.size()];
        for (int i = 0; i < featuresList.size(); i++) {
            features[i] = (String) featuresList.get(i);
        }
        obj.features = features;
        reader.end();
        reader.end();

        return obj;
    }

    public void writeObject(ObjectWriter writer) {
        writer.beginList(2);
        writer.write(this.identifier);
        String[] features = this.getFeatures();
        if (features != null) {
            writer.beginNullableList(features.length);
            for (String v : features) {
                ByteArrayObjectWriter vWriter = Context.newByteArrayObjectWriter("RLPn");
                vWriter.write(v);
                writer.write(vWriter.toByteArray());
            }
            writer.end();
        } else {
            writer.writeNull();
        }
        writer.end();
    }

    public boolean equals(Version v) {
        return this.identifier == v.identifier && Arrays.equals(this.features, v.features);
    }

    public String getIdentifier() {
        return identifier;
    }

    public void setIdentifier(String identifier) {
        this.identifier = identifier;
    }

    public String[] getFeatures() {
        return features;
    }

    public void setFeatures(String[] features) {
        this.features = features;
    }

}
