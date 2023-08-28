/*
 * Copyright (c) 2022-2022 Balanced.network.
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

package ibc.icon.integration;

import foundation.icon.score.client.Wallet;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.math.BigInteger;
import java.nio.file.Path;
import java.util.Properties;

public class Env {
    private static Chain chain;

    static {
        String envFile = System.getProperty("env.props", "conf/env.props");
        Properties props = new Properties();
        try {
            FileInputStream fis = new FileInputStream(envFile);
            props.load(fis);
            fis.close();
        } catch (IOException e) {
            System.err.printf("'%s' does not exist\n", envFile);
            throw new IllegalArgumentException(e.getMessage());
        }
        String confPath = Path.of(envFile).getParent().toString() + "/";
        readProperties(props, confPath);
    }

    private static void readProperties(Properties props, String confPath) {
        String chainName = "chain";
        String nid = props.getProperty(chainName + ".nid");
        if (nid == null) {
            throw new IllegalArgumentException("nid not found");
        }
        String godWalletPath = confPath + props.getProperty(chainName + ".godWallet");
        String godPassword = props.getProperty(chainName + ".godPassword");
        Wallet godWallet;
        try {
            godWallet = readWalletFromFile(godWalletPath, godPassword);
        } catch (IOException e) {
            throw new IllegalArgumentException(e.getMessage());
        }
        String nodeName = "node";
        String url = props.getProperty(nodeName + ".url");
        if (url == null) {
            throw new IllegalArgumentException("node url not found");
        }

        String apiVersion = props.getProperty(nodeName + ".apiVersion");
        if (apiVersion == null) {
            throw new IllegalArgumentException("apiVersion not found");
        }
        chain = new Chain(BigInteger.valueOf(Integer.parseInt(nid.substring(2), 16)), godWallet, url, apiVersion);
    }

    private static Wallet readWalletFromFile(String path, String password) throws IOException {
        File file = new File(path);
        return Wallet.load(password, file);
    }

    public static Chain getDefaultChain() {
        if (chain == null) {
            throw new AssertionError("Chain not found");
        }
        return chain;
    }

    public static class Chain {
        public final BigInteger networkId;
        public final Wallet godWallet;
        private final String nodeUrl;
        private final String apiVersion;

        public Chain(BigInteger networkId, Wallet godWallet, String url, String apiVersion) {
            this.networkId = networkId;
            this.godWallet = godWallet;
            this.nodeUrl = url;
            this.apiVersion = apiVersion;
        }

        public String getEndpointURL() {
            return this.nodeUrl + "/api/v" + apiVersion;
        }
    }
}
