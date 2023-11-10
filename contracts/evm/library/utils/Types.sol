// SPDX-License-Identifier: Apache-2.0
pragma solidity >=0.8.0;

/**
 * @notice List of ALL Struct being used to Encode and Decode RLP Messages
 */
library Types {
    // The name of CallService.
    string constant NAME = "xcallM";

    int constant CS_REQUEST = 1;
    int constant CS_RESPONSE = 2;

    struct CallRequest {
        address from;
        string to;
        string[] sources;
        bytes rollback;
        bool enabled; //whether wait response or received
    }

    struct CSMessage {
        int msgType;
        bytes payload;
    }

    struct CSMessageRequest {
        string from;
        string to;
        uint256 sn;
        bool rollback;
        bytes data;
        string[] protocols;

    }

    struct ProxyRequest {
        string from;
        string to;
        uint256 sn;
        bool rollback;
        bytes32 hash;
        string[] protocols;
    }

    int constant CS_RESP_SUCCESS = 1;
    int constant CS_RESP_FAILURE = 0;

    struct CSMessageResponse {
        uint256 sn;
        int code;
    }

    struct PendingResponse {
        bytes msg;
        string targetNetwork;
    }

}
