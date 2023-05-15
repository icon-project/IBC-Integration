package foundation.icon.btp.xcall;

import score.Address;
import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class NetworkAddress {
    private static final String DELIM_NET="/";
    String net;
    String account;
    public NetworkAddress(String net, String account) {
        this.net = net;
        this.account = account;
    }

    public NetworkAddress(String net, Address account) {
        this.net = net;
        this.account = account.toString();
    }

    public String net() {
        return net;
    }

    public String account() {
        return account;
    }

    @Override
    public String toString() {
        return net + DELIM_NET + account;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;

        NetworkAddress that = (NetworkAddress) o;
        return toString().equals(that.toString());
    }

    @Override
    public int hashCode() {
        return toString().hashCode();
    }

    public boolean isValid() {
        return  (!(net == null || net.isEmpty())) &&
                (!(account == null || account.isEmpty()));
    }

    public static NetworkAddress parse(String str) {
        if (str == null) {
            return null;
        }
        String net = "";
        String contract = "";
        int netIdx = str.indexOf(DELIM_NET);
        if (netIdx >= 0) {
            net = str.substring(0, netIdx);
            contract = str.substring(netIdx + DELIM_NET.length());
        } else {
            contract = str;
        }

        return new NetworkAddress( net, contract);
    }

    public static NetworkAddress valueOf(String str) {
        NetworkAddress address = parse(str);
        if (address == null || !address.isValid()) {
           Context.revert("failed to parse NetworkAddress");
        }
        return address;
    }
    // Accepts Native addresses as well
    public static NetworkAddress valueOf(String str, String nativeNid) {
        NetworkAddress address = parse(str);
        Context.require(address != null, "failed to parse NetworkAddress");
        if (address.net.isEmpty()) {
            Address.fromString(address.account);
            address.net = nativeNid;
        }
        return address;
    }

    public static void writeObject(ObjectWriter writer, NetworkAddress obj) {
        obj.writeObject(writer);
    }

    public void writeObject(ObjectWriter writer) {
        writer.write(this.toString());
    }

    public static NetworkAddress readObject(ObjectReader reader) {
        return NetworkAddress.parse(reader.readString());
    }

    public static NetworkAddress fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return NetworkAddress.readObject(reader);
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        NetworkAddress.writeObject(writer, this);
        return writer.toByteArray();
    }
}

