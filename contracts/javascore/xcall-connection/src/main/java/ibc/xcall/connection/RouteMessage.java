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

 /*
 pub cc_id: CrossChainId,
    pub source_address: Address,
    pub destination_chain: ChainName,
    pub destination_address: Address,
    /// for better user experience, the payload hash gets encoded into hex at the edges (input/output),
    /// but internally, we treat it as raw bytes to enforce its format.
    #[serde(with = "axelar_wasm_std::hex")]
    #[schemars(with = "String")] // necessary attribute in conjunction with #[serde(with ...)]
    pub payload_hash: [u8; 32],
 */
 
 public class RouteMessage {
     private final String cc_id;
     private final String source_address;
     private final String destination_chain;
     private final String destination_address;
     private final byte[] payload_hash;
     private static final String KECCAK256 = "keccak-256";
 
     public RouteMessage(String cc_id, String source_address,String destination_chain,String destination_address, byte[] payload_hash) {
         this.cc_id = cc_id;
         this.source_address = source_address;
         this.destination_chain = destination_chain;
         this.destination_address=destination_address;
         this.payload_hash=payload_hash;
     }
 
     public String getCCID() {
         return cc_id;
     }
 
     public String  getSourceAddress() {
         return source_address;
     }

     public String  getDestinationChain() {
        return destination_chain;
    }

    public String  getDestinationAddress() {
        return destination_address;
    }
 
     public byte[] getPayloadHash() {
         return payload_hash;
     }
 
     public static void writeObject(ObjectWriter w, RouteMessage m) {
         w.beginList(5);
         w.write(m.cc_id);
         w.write(m.source_address);
         w.write(m.destination_chain);
         w.write(m.destination_address);
         w.writeNullable(m.payload_hash);
         w.end();
     }
 
     public static RouteMessage readObject(ObjectReader r) {
         r.beginList();
         RouteMessage m = new RouteMessage(
             r.read(String.class),
             r.read(String.class),
             r.read(String.class),
             r.read(String.class),
             r.readNullable(byte[].class)
         );
         r.end();
         return m;
     }
 
     public byte[] toBytes() {
         ByteArrayObjectWriter writer = Context.newByteArrayObjectWriter("RLPn");
         RouteMessage.writeObject(writer, this);
         return writer.toByteArray();
     }
 
     public static RouteMessage fromBytes(byte[] bytes) {
         ObjectReader reader = Context.newByteArrayObjectReader("RLPn", bytes);
         return readObject(reader);
     }

     public byte[] getHash(){
        return Context.hash(KECCAK256,this.toBytes());

     }

     public byte[] getCommandId(){
        return Context.hash(KECCAK256,this.cc_id);
     }
 }