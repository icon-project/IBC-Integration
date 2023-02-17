package ibc.icon.structs.proto.core.connection;

public class Version {
    String identifier;
    String[] features;

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
