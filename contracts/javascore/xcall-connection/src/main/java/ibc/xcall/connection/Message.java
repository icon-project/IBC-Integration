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

package  ibc.xcall.connection;

import java.math.BigInteger;

import score.ByteArrayObjectWriter;
import score.Context;
import score.ObjectReader;
import score.ObjectWriter;

public class Message {
    private final BigInteger sn;
    private final byte[] data;

    public Message(BigInteger sn, byte[] data) {
        this.sn = sn;
        this.data = data;
    }

    public BigInteger getSn() {
        return sn;
    }

    public byte[] getData() {
        return data;
    }

    public static void writeObject(ObjectWriter w, Message m) {
        w.beginList(2);
        w.write(m.sn);
        w.writeNullable(m.data);
        w.end();
    }

    public static Message readObject(ObjectReader r) {
        r.beginList();
        Message m = new Message(
                r.readBigInteger(),
                r.readNullable(byte[].class)
        );
        r.end();
        return m;
    }

    public byte[] toBytes() {
        ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
        Message.writeObject(writer, this);
        return writer.toByteArray();
    }

    public static Message fromBytes(byte[] bytes) {
        ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
        return readObject(reader);
    }
}