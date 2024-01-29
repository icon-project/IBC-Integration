// SPDX-License-Identifier: MIT
pragma solidity >=0.8.18;
import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {Vm} from "forge-std/Vm.sol";

import "@xcall/contracts/xcall/CallService.sol";
import "@xcall/contracts/mocks/multi-protocol-dapp/MultiProtocolSampleDapp.sol";
import "@xcall/contracts/adapters/WormholeAdapter.sol";
import "@xcall/contracts/adapters/LayerZeroAdapter.sol";
import "@xcall/contracts/adapters/CentralizedConnection.sol";

contract WormholeTest is Script {

    using Strings for string;

    uint256 internal deployerPrivateKey;

    CallService xCallSource;
    CallService xCallTarget;

    WormholeAdapter adapterSource;
    WormholeAdapter adapterTarget;

    MultiProtocolSampleDapp dappSource;
    MultiProtocolSampleDapp dappTarget;

    string public nidSource;
    string public nidTarget;

    address public relayerSource;
    address public relayerTarget;

    string public chain1;
    string public chain2;

    uint16 public sourceChain;
    uint16 public targetChain;

    constructor() {
        deployerPrivateKey = vm.envUint("PRIVATE_KEY");
    }

    modifier broadcast(uint256 privateKey) {
        vm.startBroadcast(privateKey);

        _;

        vm.stopBroadcast();
    }

    function capitalizeString(
        string memory input
    ) public pure returns (string memory) {
        bytes memory inputBytes = bytes(input);
        for (uint i = 0; i < inputBytes.length; i++) {
            if (uint8(inputBytes[i]) >= 97 && uint8(inputBytes[i]) <= 122) {
                inputBytes[i] = bytes1(uint8(inputBytes[i]) - 32);
            }
        }
        return string(inputBytes);
    }

    function sendMessage(
        string memory chain1,
        string memory chain2,
        uint256 fee
    ) public broadcast(deployerPrivateKey) {

        address chain1_dapp = vm.envAddress(
            capitalizeString(chain1).concat("_MOCK_DAPP")
        );
        address chain2_dapp = vm.envAddress(
            capitalizeString(chain2).concat("_MOCK_DAPP")
        );

        MultiProtocolSampleDapp dapp1 = MultiProtocolSampleDapp(chain1_dapp);
        MultiProtocolSampleDapp dapp2 = MultiProtocolSampleDapp(chain2_dapp);

        string memory nid_chain1 = vm.envString(
            capitalizeString(chain1).concat("_NID")
        );
        string memory nid_chain2 = vm.envString(
            capitalizeString(chain2).concat("_NID")
        );

        string memory to = NetworkAddress.networkAddress(
            nid_chain2,
            ParseAddress.toString(chain2_dapp)
        );

        dapp1.sendMessage{value: fee}(
            to,
            bytes("Hi I am Ranju from Pokhara"),
            bytes("")
        );
    }

    function executeCall(
        uint256 req_id,
        bytes memory data,
        string memory chain
    ) public broadcast(deployerPrivateKey) {

        address chain2_xcall = vm.envAddress(
            capitalizeString(chain).concat("_XCALL")
        );

        xCallTarget = CallService(chain2_xcall);

        xCallTarget.executeCall(req_id, data);
    }
}
