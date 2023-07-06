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

import java.util.List;

import score.Address;
import score.ObjectReader;
import score.ObjectWriter;
import scorex.util.ArrayList;

public class CallRequest {
    private final Address from;
    private final String to;
    private final String[] protocols;
    private final byte[] rollback;
    private boolean enabled;

    public CallRequest(Address from, String to, String[] protocols, byte[] rollback) {
        this.from = from;
        this.to = to;
        if (protocols == null) {
            protocols = new String[]{};
        }
        this.protocols = protocols;
        this.rollback = rollback;
        this.enabled = false;
    }

    public Address getFrom() {
        return from;
    }

    public String getTo() {
        return to;
    }

    public String[] getProtocols() {
        return protocols;
    }

    public byte[] getRollback() {
        return rollback;
    }

    public static void writeObject(ObjectWriter w, CallRequest req) {
        w.beginList(5);
        w.write(req.from);
        w.write(req.to);
        w.beginList(req.protocols.length);
        for(String protocol : req.protocols) {
            w.write(protocol);
        }
        w.end();
        w.writeNullable(req.rollback);
        w.write(req.enabled);
        w.end();
    }

    public static CallRequest readObject(ObjectReader r) {
        r.beginList();
        CallRequest req = new CallRequest(
                r.readAddress(),
                r.readString(),
                readProtocols(r),
                r.readNullable(byte[].class)
        );
        if (r.readBoolean()) {
            req.setEnabled();
        }
        r.end();
        return req;
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

    public boolean enabled() {
        return enabled;
    }

    public void setEnabled() {
        this.enabled = true;
    }
}
