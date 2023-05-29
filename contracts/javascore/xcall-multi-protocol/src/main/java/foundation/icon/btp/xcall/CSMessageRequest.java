/*
 * Copyright 2022 ICON Foundation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package foundation.icon.btp.xcall;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

import java.math.BigInteger;
import java.util.List;

public class CSMessageRequest {
    private final String from;
    private final String to;
    private final String[] protocols;
    private final BigInteger sn;
    private final boolean rollback;
    private final byte[] data;

    public CSMessageRequest(String from, String to, String[] protocols, BigInteger sn, boolean rollback, byte[] data) {
        this.from = from;
        this.to = to;
        this.protocols = protocols;
        this.sn = sn;
        this.rollback = rollback;
        this.data = data;
    }


    public String getFrom() {
        return from;
    }

    public String getTo() {
        return to;
    }

    public String[] getProtocols() {
        return protocols;
    }

    public BigInteger getSn() {
        return sn;
    }

    public boolean needRollback() {
        return rollback;
    }

    public byte[] getData() {
        return data;
    }

    public static void writeObject(ObjectWriter w, CSMessageRequest m) {
        w.beginList(6);
        w.write(m.from);
        w.write(m.to);
        w.beginList(m.protocols.length);
        for(String protocol : m.protocols) {
            w.write(protocol);
        }
        w.end();
        w.write(m.sn);
        w.write(m.rollback);
        w.writeNullable(m.data);
        w.end();
    }

    public static CSMessageRequest readObject(ObjectReader r) {
        r.beginList();
        CSMessageRequest m = new CSMessageRequest(
                r.readString(),
                r.readString(),
                readProtocols(r),
                r.readBigInteger(),
                r.readBoolean(),
                r.readNullable(byte[].class)
        );
        r.end();
        return m;
    }

    private static String[] readProtocols(ObjectReader r) {
        r.beginList();
        List<String> protocolsList = new ArrayList<>();
        while(r.hasNext()) {
            protocolsList.add(r.readString());
        }
        int size = protocolsList.size();
        String[] protocols = new String[size];
        for(int i=0; i < size; i++) {
            protocols[i] = protocolsList.get(i);
        }
        r.end();
        return protocols;
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        CSMessageRequest.writeObject(writer, this);
        return writer.toByteArray();
    }

    public static CSMessageRequest fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return readObject(reader);
    }
}
